//! Core video processing engine for Rust Video Editor
//! 
//! This crate provides the fundamental video processing capabilities
//! including frame handling, codec abstraction, and pipeline management.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Video processing error: {0}")]
    ProcessingError(String),
    
    #[error("Codec error: {0}")]
    CodecError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, CoreError>;

/// Represents a video frame
#[derive(Debug, Clone)]
pub struct Frame {
    pub timestamp: std::time::Duration,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

/// Video format information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VideoFormat {
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub codec: String,
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