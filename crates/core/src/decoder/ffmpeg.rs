//! FFmpeg-based video decoder implementation

use crate::{
    error::{Result, VideoError},
    frame::Frame,
    traits::{
        MediaInfo, Packet, PixelFormat, StreamInfo, StreamType, VideoDecoder, VideoDemuxer,
        VideoStreamInfo,
    },
    VideoFormat,
};
use ffmpeg_next as ffmpeg;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// FFmpeg-based video decoder
pub struct FFmpegDecoder {
    /// Input context
    input_context: Option<ffmpeg::format::context::Input>,
    
    /// Video stream index
    video_stream_index: Option<usize>,
    
    /// Video decoder
    decoder: Option<ffmpeg::codec::decoder::Video>,
    
    /// Current position in the stream
    current_position: Duration,
    
    /// Video format information
    format: Option<VideoFormat>,
    
    /// Frame converter for pixel format conversion
    scaler: Option<ffmpeg::software::scaling::Context>,
    
    /// Target pixel format
    target_pixel_format: PixelFormat,
    
    /// Error recovery state
    error_recovery_attempts: u32,
}

impl FFmpegDecoder {
    /// Creates a new FFmpeg decoder
    pub fn new() -> Self {
        Self {
            input_context: None,
            video_stream_index: None,
            decoder: None,
            current_position: Duration::ZERO,
            format: None,
            scaler: None,
            target_pixel_format: PixelFormat::RGB24,
            error_recovery_attempts: 0,
        }
    }
    
    /// Sets the target pixel format for decoded frames
    pub fn set_target_pixel_format(&mut self, format: PixelFormat) {
        self.target_pixel_format = format;
    }
    
    /// Converts FFmpeg pixel format to our PixelFormat enum
    fn convert_pixel_format(format: ffmpeg::format::Pixel) -> PixelFormat {
        match format {
            ffmpeg::format::Pixel::RGB24 => PixelFormat::RGB24,
            ffmpeg::format::Pixel::RGBA => PixelFormat::RGBA,
            ffmpeg::format::Pixel::BGR24 => PixelFormat::BGR24,
            ffmpeg::format::Pixel::BGRA => PixelFormat::BGRA,
            ffmpeg::format::Pixel::YUV420P => PixelFormat::YUV420P,
            ffmpeg::format::Pixel::YUV422P => PixelFormat::YUV422P,
            ffmpeg::format::Pixel::YUV444P => PixelFormat::YUV444P,
            ffmpeg::format::Pixel::NV12 => PixelFormat::NV12,
            ffmpeg::format::Pixel::NV21 => PixelFormat::NV21,
            ffmpeg::format::Pixel::GRAY8 => PixelFormat::Gray8,
            _ => PixelFormat::YUV420P, // Default fallback
        }
    }
    
    /// Converts our PixelFormat to FFmpeg pixel format
    fn to_ffmpeg_pixel_format(format: PixelFormat) -> ffmpeg::format::Pixel {
        match format {
            PixelFormat::RGB24 => ffmpeg::format::Pixel::RGB24,
            PixelFormat::RGBA => ffmpeg::format::Pixel::RGBA,
            PixelFormat::BGR24 => ffmpeg::format::Pixel::BGR24,
            PixelFormat::BGRA => ffmpeg::format::Pixel::BGRA,
            PixelFormat::YUV420P => ffmpeg::format::Pixel::YUV420P,
            PixelFormat::YUV422P => ffmpeg::format::Pixel::YUV422P,
            PixelFormat::YUV444P => ffmpeg::format::Pixel::YUV444P,
            PixelFormat::NV12 => ffmpeg::format::Pixel::NV12,
            PixelFormat::NV21 => ffmpeg::format::Pixel::NV21,
            PixelFormat::Gray8 => ffmpeg::format::Pixel::GRAY8,
        }
    }
    
    /// Initializes FFmpeg if not already initialized
    fn ensure_ffmpeg_initialized() {
        // FFmpeg initialization is thread-safe and idempotent
        ffmpeg::init().unwrap_or(());
    }
    
