//! Core video processing engine for Rust Video Editor
//! 
//! This crate provides the fundamental video processing capabilities
//! including frame handling, codec abstraction, and pipeline management.
//! 
//! # Architecture
//! 
//! The core library is organized around several key concepts:
//! 
//! - **Traits**: Define the interfaces for video processing operations
//! - **Frame**: Represents individual video frames with metadata
//! - **Decoder**: FFmpeg-based video decoding implementation
//! - **Buffer**: High-performance frame buffering system
//! - **Error**: Comprehensive error handling for all operations
//! 
//! # Example
//! 
//! ```no_run
//! use rust_video_core::{Frame, VideoFormat, traits::*};
//! use std::time::Duration;
//! 
//! // Create a new frame
//! let frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_secs(0))
//!     .expect("Failed to create frame");
//! ```

// Public modules
pub mod error;
pub mod frame;
pub mod traits;
pub mod decoder;
pub mod buffer;
pub mod pipeline;

// Re-exports for convenience
pub use error::{VideoError, Result};
pub use frame::{Frame, FrameBuilder, FrameMetadata, ColorSpace, HdrMetadata};
pub use traits::{
    VideoDecoder, VideoEncoder, VideoProcessor, VideoFilter,
    PixelFormat, PixelFormatConverter,
    VideoDemuxer, VideoMuxer, EncodingStats,
    FilterParameter, FilterParameterInfo, FilterParameterType,
    MediaInfo, StreamInfo, StreamType, Packet,
    VideoStreamInfo, AudioStreamInfo,
};

use serde::{Serialize, Deserialize};

/// Video format information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFormat {
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub codec: String,
    pub duration_ms: Option<u64>,
    pub bit_rate: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_format_serialization() {
        let format = VideoFormat {
            width: 1920,
            height: 1080,
            fps: 30.0,
            codec: "h264".to_string(),
        };
        
        let json = serde_json::to_string(&format).unwrap();
        let deserialized: VideoFormat = serde_json::from_str(&json).unwrap();
        
        assert_eq!(format.width, deserialized.width);
        assert_eq!(format.height, deserialized.height);
    }
}