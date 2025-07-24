use crate::{
    effects::BaseEffect,
    error::Result,
    gpu::{EffectPipeline, GpuBuffer},
    parameters::ParameterValue,
    shaders,
    traits::*,
};
use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct ColorCorrectionParams {
    hue_shift: f32,
    saturation_mult: f32,
    lightness_add: f32,
    temperature: f32,
    tint: f32,
    _padding: [f32; 3],
}

/// Color correction effect with HSL adjustments
pub struct ColorCorrectionEffect {
    base: BaseEffect,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
}

impl ColorCorrectionEffect {
    pub fn new() -> Self {
        let mut base = BaseEffect::new("Color Correction", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("hue_shift", ParameterValue::Float(0.0));
        base.parameters
            .set_parameter("saturation", ParameterValue::Float(1.0));
        base.parameters
            .set_parameter("lightness", ParameterValue::Float(0.0));
        base.parameters
            .set_parameter("temperature", ParameterValue::Float(0.0));
        base.parameters
            .set_parameter("tint", ParameterValue::Float(0.0));
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
        }
    }
    
    fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        
        let l = (max + min) / 2.0;
        
        if delta == 0.0 {
            return (0.0, 0.0, l);
        }
        
        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };
        
        let h = if max == r {
            ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) / 6.0
        } else if max == g {
            ((b - r) / delta + 2.0) / 6.0
        } else {
            ((r - g) / delta + 4.0) / 6.0
        };
        
        (h, s, l)
    }
    
    fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
        if s == 0.0 {
            return (l, l, l);
        }
        
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        
        let p = 2.0 * l - q;
        
        let hue_to_rgb = |p: f32, q: f32, mut t: f32| {
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            
            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 1.0 / 2.0 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        };
        
        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);
        
        (r, g, b)
    }
}