    /// Recovers from decoding errors
    fn recover_from_error(&mut self) -> Result<()> {
        self.error_recovery_attempts += 1;
        
        if self.error_recovery_attempts > 3 {
            return Err(VideoError::DecoderError(
                "Maximum error recovery attempts exceeded".to_string(),
            ));
        }
        
        // Flush the decoder
        if let Some(decoder) = &mut self.decoder {
            decoder.flush();
        }
        
        Ok(())
    }
}

impl VideoDecoder for FFmpegDecoder {
    fn open(&mut self, path: &str) -> Result<VideoFormat> {
        Self::ensure_ffmpeg_initialized();
        
        // Open input file
        let input = ffmpeg::format::input(&path)
            .map_err(|e| VideoError::FFmpegError(format!("Failed to open input: {}", e)))?;
        
        // Find video stream
        let video_stream = input
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or_else(|| VideoError::FormatError("No video stream found".to_string()))?;
        
        let video_stream_index = video_stream.index();
        
        // Get decoder parameters
        let context_decoder = ffmpeg::codec::context::Context::from_parameters(video_stream.parameters())
            .map_err(|e| VideoError::FFmpegError(format!("Failed to create decoder context: {}", e)))?;
        
        let mut decoder = context_decoder.decoder().video()
            .map_err(|e| VideoError::FFmpegError(format!("Failed to create video decoder: {}", e)))?;
        
        // Set decoder options for better error resilience
        decoder.set_threading(ffmpeg::threading::Config {
            kind: ffmpeg::threading::Type::Frame,
            count: 0, // Auto-detect
        });
        
        // Extract format information
        let width = decoder.width();
        let height = decoder.height();
        let frame_rate = video_stream.avg_frame_rate();
        let fps = frame_rate.0 as f32 / frame_rate.1.max(1) as f32;
        let duration_ms = input.duration().map(|d| (d * 1000 / ffmpeg::ffi::AV_TIME_BASE) as u64);
        let bit_rate = input.bit_rate().map(|b| b as u64);
        
        let format = VideoFormat {
            width,
            height,
            fps,
            codec: decoder.codec().map(|c| c.name().to_string()).unwrap_or_else(|| "unknown".to_string()),
            duration_ms,
            bit_rate,
        };
        
        // Store decoder state
        self.input_context = Some(input);
        self.video_stream_index = Some(video_stream_index);
        self.decoder = Some(decoder);
        self.format = Some(format.clone());
        self.current_position = Duration::ZERO;
        self.error_recovery_attempts = 0;
        
        Ok(format)
    }
    
    fn seek(&mut self, timestamp: Duration) -> Result<()> {
        let input = self.input_context.as_mut()
            .ok_or_else(|| VideoError::DecoderError("Decoder not opened".to_string()))?;
        
        let stream_index = self.video_stream_index
            .ok_or_else(|| VideoError::DecoderError("No video stream".to_string()))?;
        
        let position = timestamp.as_micros() as i64;
        
        // Seek to the nearest keyframe before the requested timestamp
        input.seek(stream_index, position, position, position, ffmpeg::format::seek::Flags::BACKWARD)
            .map_err(|e| VideoError::FFmpegError(format!("Seek failed: {}", e)))?;
        
        // Flush decoder buffers
        if let Some(decoder) = &mut self.decoder {
            decoder.flush();
        }
        
        self.current_position = timestamp;
        
        Ok(())
    }
    
