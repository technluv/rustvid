//! Video encoder implementation using FFmpeg

use crate::{ExportError, Result, ExportSettings, Quality};
use ffmpeg_next as ffmpeg;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info, warn, error};
use bytes::Bytes;

/// Supported video codecs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodec {
    H264,
    H265,
    VP9,
    AV1,
}

impl VideoCodec {
    pub fn from_string(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "h264" | "libx264" => Ok(VideoCodec::H264),
            "h265" | "libx265" | "hevc" => Ok(VideoCodec::H265),
            "vp9" | "libvpx-vp9" => Ok(VideoCodec::VP9),
            "av1" | "libaom-av1" => Ok(VideoCodec::AV1),
            _ => Err(ExportError::UnsupportedCodec(s.to_string())),
        }
    }
    
    pub fn to_ffmpeg_codec(&self) -> &'static str {
        match self {
            VideoCodec::H264 => "libx264",
            VideoCodec::H265 => "libx265",
            VideoCodec::VP9 => "libvpx-vp9",
            VideoCodec::AV1 => "libaom-av1",
        }
    }
    
    pub fn supports_hardware_encoding(&self) -> bool {
        matches!(self, VideoCodec::H264 | VideoCodec::H265)
    }
}

/// Hardware encoder type
#[derive(Debug, Clone, Copy)]
pub enum HardwareEncoder {
    None,
    NVENC,      // NVIDIA
    QuickSync,  // Intel
    AMF,        // AMD
    VideoToolbox, // macOS
}

impl HardwareEncoder {
    pub fn detect() -> Self {
        // Try to detect available hardware encoders
        if cfg!(target_os = "macos") {
            return HardwareEncoder::VideoToolbox;
        }
        
        // Check for NVIDIA GPU
        if let Ok(output) = std::process::Command::new("nvidia-smi").output() {
            if output.status.success() {
                return HardwareEncoder::NVENC;
            }
        }
        
        // TODO: Add detection for Intel QuickSync and AMD AMF
        HardwareEncoder::None
    }
    
    pub fn get_codec_name(&self, codec: VideoCodec) -> Option<&'static str> {
        match (self, codec) {
            (HardwareEncoder::NVENC, VideoCodec::H264) => Some("h264_nvenc"),
            (HardwareEncoder::NVENC, VideoCodec::H265) => Some("hevc_nvenc"),
            (HardwareEncoder::QuickSync, VideoCodec::H264) => Some("h264_qsv"),
            (HardwareEncoder::QuickSync, VideoCodec::H265) => Some("hevc_qsv"),
            (HardwareEncoder::AMF, VideoCodec::H264) => Some("h264_amf"),
            (HardwareEncoder::AMF, VideoCodec::H265) => Some("hevc_amf"),
            (HardwareEncoder::VideoToolbox, VideoCodec::H264) => Some("h264_videotoolbox"),
            (HardwareEncoder::VideoToolbox, VideoCodec::H265) => Some("hevc_videotoolbox"),
            _ => None,
        }
    }
}

/// Video encoder configuration
#[derive(Debug, Clone)]
pub struct EncoderConfig {
    pub codec: VideoCodec,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub bitrate: u32,
    pub quality: Quality,
    pub hardware_encoder: HardwareEncoder,
    pub keyframe_interval: u32,
    pub b_frames: u32,
}

impl From<&ExportSettings> for EncoderConfig {
    fn from(settings: &ExportSettings) -> Self {
        let codec = VideoCodec::from_string(&settings.video_codec)
            .unwrap_or(VideoCodec::H264);
        
        let hardware_encoder = HardwareEncoder::detect();
        
        // Calculate keyframe interval (typically 2-10 seconds)
        let keyframe_interval = (settings.fps * 2.0) as u32;
        
        Self {
            codec,
            width: settings.width,
            height: settings.height,
            fps: settings.fps,
            bitrate: settings.bitrate,
            quality: settings.quality.clone(),
            hardware_encoder,
            keyframe_interval,
            b_frames: 2,
        }
    }
}

/// Video encoder using FFmpeg
pub struct VideoEncoder {
    config: EncoderConfig,
    output_path: String,
    format_context: Option<ffmpeg::format::context::Output>,
    video_stream_index: Option<usize>,
    video_encoder: Option<ffmpeg::encoder::Video>,
    frame_count: u64,
}