impl Effect for ColorCorrectionEffect {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        let mut params = HashMap::new();
        params.insert(
            "hue_shift".to_string(),
            self.base
                .parameters
                .get_parameter("hue_shift")
                .cloned()
                .unwrap_or(ParameterValue::Float(0.0)),
        );
        params.insert(
            "saturation".to_string(),
            self.base
                .parameters
                .get_parameter("saturation")
                .cloned()
                .unwrap_or(ParameterValue::Float(1.0)),
        );
        params.insert(
            "lightness".to_string(),
            self.base
                .parameters
                .get_parameter("lightness")
                .cloned()
                .unwrap_or(ParameterValue::Float(0.0)),
        );
        params.insert(
            "temperature".to_string(),
            self.base
                .parameters
                .get_parameter("temperature")
                .cloned()
                .unwrap_or(ParameterValue::Float(0.0)),
        );
        params.insert(
            "tint".to_string(),
            self.base
                .parameters
                .get_parameter("tint")
                .cloned()
                .unwrap_or(ParameterValue::Float(0.0)),
        );
        params
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match name {
            "hue_shift" => {
                let hue = value.as_float()?;
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            "saturation" => {
                let sat = value.as_float()?;
                if sat < 0.0 || sat > 3.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Saturation must be between 0 and 3".to_string(),
                    ));
                }
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            "lightness" => {
                let light = value.as_float()?;
                if light < -1.0 || light > 1.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Lightness must be between -1 and 1".to_string(),
                    ));
                }
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            "temperature" => {
                let temp = value.as_float()?;
                if temp < -1.0 || temp > 1.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Temperature must be between -1 and 1".to_string(),
                    ));
                }
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            "tint" => {
                let tint = value.as_float()?;
                if tint < -1.0 || tint > 1.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Tint must be between -1 and 1".to_string(),
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
        let hue_shift = self
            .base
            .parameters
            .get_parameter_at_time("hue_shift", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(0.0);
        
        let saturation = self
            .base
            .parameters
            .get_parameter_at_time("saturation", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(1.0);
        
        let lightness = self
            .base
            .parameters
            .get_parameter_at_time("lightness", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(0.0);
        
        let temperature = self
            .base
            .parameters
            .get_parameter_at_time("temperature", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(0.0);
        
        let tint = self
            .base
            .parameters
            .get_parameter_at_time("tint", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(0.0);
        
        let mut output = frame.clone();
        
        match frame.format {
            PixelFormat::Rgba8 => {
                for i in (0..frame.data.len()).step_by(4) {
                    let mut r = frame.data[i] as f32 / 255.0;
                    let mut g = frame.data[i + 1] as f32 / 255.0;
                    let mut b = frame.data[i + 2] as f32 / 255.0;
                    
                    // Convert to HSL
                    let (mut h, mut s, mut l) = Self::rgb_to_hsl(r, g, b);
                    
                    // Apply hue shift
                    h = (h + hue_shift).fract();
                    if h < 0.0 {
                        h += 1.0;
                    }
                    
                    // Apply saturation
                    s = (s * saturation).clamp(0.0, 1.0);
                    
                    // Apply lightness
                    l = (l + lightness).clamp(0.0, 1.0);
                    
                    // Convert back to RGB
                    let (mut r, mut g, mut b) = Self::hsl_to_rgb(h, s, l);
                    
                    // Apply temperature (warm/cool)
                    if temperature != 0.0 {
                        r = (r + temperature * 0.1).clamp(0.0, 1.0);
                        b = (b - temperature * 0.1).clamp(0.0, 1.0);
                    }
                    
                    // Apply tint (green/magenta)
                    if tint != 0.0 {
                        if tint > 0.0 {
                            // Magenta tint
                            r = (r + tint * 0.05).clamp(0.0, 1.0);
                            b = (b + tint * 0.05).clamp(0.0, 1.0);
                            g = (g - tint * 0.05).clamp(0.0, 1.0);
                        } else {
                            // Green tint
                            g = (g - tint * 0.05).clamp(0.0, 1.0);
                            r = (r + tint * 0.025).clamp(0.0, 1.0);
                            b = (b + tint * 0.025).clamp(0.0, 1.0);
                        }
                    }
                    
                    output.data[i] = (r * 255.0) as u8;
                    output.data[i + 1] = (g * 255.0) as u8;
                    output.data[i + 2] = (b * 255.0) as u8;
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
                shaders::color_correction::COLOR_CORRECTION_SHADER,
                "Color Correction",
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
        "Complete color correction with HSL adjustments, temperature, and tint"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_effect = Self::new();
        for (param, value) in &self.parameters() {
            let _ = new_effect.set_parameter(param, value.clone());
        }
        Box::new(new_effect)
    }
}

impl Filter for ColorCorrectionEffect {
    fn filter_type(&self) -> FilterType {
        FilterType::ColorCorrection
    }
    
    fn supports_inplace(&self) -> bool {
        true
    }
}

/// Levels adjustment effect
pub struct LevelsEffect {
    base: BaseEffect,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct LevelsParams {
    input_black: f32,
    input_white: f32,
    gamma: f32,
    output_black: f32,
    output_white: f32,
    _padding: [f32; 3],
}

impl LevelsEffect {
    pub fn new() -> Self {
        let mut base = BaseEffect::new("Levels", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("input_black", ParameterValue::Float(0.0));
        base.parameters
            .set_parameter("input_white", ParameterValue::Float(1.0));
        base.parameters
            .set_parameter("gamma", ParameterValue::Float(1.0));
        base.parameters
            .set_parameter("output_black", ParameterValue::Float(0.0));
        base.parameters
            .set_parameter("output_white", ParameterValue::Float(1.0));
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
        }
    }
}

impl Effect for LevelsEffect {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        let mut params = HashMap::new();
        for param in ["input_black", "input_white", "gamma", "output_black", "output_white"] {
            if let Some(value) = self.base.parameters.get_parameter(param) {
                params.insert(param.to_string(), value.clone());
            }
        }
        params
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        let val = value.as_float()?;
        
        match name {
            "input_black" | "input_white" | "output_black" | "output_white" => {
                if val < 0.0 || val > 1.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        format!("{} must be between 0 and 1", name),
                    ));
                }
            }
            "gamma" => {
                if val <= 0.0 || val > 5.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Gamma must be between 0 and 5".to_string(),
                    ));
                }
            }
            _ => {
                return Err(crate::error::EffectError::InvalidParameter(format!(
                    "Unknown parameter: {}",
                    name
                )));
            }
        }
        
        self.base.parameters.set_parameter(name, value);
        Ok(())
    }
    
    fn apply(&mut self, frame: &Frame, time: f64) -> Result<Frame> {
        let input_black = self
            .base
            .parameters
            .get_parameter_at_time("input_black", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(0.0);
        
        let input_white = self
            .base
            .parameters
            .get_parameter_at_time("input_white", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(1.0);
        
        let gamma = self
            .base
            .parameters
            .get_parameter_at_time("gamma", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(1.0);
        
        let output_black = self
            .base
            .parameters
            .get_parameter_at_time("output_black", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(0.0);
        
        let output_white = self
            .base
            .parameters
            .get_parameter_at_time("output_white", time)
            .and_then(|v| v.as_float().ok())
            .unwrap_or(1.0);
        
        let mut output = frame.clone();
        let inv_gamma = 1.0 / gamma;
        let input_range = input_white - input_black;
        let output_range = output_white - output_black;
        
        match frame.format {
            PixelFormat::Rgba8 => {
                for i in (0..frame.data.len()).step_by(4) {
                    // Process RGB channels
                    for j in 0..3 {
                        let value = frame.data[i + j] as f32 / 255.0;
                        
                        // Normalize input levels
                        let normalized = if input_range > 0.0 {
                            ((value - input_black) / input_range).clamp(0.0, 1.0)
                        } else {
                            0.0
                        };
                        
                        // Apply gamma correction
                        let gamma_corrected = normalized.powf(inv_gamma);
                        
                        // Apply output levels
                        let final_value = gamma_corrected * output_range + output_black;
                        
                        output.data[i + j] = (final_value.clamp(0.0, 1.0) * 255.0) as u8;
                    }
                    // Preserve alpha
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
                shaders::color_correction::LEVELS_SHADER,
                "Levels",
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
        "Adjust input/output levels and gamma"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_effect = Self::new();
        for (param, value) in &self.parameters() {
            let _ = new_effect.set_parameter(param, value.clone());
        }
        Box::new(new_effect)
    }
}

impl Filter for LevelsEffect {
    fn filter_type(&self) -> FilterType {
        FilterType::ColorCorrection
    }
    
    fn supports_inplace(&self) -> bool {
        true
    }
}