    fn decode_frame(&mut self) -> Result<Option<Frame>> {
        let input = self.input_context.as_mut()
            .ok_or_else(|| VideoError::DecoderError("Decoder not opened".to_string()))?;
        
        let decoder = self.decoder.as_mut()
            .ok_or_else(|| VideoError::DecoderError("Decoder not initialized".to_string()))?;
        
        let video_stream_index = self.video_stream_index
            .ok_or_else(|| VideoError::DecoderError("No video stream".to_string()))?;
        
        let format = self.format.as_ref()
            .ok_or_else(|| VideoError::DecoderError("Format not initialized".to_string()))?;
        
        // Read packets until we get a video frame
        for (stream, packet) in input.packets() {
            if stream.index() != video_stream_index {
                continue;
            }
            
            // Send packet to decoder
            match decoder.send_packet(&packet) {
                Ok(()) => {},
                Err(ffmpeg::Error::Eof) => return Ok(None),
                Err(e) => {
                    self.recover_from_error()?;
                    return Err(VideoError::FFmpegError(format!("Failed to send packet: {}", e)));
                }
            }
            
            // Try to receive decoded frame
            let mut decoded = ffmpeg::frame::Video::empty();
            match decoder.receive_frame(&mut decoded) {
                Ok(()) => {
                    // Update current position
                    if let Some(pts) = decoded.pts() {
                        let time_base = stream.time_base();
                        let timestamp_us = pts * time_base.0 as i64 * 1_000_000 / time_base.1 as i64;
                        self.current_position = Duration::from_micros(timestamp_us as u64);
                    }
                    
                    // Convert frame to target pixel format if needed
                    let source_format = decoded.format();
                    let target_ffmpeg_format = Self::to_ffmpeg_pixel_format(self.target_pixel_format);
                    
                    let frame_data = if source_format != target_ffmpeg_format {
                        // Need format conversion
                        if self.scaler.is_none() {
                            self.scaler = Some(
                                ffmpeg::software::scaling::Context::get(
                                    source_format,
                                    decoded.width(),
                                    decoded.height(),
                                    target_ffmpeg_format,
                                    format.width,
                                    format.height,
                                    ffmpeg::software::scaling::Flags::BILINEAR,
                                )
                                .map_err(|e| VideoError::FFmpegError(format!("Failed to create scaler: {}", e)))?
                            );
                        }
                        
                        let scaler = self.scaler.as_mut().unwrap();
                        let mut scaled = ffmpeg::frame::Video::empty();
                        scaler.run(&decoded, &mut scaled)
                            .map_err(|e| VideoError::FFmpegError(format!("Failed to scale frame: {}", e)))?;
                        
                        // Extract data from scaled frame
                        scaled.data(0).to_vec()
                    } else {
                        // No conversion needed
                        decoded.data(0).to_vec()
                    };
                    
                    let frame = Frame {
                        timestamp: self.current_position,
                        width: format.width,
                        height: format.height,
                        pixel_format: self.target_pixel_format,
                        data: Arc::new(frame_data),
                        is_keyframe: decoded.is_key(),
                        frame_number: Some(decoded.pts().unwrap_or(0) as u64),
                    };
                    
                    self.error_recovery_attempts = 0; // Reset on successful decode
                    return Ok(Some(frame));
                }
                Err(ffmpeg::Error::EAGAIN) => {
                    // Need more data
                    continue;
                }
                Err(ffmpeg::Error::Eof) => {
                    return Ok(None);
                }
                Err(e) => {
                    self.recover_from_error()?;
                    return Err(VideoError::FFmpegError(format!("Failed to receive frame: {}", e)));
                }
            }
        }
        
        // No more packets
        Ok(None)
    }
    
    fn current_position(&self) -> Duration {
        self.current_position
    }
    
    fn duration(&self) -> Option<Duration> {
        self.format.as_ref()
            .and_then(|f| f.duration_ms)
            .map(Duration::from_millis)
    }
    
    fn flush(&mut self) -> Result<()> {
        if let Some(decoder) = &mut self.decoder {
            decoder.flush();
        }
        Ok(())
    }
    
    fn close(&mut self) -> Result<()> {
        self.decoder = None;
        self.input_context = None;
        self.video_stream_index = None;
        self.format = None;
        self.scaler = None;
        self.current_position = Duration::ZERO;
        self.error_recovery_attempts = 0;
        Ok(())
    }
}

impl Drop for FFmpegDecoder {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decoder_creation() {
        let decoder = FFmpegDecoder::new();
        assert_eq!(decoder.current_position, Duration::ZERO);
        assert!(decoder.input_context.is_none());
    }
    
    #[test]
    fn test_pixel_format_conversion() {
        let formats = vec![
            (ffmpeg::format::Pixel::RGB24, PixelFormat::RGB24),
            (ffmpeg::format::Pixel::YUV420P, PixelFormat::YUV420P),
            (ffmpeg::format::Pixel::RGBA, PixelFormat::RGBA),
        ];
        
        for (ffmpeg_format, our_format) in formats {
            assert_eq!(FFmpegDecoder::convert_pixel_format(ffmpeg_format), our_format);
            assert_eq!(FFmpegDecoder::to_ffmpeg_pixel_format(our_format), ffmpeg_format);
        }
    }
}