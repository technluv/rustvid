use crate::{error::Result, parameters::ParameterValue};
use glam::{Vec2, Vec4};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a video frame that can be processed by effects
#[derive(Debug, Clone)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: PixelFormat,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PixelFormat {
    Rgba8,
    Bgra8,
    Rgb8,
}

/// Base trait for all video effects
pub trait Effect: Send + Sync {
    /// Unique identifier for this effect instance
    fn id(&self) -> Uuid;
    
    /// Human-readable name of the effect
    fn name(&self) -> &str;
    
    /// Get current parameters
    fn parameters(&self) -> &HashMap<String, ParameterValue>;
    
    /// Set a parameter value
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()>;
    
    /// Apply the effect to a frame
    fn apply(&mut self, frame: &Frame, time: f64) -> Result<Frame>;
    
    /// Prepare GPU resources if needed
    fn prepare_gpu(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<()>;
    
    /// Check if this effect requires GPU processing
    fn requires_gpu(&self) -> bool {
        false
    }
    
    /// Get the effect's description
    fn description(&self) -> &str {
        ""
    }
    
    /// Clone the effect
    fn clone_effect(&self) -> Box<dyn Effect>;
}

/// Trait for filter effects (single input/output)
pub trait Filter: Effect {
    /// Get filter-specific properties
    fn filter_type(&self) -> FilterType;
    
    /// Check if the filter can be applied in-place
    fn supports_inplace(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterType {
    ColorCorrection,
    Blur,
    Sharpen,
    Distortion,
    Stylize,
    Custom,
}

/// Trait for transition effects (two inputs, one output)
pub trait Transition: Effect {
    /// Apply transition between two frames
    fn apply_transition(
        &mut self,
        from_frame: &Frame,
        to_frame: &Frame,
        progress: f32,
        time: f64,
    ) -> Result<Frame>;
    
    /// Get transition duration in seconds
    fn duration(&self) -> f64;
    
    /// Set transition duration
    fn set_duration(&mut self, duration: f64);
    
    /// Get transition type
    fn transition_type(&self) -> TransitionType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionType {
    Fade,
    Dissolve,
    Wipe,
    Slide,
    Zoom,
    Custom,
}

/// Trait for effects that support keyframe animation
pub trait Keyframable {
    /// Add a keyframe at the specified time
    fn add_keyframe(&mut self, time: f64, parameter: &str, value: ParameterValue) -> Result<()>;
    
    /// Remove a keyframe
    fn remove_keyframe(&mut self, time: f64, parameter: &str) -> Result<()>;
    
    /// Get all keyframes for a parameter
    fn get_keyframes(&self, parameter: &str) -> Vec<(f64, ParameterValue)>;
    
    /// Interpolate parameter value at given time
    fn interpolate_parameter(&self, parameter: &str, time: f64) -> Result<ParameterValue>;
}

/// Trait for effects that can be previewed in real-time
pub trait Previewable {
    /// Generate a preview frame
    fn preview(&self, input: &Frame, preview_quality: PreviewQuality) -> Result<Frame>;
    
    /// Check if real-time preview is supported
    fn supports_realtime_preview(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewQuality {
    Low,
    Medium,
    High,
    Full,
}

/// Trait for effects that can be serialized/deserialized
pub trait Persistable {
    /// Serialize the effect to JSON
    fn to_json(&self) -> Result<String>;
    
    /// Deserialize from JSON
    fn from_json(json: &str) -> Result<Self>
    where
        Self: Sized;
}

/// Factory trait for creating effects
pub trait EffectFactory: Send + Sync {
    /// Create a new effect instance
    fn create(&self, name: &str) -> Result<Box<dyn Effect>>;
    
    /// List available effect names
    fn available_effects(&self) -> Vec<String>;
    
    /// Get effect metadata
    fn effect_metadata(&self, name: &str) -> Option<EffectMetadata>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectMetadata {
    pub name: String,
    pub category: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub requires_gpu: bool,
    pub parameters: Vec<ParameterMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterMetadata {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub parameter_type: ParameterType,
    pub default_value: ParameterValue,
    pub min_value: Option<ParameterValue>,
    pub max_value: Option<ParameterValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    Float,
    Int,
    Bool,
    Color,
    Vec2,
    Vec3,
    Vec4,
    String,
}