impl VideoEncoder {
    pub fn new(config: EncoderConfig, output_path: impl AsRef<Path>) -> Result<Self> {
        // Initialize FFmpeg
        ffmpeg::init().map_err(|e| ExportError::ExportFailed(format!("FFmpeg init failed: {}", e)))?;
        
        Ok(Self {
            config,
            output_path: output_path.as_ref().to_string_lossy().to_string(),
            format_context: None,
            video_stream_index: None,
            video_encoder: None,
            frame_count: 0,
        })
    }
    
    /// Initialize the encoder and output format
    pub fn initialize(&mut self) -> Result<()> {
        info!("Initializing video encoder: {:?}", self.config.codec);
        
        // Create output format context
        let mut format_context = ffmpeg::format::output(&self.output_path)
            .map_err(|e| ExportError::ExportFailed(format!("Failed to create output context: {}", e)))?;
        
        // Find the appropriate codec
        let codec_name = if let Some(hw_codec) = self.config.hardware_encoder.get_codec_name(self.config.codec) {
            info!("Using hardware encoder: {}", hw_codec);
            hw_codec
        } else {
            info!("Using software encoder: {}", self.config.codec.to_ffmpeg_codec());
            self.config.codec.to_ffmpeg_codec()
        };
        
        let codec = ffmpeg::encoder::find_by_name(codec_name)
            .or_else(|| {
                warn!("Hardware encoder not available, falling back to software");
                ffmpeg::encoder::find_by_name(self.config.codec.to_ffmpeg_codec())
            })
            .ok_or_else(|| ExportError::UnsupportedCodec(codec_name.to_string()))?;
        
        // Add video stream
        let mut stream = format_context.add_stream(codec)
            .map_err(|e| ExportError::ExportFailed(format!("Failed to add stream: {}", e)))?;
        
        let video_stream_index = stream.index();
        
        // Configure video encoder
        let codec_context = stream.codec();
        let mut video_encoder = codec_context.encoder().video()
            .map_err(|e| ExportError::ExportFailed(format!("Failed to get video encoder: {}", e)))?;
        
        // Set encoder parameters
        video_encoder.set_width(self.config.width);
        video_encoder.set_height(self.config.height);
        video_encoder.set_time_base(ffmpeg::Rational::new(1, self.config.fps as i32));
        video_encoder.set_frame_rate(Some(ffmpeg::Rational::new(self.config.fps as i32, 1)));
        video_encoder.set_bit_rate(self.config.bitrate as usize);
        video_encoder.set_max_b_frames(self.config.b_frames as usize);
        video_encoder.set_gop(self.config.keyframe_interval as usize);
        
        // Set pixel format (YUV420P is most compatible)
        video_encoder.set_format(ffmpeg::format::Pixel::YUV420P);
        
        // Set quality-based parameters
        match &self.config.quality {
            Quality::Low => {
                video_encoder.set_qmin(25);
                video_encoder.set_qmax(35);
            },
            Quality::Medium => {
                video_encoder.set_qmin(20);
                video_encoder.set_qmax(30);
            },
            Quality::High => {
                video_encoder.set_qmin(15);
                video_encoder.set_qmax(25);
            },
            Quality::Ultra => {
                video_encoder.set_qmin(10);
                video_encoder.set_qmax(20);
            },
            Quality::Custom(q) => {
                let q = *q as usize;
                video_encoder.set_qmin(q.saturating_sub(5));
                video_encoder.set_qmax(q.saturating_add(5));
            },
        }
        
        // Codec-specific settings
        match self.config.codec {
            VideoCodec::H264 => {
                // Set H.264 profile and preset
                let mut options = ffmpeg::Dictionary::new();
                options.set("preset", "medium");
                options.set("profile", "high");
                options.set("level", "4.1");
                video_encoder.open_with(options)
                    .map_err(|e| ExportError::ExportFailed(format!("Failed to open H.264 encoder: {}", e)))?;
            },
            VideoCodec::H265 => {
                let mut options = ffmpeg::Dictionary::new();
                options.set("preset", "medium");
                options.set("profile", "main");
                video_encoder.open_with(options)
                    .map_err(|e| ExportError::ExportFailed(format!("Failed to open H.265 encoder: {}", e)))?;
            },
            VideoCodec::VP9 => {
                let mut options = ffmpeg::Dictionary::new();
                options.set("deadline", "good");
                options.set("cpu-used", "2");
                video_encoder.open_with(options)
                    .map_err(|e| ExportError::ExportFailed(format!("Failed to open VP9 encoder: {}", e)))?;
            },
            VideoCodec::AV1 => {
                let mut options = ffmpeg::Dictionary::new();
                options.set("cpu-used", "4");
                options.set("tiles", "2x2");
                video_encoder.open_with(options)
                    .map_err(|e| ExportError::ExportFailed(format!("Failed to open AV1 encoder: {}", e)))?;
            },
        }
        
        stream.set_parameters(&video_encoder);
        
        // Write format header
        format_context.write_header()
            .map_err(|e| ExportError::ExportFailed(format!("Failed to write header: {}", e)))?;
        
        self.format_context = Some(format_context);
        self.video_stream_index = Some(video_stream_index);
        self.video_encoder = Some(video_encoder);
        
        info!("Video encoder initialized successfully");
        Ok(())
    }
    
