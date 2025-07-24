use crate::{
    effects::BaseEffect,
    error::Result,
    gpu::{EffectPipeline, GpuBuffer},
    parameters::ParameterValue,
    shaders,
    traits::*,
};
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct BrightnessContrastParams {
    brightness: f32,
    contrast: f32,
    _padding: [f32; 2],
}

/// Brightness and contrast adjustment effect
pub struct BrightnessContrastEffect {
    base: BaseEffect,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
}

impl BrightnessContrastEffect {
    pub fn new() -> Self {
        let mut base = BaseEffect::new("Brightness/Contrast", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("brightness", ParameterValue::Float(0.0));
        base.parameters
            .set_parameter("contrast", ParameterValue::Float(1.0));
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
        }
    }
}

impl Effect for BrightnessContrastEffect {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        let mut params = HashMap::new();
        params.insert(
            "brightness".to_string(),
            self.base
                .parameters
                .get_parameter("brightness")
                .cloned()
                .unwrap_or(ParameterValue::Float(0.0)),
        );
        params.insert(
            "contrast".to_string(),
            self.base
                .parameters
                .get_parameter("contrast")
                .cloned()
                .unwrap_or(ParameterValue::Float(1.0)),
        );
        params
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match name {
            "brightness" => {
                let brightness = value.as_float()?;
                if brightness < -1.0 || brightness > 1.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Brightness must be between -1 and 1".to_string(),
                    ));
                }
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            "contrast" => {
                let contrast = value.as_float()?;
                if contrast < 0.0 || contrast > 3.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Contrast must be between 0 and 3".to_string(),
                    ));
                }
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            _ => Err(crate::error::EffectError::InvalidParameter(format!(
                "Unknown parameter: {}",
                name
            ))),
        }
    }
    
    fn apply(&mut self, frame: &Frame, time: f64) -> Result<Frame> {
        let brightness = self
            .base
            .parameters
            .get_parameter_at_time("brightness", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(0.0);
        
        let contrast = self
            .base
            .parameters
            .get_parameter_at_time("contrast", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(1.0);
        
        let mut output = frame.clone();
        
        match frame.format {
            PixelFormat::Rgba8 => {
                for i in (0..frame.data.len()).step_by(4) {
                    // Apply brightness and contrast to RGB channels
                    for j in 0..3 {
                        let value = frame.data[i + j] as f32 / 255.0;
                        
                        // Apply brightness (additive)
                        let bright = value + brightness;
                        
                        // Apply contrast (multiplicative around 0.5)
                        let contrasted = (bright - 0.5) * contrast + 0.5;
                        
                        // Clamp and convert back to byte
                        output.data[i + j] = (contrasted.clamp(0.0, 1.0) * 255.0) as u8;
                    }
                    // Preserve alpha channel
                    output.data[i + 3] = frame.data[i + 3];
                }
            }
            _ => {
                return Err(crate::error::EffectError::Other(anyhow::anyhow!(
                    "Unsupported pixel format"
                )));
            }
        }
        
        Ok(output)
    }
    
    fn prepare_gpu(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) -> Result<()> {
        if self.pipeline.is_none() {
            let shader = shaders::create_effect_shader(
                device,
                shaders::brightness_contrast::BRIGHTNESS_CONTRAST_SHADER,
                "Brightness/Contrast",
            )?;
            self.pipeline = Some(EffectPipeline::new(
                device,
                &shader,
                wgpu::TextureFormat::Rgba8UnormSrgb,
            ));
        }
        Ok(())
    }
    
    fn requires_gpu(&self) -> bool {
        self.base.requires_gpu
    }
    
    fn description(&self) -> &str {
        "Adjust image brightness and contrast"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_effect = Self::new();
        if let Some(brightness) = self.base.parameters.get_parameter("brightness") {
            new_effect
                .base
                .parameters
                .set_parameter("brightness", brightness.clone());
        }
        if let Some(contrast) = self.base.parameters.get_parameter("contrast") {
            new_effect
                .base
                .parameters
                .set_parameter("contrast", contrast.clone());
        }
        Box::new(new_effect)
    }
}

impl Filter for BrightnessContrastEffect {
    fn filter_type(&self) -> FilterType {
        FilterType::ColorCorrection
    }
    
    fn supports_inplace(&self) -> bool {
        true
    }
}

/// Saturation adjustment effect
pub struct SaturationEffect {
    base: BaseEffect,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct SaturationParams {
    saturation: f32,
    _padding: [f32; 3],
}

impl SaturationEffect {
    pub fn new() -> Self {
        let mut base = BaseEffect::new("Saturation", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("saturation", ParameterValue::Float(1.0));
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
        }
    }
}

impl Effect for SaturationEffect {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        let mut params = HashMap::new();
        params.insert(
            "saturation".to_string(),
            self.base
                .parameters
                .get_parameter("saturation")
                .cloned()
                .unwrap_or(ParameterValue::Float(1.0)),
        );
        params
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match name {
            "saturation" => {
                let saturation = value.as_float()?;
                if saturation < 0.0 || saturation > 3.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Saturation must be between 0 and 3".to_string(),
                    ));
                }
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            _ => Err(crate::error::EffectError::InvalidParameter(format!(
                "Unknown parameter: {}",
                name
            ))),
        }
    }
    
    fn apply(&mut self, frame: &Frame, time: f64) -> Result<Frame> {
        let saturation = self
            .base
            .parameters
            .get_parameter_at_time("saturation", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(1.0);
        
        let mut output = frame.clone();
        
        match frame.format {
            PixelFormat::Rgba8 => {
                for i in (0..frame.data.len()).step_by(4) {
                    let r = frame.data[i] as f32 / 255.0;
                    let g = frame.data[i + 1] as f32 / 255.0;
                    let b = frame.data[i + 2] as f32 / 255.0;
                    
                    // Calculate grayscale using luminance weights
                    let gray = 0.299 * r + 0.587 * g + 0.114 * b;
                    
                    // Interpolate between grayscale and original color
                    let new_r = gray + (r - gray) * saturation;
                    let new_g = gray + (g - gray) * saturation;
                    let new_b = gray + (b - gray) * saturation;
                    
                    output.data[i] = (new_r.clamp(0.0, 1.0) * 255.0) as u8;
                    output.data[i + 1] = (new_g.clamp(0.0, 1.0) * 255.0) as u8;
                    output.data[i + 2] = (new_b.clamp(0.0, 1.0) * 255.0) as u8;
                    output.data[i + 3] = frame.data[i + 3];
                }
            }
            _ => {
                return Err(crate::error::EffectError::Other(anyhow::anyhow!(
                    "Unsupported pixel format"
                )));
            }
        }
        
        Ok(output)
    }
    
    fn prepare_gpu(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) -> Result<()> {
        if self.pipeline.is_none() {
            let shader = shaders::create_effect_shader(
                device,
                shaders::brightness_contrast::SATURATION_SHADER,
                "Saturation",
            )?;
            self.pipeline = Some(EffectPipeline::new(
                device,
                &shader,
                wgpu::TextureFormat::Rgba8UnormSrgb,
            ));
        }
        Ok(())
    }
    
    fn requires_gpu(&self) -> bool {
        self.base.requires_gpu
    }
    
    fn description(&self) -> &str {
        "Adjust color saturation"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_effect = Self::new();
        if let Some(saturation) = self.base.parameters.get_parameter("saturation") {
            new_effect
                .base
                .parameters
                .set_parameter("saturation", saturation.clone());
        }
        Box::new(new_effect)
    }
}

