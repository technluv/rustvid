use crate::{
    effects::BaseEffect,
    error::Result,
    gpu::{EffectPipeline, GpuBuffer, GpuTexture},
    parameters::ParameterValue,
    shaders,
    traits::*,
};
use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct BlurParams {
    radius: f32,
    sigma: f32,
    direction: [f32; 2],
}

/// Box blur effect
pub struct BlurEffect {
    base: BaseEffect,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
}

impl BlurEffect {
    pub fn new() -> Self {
        let mut base = BaseEffect::new("Box Blur", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("radius", ParameterValue::Float(5.0));
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
        }
    }
}

impl Effect for BlurEffect {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        std::iter::once((
            "radius".to_string(),
            self.base
                .parameters
                .get_parameter("radius")
                .cloned()
                .unwrap_or(ParameterValue::Float(5.0)),
        ))
        .collect()
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match name {
            "radius" => {
                let radius = value.as_float()?;
                if radius < 0.0 || radius > 50.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Radius must be between 0 and 50".to_string(),
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
    
    fn apply(&mut self, frame: &Frame, _time: f64) -> Result<Frame> {
        // For CPU fallback, implement simple box blur
        let radius = self
            .base
            .parameters
            .get_parameter("radius")
            .and_then(|v| v.as_float().ok())
            .unwrap_or(5.0) as i32;
        
        if radius == 0 {
            return Ok(frame.clone());
        }
        
        let mut output = frame.clone();
        let width = frame.width as i32;
        let height = frame.height as i32;
        
        // Simple box blur implementation
        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut a_sum = 0u32;
                let mut count = 0u32;
                
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let px = (x + dx).clamp(0, width - 1);
                        let py = (y + dy).clamp(0, height - 1);
                        let idx = ((py * width + px) * 4) as usize;
                        
                        r_sum += frame.data[idx] as u32;
                        g_sum += frame.data[idx + 1] as u32;
                        b_sum += frame.data[idx + 2] as u32;
                        a_sum += frame.data[idx + 3] as u32;
                        count += 1;
                    }
                }
                
                let idx = ((y * width + x) * 4) as usize;
                output.data[idx] = (r_sum / count) as u8;
                output.data[idx + 1] = (g_sum / count) as u8;
                output.data[idx + 2] = (b_sum / count) as u8;
                output.data[idx + 3] = (a_sum / count) as u8;
            }
        }
        
        Ok(output)
    }
    
    fn prepare_gpu(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) -> Result<()> {
        if self.pipeline.is_none() {
            let shader = shaders::create_effect_shader(device, shaders::blur::BOX_BLUR_SHADER, "Box Blur")?;
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
        "Simple box blur effect"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_effect = Self::new();
        if let Some(radius) = self.base.parameters.get_parameter("radius") {
            new_effect.base.parameters.set_parameter("radius", radius.clone());
        }
        Box::new(new_effect)
    }
}

/// Gaussian blur effect (higher quality)
pub struct GaussianBlurEffect {
    base: BaseEffect,
    horizontal_pipeline: Option<EffectPipeline>,
    vertical_pipeline: Option<EffectPipeline>,
    intermediate_texture: Option<GpuTexture>,
}

