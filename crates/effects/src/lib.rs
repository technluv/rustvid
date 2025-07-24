//! Video and audio effects system for Rust Video Editor
//! 
//! This crate provides a flexible effects system for applying
//! transformations to video and audio data.

use thiserror::Error;
use std::any::Any;

#[derive(Error, Debug)]
pub enum EffectError {
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
    
    #[error("Unsupported format")]
    UnsupportedFormat,
}

pub type Result<T> = std::result::Result<T, EffectError>;

/// Trait for all effects
pub trait Effect: Send + Sync {
    fn name(&self) -> &str;
    fn process(&mut self, input: &mut dyn Any) -> Result<()>;
    fn get_parameters(&self) -> Vec<Parameter>;
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()>;
}

/// Parameter types for effects
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ParameterValue {
    Float(f32),
    Integer(i32),
    Boolean(bool),
    String(String),
    Color(u32),
}

/// Effect parameter definition
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: ParameterValue,
    pub min: Option<ParameterValue>,
    pub max: Option<ParameterValue>,
}

/// Basic brightness effect example
pub struct BrightnessEffect {
    brightness: f32,
}

impl BrightnessEffect {
    pub fn new() -> Self {
        Self { brightness: 1.0 }
    }
}

impl Effect for BrightnessEffect {
    fn name(&self) -> &str {
        "Brightness"
    }
    
    fn process(&mut self, _input: &mut dyn Any) -> Result<()> {
        // Implementation would process the actual frame data
        Ok(())
    }
    
    fn get_parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter {
                name: "brightness".to_string(),
                value: ParameterValue::Float(self.brightness),
                min: Some(ParameterValue::Float(0.0)),
                max: Some(ParameterValue::Float(2.0)),
            }
        ]
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match (name, value) {
            ("brightness", ParameterValue::Float(v)) => {
                self.brightness = v;
                Ok(())
            }
            _ => Err(EffectError::InvalidParameter(name.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brightness_effect() {
        let mut effect = BrightnessEffect::new();
        assert_eq!(effect.name(), "Brightness");
        
        effect.set_parameter("brightness", ParameterValue::Float(1.5)).unwrap();
        let params = effect.get_parameters();
        
        match &params[0].value {
            ParameterValue::Float(v) => assert_eq!(*v, 1.5),
            _ => panic!("Expected float parameter"),
        }
    }
}