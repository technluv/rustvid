//! Error types for video processing
//! 
//! This module defines comprehensive error types for all video processing
//! operations, providing detailed error information for debugging and recovery.

use std::fmt;
use thiserror::Error;

/// Main error type for video processing operations
#[derive(Error, Debug)]
pub enum VideoError {
    /// Codec-related errors
    #[error("Codec error: {0}")]
    Codec(CodecError),
    
    /// Decoder-specific errors
    #[error("Decoder error: {0}")]
    Decoder(String),
    
    /// Encoder-specific errors
    #[error("Encoder error: {0}")]
    Encoder(String),
    
    /// Frame processing errors
    #[error("Frame error: {0}")]
    Frame(String),
    
    /// Invalid frame data
    #[error("Invalid frame data: {0}")]
    InvalidFrameData(String),
    
    /// Format-related errors
    #[error("Format error: {0}")]
    Format(FormatError),
    
    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Filter errors
    #[error("Filter error: {0}")]
    Filter(FilterError),
    
    /// Pipeline errors
    #[error("Pipeline error: {0}")]
    Pipeline(String),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    /// Unsupported operation
    #[error("Unsupported operation: {0}")]
    Unsupported(String),
    
    /// Resource errors
    #[error("Resource error: {0}")]
    Resource(ResourceError),
    
    /// Timeout errors
    #[error("Operation timed out: {0}")]
    Timeout(String),
    
    /// Unknown errors
    #[error("Unknown error: {0}")]
    Unknown(String),
    
    /// FFmpeg-specific errors (when using FFmpeg backend)
    #[error("FFmpeg error: {0}")]
    FFmpeg(FFmpegError),
}

/// Codec-specific errors
#[derive(Error, Debug)]
pub enum CodecError {
    #[error("Codec not found: {0}")]
    NotFound(String),
    
    #[error("Codec initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Unsupported codec: {0}")]
    Unsupported(String),
    
    #[error("Codec parameter error: {0}")]
    InvalidParameter(String),
    
    #[error("Codec already initialized")]
    AlreadyInitialized,
    
    #[error("Codec not initialized")]
    NotInitialized,
}

/// Format-specific errors
#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Unknown format: {0}")]
    Unknown(String),
    
    #[error("Invalid format parameters: {0}")]
    InvalidParameters(String),
    
    #[error("Format not supported: {0}")]
    NotSupported(String),
    
    #[error("Format conversion failed: {0}")]
    ConversionFailed(String),
    
    #[error("Invalid pixel format: {0}")]
    InvalidPixelFormat(String),
}

/// Filter-specific errors
#[derive(Error, Debug)]
pub enum FilterError {
    #[error("Filter not found: {0}")]
    NotFound(String),
    
    #[error("Filter initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Invalid filter parameter: {0}")]
    InvalidParameter(String),
    
    #[error("Filter graph error: {0}")]
    GraphError(String),
    
    #[error("Filter processing failed: {0}")]
    ProcessingFailed(String),
}

/// Resource-related errors
#[derive(Error, Debug)]
pub enum ResourceError {
    #[error("Out of memory")]
    OutOfMemory,
    
    #[error("Resource limit exceeded: {0}")]
    LimitExceeded(String),
    
    #[error("Resource not available: {0}")]
    NotAvailable(String),
    
    #[error("Resource allocation failed: {0}")]
    AllocationFailed(String),
}

/// FFmpeg-specific errors
#[derive(Debug)]
pub struct FFmpegError {
    pub code: i32,
    pub message: String,
}

impl fmt::Display for FFmpegError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code {}: {}", self.code, self.message)
    }
}

impl std::error::Error for FFmpegError {}

/// Result type alias for video operations
pub type Result<T> = std::result::Result<T, VideoError>;

/// Extension trait for converting FFmpeg error codes
pub trait FFmpegErrorExt {
    /// Converts an FFmpeg error code to a VideoError
    fn to_video_error(self, context: &str) -> VideoError;
}

impl FFmpegErrorExt for i32 {
    fn to_video_error(self, context: &str) -> VideoError {
        if self >= 0 {
            VideoError::Unknown(format!("Unexpected success code {} in context: {}", self, context))
        } else {
            VideoError::FFmpeg(FFmpegError {
                code: self,
                message: format!("{} (error code: {})", context, self),
            })
        }
    }
}

/// Helper functions for error handling
impl VideoError {
    /// Creates a codec not found error
    pub fn codec_not_found(codec: impl Into<String>) -> Self {
        VideoError::Codec(CodecError::NotFound(codec.into()))
    }
    
    /// Creates an unsupported operation error
    pub fn unsupported(operation: impl Into<String>) -> Self {
        VideoError::Unsupported(operation.into())
    }
    
    /// Creates a configuration error
    pub fn config(message: impl Into<String>) -> Self {
        VideoError::Configuration(message.into())
    }
    
    /// Creates a format error
    pub fn format(message: impl Into<String>) -> Self {
        VideoError::Format(FormatError::Unknown(message.into()))
    }
    
    /// Checks if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            VideoError::Timeout(_) | VideoError::Resource(ResourceError::NotAvailable(_))
        )
    }
    
    /// Checks if this is a codec error
    pub fn is_codec_error(&self) -> bool {
        matches!(self, VideoError::Codec(_))
    }
    
    /// Checks if this is an IO error
    pub fn is_io_error(&self) -> bool {
        matches!(self, VideoError::Io(_))
    }
}

/// Conversion from standard error types
impl From<fmt::Error> for VideoError {
    fn from(err: fmt::Error) -> Self {
        VideoError::Unknown(err.to_string())
    }
}

impl From<std::num::TryFromIntError> for VideoError {
    fn from(err: std::num::TryFromIntError) -> Self {
        VideoError::Configuration(format!("Integer conversion error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let err = VideoError::codec_not_found("h264");
        assert!(err.is_codec_error());
        
        let err = VideoError::unsupported("hardware encoding");
        match err {
            VideoError::Unsupported(msg) => assert_eq!(msg, "hardware encoding"),
            _ => panic!("Wrong error type"),
        }
    }
    
    #[test]
    fn test_error_display() {
        let err = VideoError::Codec(CodecError::NotFound("vp9".to_string()));
        assert_eq!(err.to_string(), "Codec error: Codec not found: vp9");
        
        let err = VideoError::Format(FormatError::InvalidPixelFormat("xyz".to_string()));
        assert_eq!(err.to_string(), "Format error: Invalid pixel format: xyz");
    }
    
    #[test]
    fn test_ffmpeg_error_conversion() {
        let err = (-22).to_video_error("Failed to open file");
        match err {
            VideoError::FFmpeg(ffmpeg_err) => {
                assert_eq!(ffmpeg_err.code, -22);
                assert!(ffmpeg_err.message.contains("Failed to open file"));
            }
            _ => panic!("Wrong error type"),
        }
    }
    
    #[test]
    fn test_error_classification() {
        let err = VideoError::Timeout("Operation took too long".to_string());
        assert!(err.is_recoverable());
        
        let err = VideoError::Codec(CodecError::NotFound("test".to_string()));
        assert!(!err.is_recoverable());
        assert!(err.is_codec_error());
        
        let err = VideoError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file"));
        assert!(err.is_io_error());
    }
}