//! Video frame representation and utilities

use crate::{traits::PixelFormat, error::{VideoError, Result}};
use std::sync::Arc;
use std::time::Duration;
use serde::{Serialize, Deserialize};

/// Color space for video frames
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorSpace {
    BT601,
    BT709,
    BT2020,
    SRGB,
}

/// HDR metadata for frames
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HdrMetadata {
    pub max_display_mastering_luminance: f32,
    pub min_display_mastering_luminance: f32,
    pub max_content_light_level: u16,
    pub max_frame_average_light_level: u16,
}

/// Frame metadata
#[derive(Debug, Clone, Default)]
pub struct FrameMetadata {
    pub pts: Option<i64>,
    pub dts: Option<i64>,
    pub duration: Option<Duration>,
    pub key_frame: bool,
    pub color_space: ColorSpace,
    pub hdr_metadata: Option<HdrMetadata>,
}

impl Default for ColorSpace {
    fn default() -> Self {
        ColorSpace::BT709
    }
}

/// Represents a video frame with its data and metadata
#[derive(Debug, Clone)]
pub struct Frame {
    /// Presentation timestamp
    pub timestamp: Duration,
    
    /// Frame width in pixels
    pub width: u32,
    
    /// Frame height in pixels
    pub height: u32,
    
    /// Pixel format
    pub format: PixelFormat,
    
    /// Raw frame data (layout depends on pixel_format)
    pub data: Vec<u8>,
    
    /// Frame metadata
    pub metadata: FrameMetadata,
}

impl Frame {
    /// Creates a new frame with the given parameters
    pub fn new(
        width: u32,
        height: u32,
        format: PixelFormat,
        timestamp: Duration,
    ) -> Result<Self> {
        // Validate dimensions
        if width == 0 || height == 0 {
            return Err(VideoError::InvalidParameters("Frame dimensions must be non-zero".into()));
        }
        
        if width > 16384 || height > 16384 {
            return Err(VideoError::InvalidParameters("Frame dimensions exceed maximum (16384x16384)".into()));
        }
        
        // Calculate data size
        let data_size = Self::calculate_data_size(width, height, format);
        if data_size > 1024 * 1024 * 1024 { // 1GB limit
            return Err(VideoError::InvalidParameters("Frame data size exceeds 1GB limit".into()));
        }
        
        Ok(Self {
            timestamp,
            width,
            height,
            format,
            data: vec![0u8; data_size],
            metadata: FrameMetadata::default(),
        })
    }
    
    /// Calculate the data size for given dimensions and format
    fn calculate_data_size(width: u32, height: u32, format: PixelFormat) -> usize {
        match format {
            PixelFormat::RGB24 | PixelFormat::BGR24 => (width * height * 3) as usize,
            PixelFormat::RGBA | PixelFormat::BGRA => (width * height * 4) as usize,
            PixelFormat::YUV420P | PixelFormat::NV12 | PixelFormat::NV21 => {
                ((width * height * 3) / 2) as usize
            }
            PixelFormat::YUV422P => (width * height * 2) as usize,
            PixelFormat::YUV444P => (width * height * 3) as usize,
            PixelFormat::Gray8 => (width * height) as usize,
        }
    }
    
    /// Returns the size of the frame data in bytes
    pub fn data_size(&self) -> usize {
        self.data.len()
    }
    
    /// Calculates the expected data size for the given pixel format
    pub fn expected_data_size(&self) -> usize {
        Self::calculate_data_size(self.width, self.height, self.format)
    }
    
    /// Validates that the frame data size matches the expected size
    pub fn validate(&self) -> bool {
        self.width > 0 && 
        self.height > 0 && 
        self.data_size() == self.expected_data_size()
    }
}

/// Builder for creating frames with specific configurations
pub struct FrameBuilder {
    width: Option<u32>,
    height: Option<u32>,
    format: Option<PixelFormat>,
    timestamp: Option<Duration>,
    pts: Option<i64>,
    dts: Option<i64>,
    key_frame: bool,
    color_space: ColorSpace,
}

impl FrameBuilder {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            format: None,
            timestamp: None,
            pts: None,
            dts: None,
            key_frame: false,
            color_space: ColorSpace::BT709,
        }
    }
    
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }
    
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
    
    pub fn format(mut self, format: PixelFormat) -> Self {
        self.format = Some(format);
        self
    }
    
    pub fn timestamp(mut self, timestamp: Duration) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    
    pub fn pts(mut self, pts: Option<i64>) -> Self {
        self.pts = pts;
        self
    }
    
    pub fn dts(mut self, dts: Option<i64>) -> Self {
        self.dts = dts;
        self
    }
    
    pub fn key_frame(mut self, key_frame: bool) -> Self {
        self.key_frame = key_frame;
        self
    }
    
    pub fn color_space(mut self, color_space: ColorSpace) -> Self {
        self.color_space = color_space;
        self
    }
    
    pub fn build(self) -> Result<Frame> {
        let width = self.width.ok_or(VideoError::InvalidParameters("Width not set".into()))?;
        let height = self.height.ok_or(VideoError::InvalidParameters("Height not set".into()))?;
        let format = self.format.ok_or(VideoError::InvalidParameters("Format not set".into()))?;
        let timestamp = self.timestamp.ok_or(VideoError::InvalidParameters("Timestamp not set".into()))?;
        
        let mut frame = Frame::new(width, height, format, timestamp)?;
        frame.metadata.pts = self.pts;
        frame.metadata.dts = self.dts;
        frame.metadata.key_frame = self.key_frame;
        frame.metadata.color_space = self.color_space;
        
        Ok(frame)
    }
}

#[cfg(test)]
mod tests;