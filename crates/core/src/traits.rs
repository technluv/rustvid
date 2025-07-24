//! Core traits for video processing
//! 
//! These traits define the fundamental interfaces for video processing
//! operations, designed to be codec-agnostic but FFmpeg-friendly.

use crate::{error::VideoError, frame::Frame, VideoFormat};
use std::time::Duration;

/// Trait for decoding video streams
pub trait VideoDecoder: Send + Sync {
    /// Opens a video file or stream for decoding
    fn open(&mut self, path: &str) -> Result<VideoFormat, VideoError>;
    
    /// Seeks to a specific timestamp
    fn seek(&mut self, timestamp: Duration) -> Result<(), VideoError>;
    
    /// Decodes the next frame
    fn decode_frame(&mut self) -> Result<Option<Frame>, VideoError>;
    
    /// Gets the current position in the stream
    fn current_position(&self) -> Duration;
    
    /// Gets the total duration of the video
    fn duration(&self) -> Option<Duration>;
    
    /// Flushes any pending frames
    fn flush(&mut self) -> Result<(), VideoError>;
    
    /// Closes the decoder and releases resources
    fn close(&mut self) -> Result<(), VideoError>;
}

/// Trait for encoding video streams
pub trait VideoEncoder: Send + Sync {
    /// Initializes the encoder with the specified format
    fn initialize(&mut self, format: &VideoFormat, output_path: &str) -> Result<(), VideoError>;
    
    /// Encodes a frame
    fn encode_frame(&mut self, frame: &Frame) -> Result<(), VideoError>;
    
    /// Flushes any pending frames and finalizes the output
    fn finalize(&mut self) -> Result<(), VideoError>;
    
    /// Sets encoding parameters
    fn set_option(&mut self, key: &str, value: &str) -> Result<(), VideoError>;
    
    /// Gets the current encoding statistics
    fn stats(&self) -> EncodingStats;
}

/// Trait for video frame transformations
pub trait VideoProcessor: Send + Sync {
    /// Processes a single frame
    fn process_frame(&mut self, frame: &mut Frame) -> Result<(), VideoError>;
    
    /// Returns the name of this processor
    fn name(&self) -> &str;
    
    /// Checks if this processor can be parallelized
    fn is_parallelizable(&self) -> bool {
        false
    }
    
    /// Clones the processor for parallel processing
    fn clone_box(&self) -> Box<dyn VideoProcessor>;
}

/// Trait for video filters that can be chained
pub trait VideoFilter: VideoProcessor {
    /// Sets filter-specific parameters
    fn set_parameter(&mut self, name: &str, value: FilterParameter) -> Result<(), VideoError>;
    
    /// Gets current filter parameters
    fn parameters(&self) -> Vec<FilterParameterInfo>;
}

/// Trait for pixel format conversion
pub trait PixelFormatConverter: Send + Sync {
    /// Converts from one pixel format to another
    fn convert(
        &self,
        input: &[u8],
        input_format: PixelFormat,
        output_format: PixelFormat,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, VideoError>;
    
    /// Checks if a conversion is supported
    fn supports_conversion(&self, from: PixelFormat, to: PixelFormat) -> bool;
}

/// Trait for video stream demuxing
pub trait VideoDemuxer: Send + Sync {
    /// Opens a media file for demuxing
    fn open(&mut self, path: &str) -> Result<MediaInfo, VideoError>;
    
    /// Reads the next packet from any stream
    fn read_packet(&mut self) -> Result<Option<Packet>, VideoError>;
    
    /// Seeks to a specific position
    fn seek(&mut self, stream_index: usize, timestamp: Duration) -> Result<(), VideoError>;
    
    /// Gets information about available streams
    fn streams(&self) -> &[StreamInfo];
}

/// Trait for video stream muxing
pub trait VideoMuxer: Send + Sync {
    /// Creates a new output file
    fn create(&mut self, path: &str, format: &str) -> Result<(), VideoError>;
    
    /// Adds a stream to the output
    fn add_stream(&mut self, stream_info: &StreamInfo) -> Result<usize, VideoError>;
    
    /// Writes a packet to the output
    fn write_packet(&mut self, packet: &Packet) -> Result<(), VideoError>;
    
    /// Finalizes the output file
    fn finalize(&mut self) -> Result<(), VideoError>;
}

/// Encoding statistics
#[derive(Debug, Clone, Default)]
pub struct EncodingStats {
    pub frames_encoded: u64,
    pub bytes_written: u64,
    pub encoding_speed: f32,
    pub average_bitrate: u32,
}

/// Filter parameter types
#[derive(Debug, Clone)]
pub enum FilterParameter {
    Float(f32),
    Integer(i32),
    String(String),
    Boolean(bool),
}

/// Filter parameter information
#[derive(Debug, Clone)]
pub struct FilterParameterInfo {
    pub name: String,
    pub description: String,
    pub parameter_type: FilterParameterType,
    pub default_value: FilterParameter,
    pub min_value: Option<FilterParameter>,
    pub max_value: Option<FilterParameter>,
}

/// Filter parameter type information
#[derive(Debug, Clone)]
pub enum FilterParameterType {
    Float,
    Integer,
    String,
    Boolean,
}

/// Pixel format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    RGB24,
    RGBA,
    BGR24,
    BGRA,
    YUV420P,
    YUV422P,
    YUV444P,
    NV12,
    NV21,
    Gray8,
}

impl PixelFormat {
    /// Returns the number of bytes per pixel for this format
    /// Note: For planar formats, this returns the total bytes across all planes
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            PixelFormat::RGB24 | PixelFormat::BGR24 | PixelFormat::YUV444P => 3,
            PixelFormat::RGBA | PixelFormat::BGRA => 4,
            PixelFormat::YUV420P | PixelFormat::NV12 | PixelFormat::NV21 => 3, // 1.5 bytes/pixel average
            PixelFormat::YUV422P => 3, // 2 bytes/pixel average
            PixelFormat::Gray8 => 1,
        }
    }
}

/// Media information
#[derive(Debug, Clone)]
pub struct MediaInfo {
    pub duration: Option<Duration>,
    pub format_name: String,
    pub bit_rate: Option<u64>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Stream information
#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub index: usize,
    pub stream_type: StreamType,
    pub codec_name: String,
    pub time_base: (u32, u32),
    pub metadata: std::collections::HashMap<String, String>,
}

/// Stream type enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamType {
    Video(VideoStreamInfo),
    Audio(AudioStreamInfo),
    Subtitle,
    Data,
}

/// Video stream specific information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoStreamInfo {
    pub width: u32,
    pub height: u32,
    pub pixel_format: PixelFormat,
    pub frame_rate: (u32, u32),
}

/// Audio stream specific information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioStreamInfo {
    pub channels: u32,
    pub sample_rate: u32,
    pub channel_layout: String,
}

/// Media packet
#[derive(Debug, Clone)]
pub struct Packet {
    pub stream_index: usize,
    pub timestamp: Duration,
    pub duration: Duration,
    pub data: Vec<u8>,
    pub is_keyframe: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pixel_format_equality() {
        assert_eq!(PixelFormat::RGB24, PixelFormat::RGB24);
        assert_ne!(PixelFormat::RGB24, PixelFormat::RGBA);
    }
    
    #[test]
    fn test_filter_parameter_creation() {
        let param = FilterParameter::Float(1.5);
        match param {
            FilterParameter::Float(v) => assert_eq!(v, 1.5),
            _ => panic!("Wrong parameter type"),
        }
    }
}