impl GaussianBlurEffect {
    pub fn new() -> Self {
        let mut base = BaseEffect::new("Gaussian Blur", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("radius", ParameterValue::Float(5.0));
        base.parameters
            .set_parameter("sigma", ParameterValue::Float(2.0));
        
        Self {
            base,
            horizontal_pipeline: None,
            vertical_pipeline: None,
            intermediate_texture: None,
        }
    }
}

impl Effect for GaussianBlurEffect {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        let mut params = HashMap::new();
        params.insert(
            "radius".to_string(),
            self.base
                .parameters
                .get_parameter("radius")
                .cloned()
                .unwrap_or(ParameterValue::Float(5.0)),
        );
        params.insert(
            "sigma".to_string(),
            self.base
                .parameters
                .get_parameter("sigma")
                .cloned()
                .unwrap_or(ParameterValue::Float(2.0)),
        );
        params
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match name {
            "radius" => {
                let radius = value.as_float()?;
                if radius < 0.0 || radius > 50.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Radius must be between 0 and 50".to_string(),
                    ));
                }
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            "sigma" => {
                let sigma = value.as_float()?;
                if sigma <= 0.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Sigma must be positive".to_string(),
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
    
    fn apply(&mut self, frame: &Frame, _time: f64) -> Result<Frame> {
        // CPU fallback - use simple gaussian approximation
        let radius = self
            .base
            .parameters
            .get_parameter("radius")
            .and_then(|v| v.as_float().ok())
            .unwrap_or(5.0) as i32;
        
        let sigma = self
            .base
            .parameters
            .get_parameter("sigma")
            .and_then(|v| v.as_float().ok())
            .unwrap_or(2.0);
        
        if radius == 0 {
            return Ok(frame.clone());
        }
        
        // Create gaussian kernel
        let kernel_size = (radius * 2 + 1) as usize;
        let mut kernel = vec![0.0f32; kernel_size];
        let mut sum = 0.0;
        
        for i in 0..kernel_size {
            let x = (i as i32 - radius) as f32;
            kernel[i] = (-x * x / (2.0 * sigma * sigma)).exp();
            sum += kernel[i];
        }
        
        // Normalize kernel
        for k in &mut kernel {
            *k /= sum;
        }
        
        let mut intermediate = frame.clone();
        let mut output = frame.clone();
        let width = frame.width as i32;
        let height = frame.height as i32;
        
        // Horizontal pass
        for y in 0..height {
            for x in 0..width {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                let mut a = 0.0;
                
                for (i, &weight) in kernel.iter().enumerate() {
                    let sx = (x + i as i32 - radius).clamp(0, width - 1);
                    let idx = ((y * width + sx) * 4) as usize;
                    
                    r += frame.data[idx] as f32 * weight;
                    g += frame.data[idx + 1] as f32 * weight;
                    b += frame.data[idx + 2] as f32 * weight;
                    a += frame.data[idx + 3] as f32 * weight;
                }
                
                let idx = ((y * width + x) * 4) as usize;
                intermediate.data[idx] = r as u8;
                intermediate.data[idx + 1] = g as u8;
                intermediate.data[idx + 2] = b as u8;
                intermediate.data[idx + 3] = a as u8;
            }
        }
        
        // Vertical pass
        for y in 0..height {
            for x in 0..width {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                let mut a = 0.0;
                
                for (i, &weight) in kernel.iter().enumerate() {
                    let sy = (y + i as i32 - radius).clamp(0, height - 1);
                    let idx = ((sy * width + x) * 4) as usize;
                    
                    r += intermediate.data[idx] as f32 * weight;
                    g += intermediate.data[idx + 1] as f32 * weight;
                    b += intermediate.data[idx + 2] as f32 * weight;
                    a += intermediate.data[idx + 3] as f32 * weight;
                }
                
                let idx = ((y * width + x) * 4) as usize;
                output.data[idx] = r as u8;
                output.data[idx + 1] = g as u8;
                output.data[idx + 2] = b as u8;
                output.data[idx + 3] = a as u8;
            }
        }
        
        Ok(output)
    }
    
    fn prepare_gpu(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) -> Result<()> {
        if self.horizontal_pipeline.is_none() {
            let shader = shaders::create_effect_shader(
                device,
                shaders::blur::BLUR_SHADER,
                "Gaussian Blur",
            )?;
            
            self.horizontal_pipeline = Some(EffectPipeline::new(
                device,
                &shader,
                wgpu::TextureFormat::Rgba8UnormSrgb,
            ));
            
            self.vertical_pipeline = Some(EffectPipeline::new(
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
        "High-quality gaussian blur effect"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_effect = Self::new();
        if let Some(radius) = self.base.parameters.get_parameter("radius") {
            new_effect.base.parameters.set_parameter("radius", radius.clone());
        }
        if let Some(sigma) = self.base.parameters.get_parameter("sigma") {
            new_effect.base.parameters.set_parameter("sigma", sigma.clone());
        }
        Box::new(new_effect)
    }
}

impl Filter for BlurEffect {
    fn filter_type(&self) -> FilterType {
        FilterType::Blur
    }
    
    fn supports_inplace(&self) -> bool {
        false
    }
}

impl Filter for GaussianBlurEffect {
    fn filter_type(&self) -> FilterType {
        FilterType::Blur
    }
    
    fn supports_inplace(&self) -> bool {
        false
    }
}