pub mod blur;
pub mod brightness_contrast;
pub mod color_correction;

use crate::{
    error::Result,
    parameters::{ParameterManager, ParameterValue},
    traits::*,
};
use std::collections::HashMap;
use uuid::Uuid;

/// Base implementation for common effect functionality
pub struct BaseEffect {
    pub id: Uuid,
    pub name: String,
    pub parameters: ParameterManager,
    pub requires_gpu: bool,
}

impl BaseEffect {
    pub fn new(name: impl Into<String>, requires_gpu: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            parameters: ParameterManager::new(),
            requires_gpu,
        }
    }
}

/// Registry for all available effects
pub struct EffectRegistry {
    factories: HashMap<String, Box<dyn Fn() -> Box<dyn Effect> + Send + Sync>>,
}

impl EffectRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            factories: HashMap::new(),
        };
        
        // Register built-in effects
        registry.register_builtin_effects();
        
        registry
    }
    
    fn register_builtin_effects(&mut self) {
        // Blur effects
        self.register("blur", || Box::new(blur::BlurEffect::new()));
        self.register("gaussian_blur", || Box::new(blur::GaussianBlurEffect::new()));
        
        // Color adjustment effects
        self.register("brightness_contrast", || {
            Box::new(brightness_contrast::BrightnessContrastEffect::new())
        });
        self.register("saturation", || {
            Box::new(brightness_contrast::SaturationEffect::new())
        });
        
        // Color correction effects
        self.register("color_correction", || {
            Box::new(color_correction::ColorCorrectionEffect::new())
        });
        self.register("levels", || Box::new(color_correction::LevelsEffect::new()));
    }
    
    pub fn register<F>(&mut self, name: &str, factory: F)
    where
        F: Fn() -> Box<dyn Effect> + Send + Sync + 'static,
    {
        self.factories.insert(name.to_string(), Box::new(factory));
    }
    
    pub fn create(&self, name: &str) -> Option<Box<dyn Effect>> {
        self.factories.get(name).map(|factory| factory())
    }
    
    pub fn list_effects(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }
}

impl Default for EffectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory implementation
pub struct DefaultEffectFactory {
    registry: EffectRegistry,
}

impl DefaultEffectFactory {
    pub fn new() -> Self {
        Self {
            registry: EffectRegistry::new(),
        }
    }
}

impl EffectFactory for DefaultEffectFactory {
    fn create(&self, name: &str) -> Result<Box<dyn Effect>> {
        self.registry
            .create(name)
            .ok_or_else(|| crate::error::EffectError::Other(
                anyhow::anyhow!("Effect '{}' not found", name)
            ))
    }
    
    fn available_effects(&self) -> Vec<String> {
        self.registry.list_effects()
    }
    
    fn effect_metadata(&self, name: &str) -> Option<EffectMetadata> {
        // Return metadata for known effects
        match name {
            "blur" => Some(EffectMetadata {
                name: "Blur".to_string(),
                category: "Filter".to_string(),
                description: "Box blur effect".to_string(),
                author: "Built-in".to_string(),
                version: "1.0.0".to_string(),
                requires_gpu: true,
                parameters: vec![
                    ParameterMetadata {
                        name: "radius".to_string(),
                        display_name: "Blur Radius".to_string(),
                        description: "Radius of the blur in pixels".to_string(),
                        parameter_type: ParameterType::Float,
                        default_value: ParameterValue::Float(5.0),
                        min_value: Some(ParameterValue::Float(0.0)),
                        max_value: Some(ParameterValue::Float(50.0)),
                    },
                ],
            }),
            "brightness_contrast" => Some(EffectMetadata {
                name: "Brightness/Contrast".to_string(),
                category: "Color".to_string(),
                description: "Adjust brightness and contrast".to_string(),
                author: "Built-in".to_string(),
                version: "1.0.0".to_string(),
                requires_gpu: true,
                parameters: vec![
                    ParameterMetadata {
                        name: "brightness".to_string(),
                        display_name: "Brightness".to_string(),
                        description: "Brightness adjustment".to_string(),
                        parameter_type: ParameterType::Float,
                        default_value: ParameterValue::Float(0.0),
                        min_value: Some(ParameterValue::Float(-1.0)),
                        max_value: Some(ParameterValue::Float(1.0)),
                    },
                    ParameterMetadata {
                        name: "contrast".to_string(),
                        display_name: "Contrast".to_string(),
                        description: "Contrast adjustment".to_string(),
                        parameter_type: ParameterType::Float,
                        default_value: ParameterValue::Float(1.0),
                        min_value: Some(ParameterValue::Float(0.0)),
                        max_value: Some(ParameterValue::Float(3.0)),
                    },
                ],
            }),
            _ => None,
        }
    }
}