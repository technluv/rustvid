// Re-export filter effects
pub use crate::effects::{
    blur::{BlurEffect, GaussianBlurEffect},
    brightness_contrast::{BrightnessContrastEffect, SaturationEffect},
    color_correction::{ColorCorrectionEffect, LevelsEffect},
};

use crate::{
    error::Result,
    traits::*,
};

/// Factory for creating filter effects
pub struct FilterFactory;

impl FilterFactory {
    pub fn new() -> Self {
        Self
    }
    
    pub fn create_blur(radius: f32) -> Result<Box<dyn Filter>> {
        let mut effect = BlurEffect::new();
        effect.set_parameter("radius", crate::parameters::ParameterValue::Float(radius))?;
        Ok(Box::new(effect))
    }
    
    pub fn create_gaussian_blur(radius: f32, sigma: f32) -> Result<Box<dyn Filter>> {
        let mut effect = GaussianBlurEffect::new();
        effect.set_parameter("radius", crate::parameters::ParameterValue::Float(radius))?;
        effect.set_parameter("sigma", crate::parameters::ParameterValue::Float(sigma))?;
        Ok(Box::new(effect))
    }
    
    pub fn create_brightness_contrast(brightness: f32, contrast: f32) -> Result<Box<dyn Filter>> {
        let mut effect = BrightnessContrastEffect::new();
        effect.set_parameter("brightness", crate::parameters::ParameterValue::Float(brightness))?;
        effect.set_parameter("contrast", crate::parameters::ParameterValue::Float(contrast))?;
        Ok(Box::new(effect))
    }
    
    pub fn create_saturation(saturation: f32) -> Result<Box<dyn Filter>> {
        let mut effect = SaturationEffect::new();
        effect.set_parameter("saturation", crate::parameters::ParameterValue::Float(saturation))?;
        Ok(Box::new(effect))
    }
    
    pub fn create_color_correction(
        hue_shift: f32,
        saturation: f32,
        lightness: f32,
        temperature: f32,
        tint: f32,
    ) -> Result<Box<dyn Filter>> {
        let mut effect = ColorCorrectionEffect::new();
        effect.set_parameter("hue_shift", crate::parameters::ParameterValue::Float(hue_shift))?;
        effect.set_parameter("saturation", crate::parameters::ParameterValue::Float(saturation))?;
        effect.set_parameter("lightness", crate::parameters::ParameterValue::Float(lightness))?;
        effect.set_parameter("temperature", crate::parameters::ParameterValue::Float(temperature))?;
        effect.set_parameter("tint", crate::parameters::ParameterValue::Float(tint))?;
        Ok(Box::new(effect))
    }
    
    pub fn create_levels(
        input_black: f32,
        input_white: f32,
        gamma: f32,
        output_black: f32,
        output_white: f32,
    ) -> Result<Box<dyn Filter>> {
        let mut effect = LevelsEffect::new();
        effect.set_parameter("input_black", crate::parameters::ParameterValue::Float(input_black))?;
        effect.set_parameter("input_white", crate::parameters::ParameterValue::Float(input_white))?;
        effect.set_parameter("gamma", crate::parameters::ParameterValue::Float(gamma))?;
        effect.set_parameter("output_black", crate::parameters::ParameterValue::Float(output_black))?;
        effect.set_parameter("output_white", crate::parameters::ParameterValue::Float(output_white))?;
        Ok(Box::new(effect))
    }
}