impl Filter for SaturationEffect {
    fn filter_type(&self) -> FilterType {
        FilterType::ColorCorrection
    }
    
    fn supports_inplace(&self) -> bool {
        true
    }
}

impl Keyframable for BrightnessContrastEffect {
    fn add_keyframe(&mut self, time: f64, parameter: &str, value: ParameterValue) -> Result<()> {
        use crate::parameters::{Keyframe, InterpolationType};
        
        self.base.parameters.add_keyframe(
            parameter,
            Keyframe {
                time,
                value,
                interpolation: InterpolationType::Linear,
            },
        );
        Ok(())
    }
    
    fn remove_keyframe(&mut self, time: f64, parameter: &str) -> Result<()> {
        self.base.parameters.remove_keyframe(parameter, time)
    }
    
    fn get_keyframes(&self, parameter: &str) -> Vec<(f64, ParameterValue)> {
        self.base.parameters.get_keyframes(parameter)
    }
    
    fn interpolate_parameter(&self, parameter: &str, time: f64) -> Result<ParameterValue> {
        self.base
            .parameters
            .get_parameter_at_time(parameter, time)
            .ok_or_else(|| {
                crate::error::EffectError::InvalidParameter(format!(
                    "Parameter '{}' not found",
                    parameter
                ))
            })
    }
}