    /// Encode a single frame
    pub fn encode_frame(&mut self, frame_data: &[u8]) -> Result<()> {
        let encoder = self.video_encoder.as_mut()
            .ok_or_else(|| ExportError::ExportFailed("Encoder not initialized".to_string()))?;
        
        let format_context = self.format_context.as_mut()
            .ok_or_else(|| ExportError::ExportFailed("Format context not initialized".to_string()))?;
        
        let stream_index = self.video_stream_index
            .ok_or_else(|| ExportError::ExportFailed("Stream index not set".to_string()))?;
        
        // Create video frame
        let mut frame = ffmpeg::frame::Video::new(
            ffmpeg::format::Pixel::YUV420P,
            self.config.width,
            self.config.height,
        );
        
        // Copy frame data
        // Note: This assumes the input is already in YUV420P format
        // In a real implementation, you'd need to convert from RGB/RGBA
        frame.data_mut(0).copy_from_slice(&frame_data[..]);
        
        // Set frame timestamp
        frame.set_pts(Some(self.frame_count as i64));
        self.frame_count += 1;
        
        // Send frame to encoder
        encoder.send_frame(&frame)
            .map_err(|e| ExportError::ExportFailed(format!("Failed to send frame: {}", e)))?;
        
        // Receive and write packets
        let mut packet = ffmpeg::Packet::empty();
        while encoder.receive_packet(&mut packet).is_ok() {
            packet.set_stream(stream_index);
            packet.write_interleaved(format_context)
                .map_err(|e| ExportError::ExportFailed(format!("Failed to write packet: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Flush the encoder and finalize the output file
    pub fn finalize(&mut self) -> Result<()> {
        info!("Finalizing video encoding");
        
        if let Some(encoder) = &mut self.video_encoder {
            // Flush encoder
            encoder.send_eof()
                .map_err(|e| ExportError::ExportFailed(format!("Failed to send EOF: {}", e)))?;
            
            let format_context = self.format_context.as_mut()
                .ok_or_else(|| ExportError::ExportFailed("Format context not initialized".to_string()))?;
            
            let stream_index = self.video_stream_index
                .ok_or_else(|| ExportError::ExportFailed("Stream index not set".to_string()))?;
            
            // Receive remaining packets
            let mut packet = ffmpeg::Packet::empty();
            while encoder.receive_packet(&mut packet).is_ok() {
                packet.set_stream(stream_index);
                packet.write_interleaved(format_context)
                    .map_err(|e| ExportError::ExportFailed(format!("Failed to write packet: {}", e)))?;
            }
        }
        
        // Write trailer
        if let Some(mut format_context) = self.format_context.take() {
            format_context.write_trailer()
                .map_err(|e| ExportError::ExportFailed(format!("Failed to write trailer: {}", e)))?;
        }
        
        info!("Video encoding completed. Total frames: {}", self.frame_count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_video_codec_from_string() {
        assert_eq!(VideoCodec::from_string("h264").unwrap(), VideoCodec::H264);
        assert_eq!(VideoCodec::from_string("H265").unwrap(), VideoCodec::H265);
        assert_eq!(VideoCodec::from_string("vp9").unwrap(), VideoCodec::VP9);
        assert_eq!(VideoCodec::from_string("av1").unwrap(), VideoCodec::AV1);
        assert!(VideoCodec::from_string("invalid").is_err());
    }
    
    #[test]
    fn test_hardware_encoder_detection() {
        let hw_encoder = HardwareEncoder::detect();
        // This will vary by system, so we just check it doesn't panic
        println!("Detected hardware encoder: {:?}", hw_encoder);
    }
    
    #[test]
    fn test_encoder_config_from_export_settings() {
        let settings = ExportSettings::default();
        let config = EncoderConfig::from(&settings);
        
        assert_eq!(config.codec, VideoCodec::H264);
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.fps, 30.0);
        assert_eq!(config.bitrate, 8000000);
    }
}