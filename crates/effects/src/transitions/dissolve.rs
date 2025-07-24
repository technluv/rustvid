use crate::{
    error::Result,
    gpu::{EffectPipeline, GpuBuffer},
    parameters::ParameterValue,
    shaders,
    traits::*,
    transitions::BaseTransition,
};
use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct DissolveParams {
    progress: f32,
    seed: f32,
    _padding: [f32; 2],
}

/// Dissolve transition using noise-based threshold
pub struct DissolveTransition {
    base: BaseTransition,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
    seed: f32,
}

impl DissolveTransition {
    pub fn new() -> Self {
        let mut base = BaseTransition::new("Dissolve", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("seed", ParameterValue::Float(42.0));
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
            seed: 42.0,
        }
    }
    
    /// Simple pseudo-random function for CPU fallback
    fn random(&self, x: f32, y: f32) -> f32 {
        let a = 12.9898;
        let b = 78.233;
        let c = 43758.5453;
        let dt = a * x + b * y + self.seed;
        let sn = dt % std::f32::consts::PI;
        (sn.sin() * c).fract()
    }
}

impl Effect for DissolveTransition {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        let mut params = HashMap::new();
        params.insert(
            "seed".to_string(),
            self.base
                .parameters
                .get_parameter("seed")
                .cloned()
                .unwrap_or(ParameterValue::Float(42.0)),
        );
        params
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match name {
            "seed" => {
                let seed = value.as_float()?;
                self.seed = seed;
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
        // This method should not be called for transitions
        Ok(frame.clone())
    }
    
    fn prepare_gpu(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) -> Result<()> {
        if self.pipeline.is_none() {
            let shader = shaders::create_effect_shader(
                device,
                shaders::transitions::DISSOLVE_TRANSITION_SHADER,
                "Dissolve Transition",
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
        "Dissolve transition using noise-based threshold"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_transition = Self::new();
        new_transition.seed = self.seed;
        if let Some(seed) = self.base.parameters.get_parameter("seed") {
            new_transition
                .base
                .parameters
                .set_parameter("seed", seed.clone());
        }
        Box::new(new_transition)
    }
}

impl Transition for DissolveTransition {
    fn apply_transition(
        &mut self,
        from_frame: &Frame,
        to_frame: &Frame,
        progress: f32,
        _time: f64,
    ) -> Result<Frame> {
        if from_frame.width != to_frame.width || from_frame.height != to_frame.height {
            return Err(crate::error::EffectError::TransitionError(
                "Frame dimensions must match".to_string(),
            ));
        }
        
        let mut output = from_frame.clone();
        let width = from_frame.width as f32;
        let height = from_frame.height as f32;
        
        match (from_frame.format, to_frame.format) {
            (PixelFormat::Rgba8, PixelFormat::Rgba8) => {
                for y in 0..from_frame.height {
                    for x in 0..from_frame.width {
                        let pixel_index = ((y * from_frame.width + x) * 4) as usize;
                        
                        // Generate noise threshold for this pixel
                        let noise_x = x as f32 / width;
                        let noise_y = y as f32 / height;
                        let threshold = self.random(noise_x, noise_y);
                        
                        // Use step function to create dissolve effect
                        let use_to_frame = if progress > threshold { 1.0 } else { 0.0 };
                        
                        // Blend between frames based on threshold
                        for j in 0..4 {
                            let from_val = from_frame.data[pixel_index + j] as f32;
                            let to_val = to_frame.data[pixel_index + j] as f32;
                            let result = from_val + (to_val - from_val) * use_to_frame;
                            output.data[pixel_index + j] = result as u8;
                        }
                    }
                }
            }
            _ => {
                return Err(crate::error::EffectError::TransitionError(
                    "Unsupported pixel format for transition".to_string(),
                ));
            }
        }
        
        Ok(output)
    }
    
    fn duration(&self) -> f64 {
        self.base.duration
    }
    
    fn set_duration(&mut self, duration: f64) {
        self.base.duration = duration.max(0.0);
    }
    
    fn transition_type(&self) -> TransitionType {
        TransitionType::Dissolve
    }
}

/// Advanced dissolve with edge softness
pub struct SoftDissolveTransition {
    base: BaseTransition,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
    seed: f32,
    softness: f32,
}

impl SoftDissolveTransition {
    pub fn new() -> Self {
        let mut base = BaseTransition::new("Soft Dissolve", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("seed", ParameterValue::Float(42.0));
        base.parameters
            .set_parameter("softness", ParameterValue::Float(0.1));
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
            seed: 42.0,
            softness: 0.1,
        }
    }
    
    fn random(&self, x: f32, y: f32) -> f32 {
        let a = 12.9898;
        let b = 78.233;
        let c = 43758.5453;
        let dt = a * x + b * y + self.seed;
        let sn = dt % std::f32::consts::PI;
        (sn.sin() * c).fract()
    }
    
    fn smoothstep(&self, edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

impl Effect for SoftDissolveTransition {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        let mut params = HashMap::new();
        params.insert(
            "seed".to_string(),
            self.base
                .parameters
                .get_parameter("seed")
                .cloned()
                .unwrap_or(ParameterValue::Float(42.0)),
        );
        params.insert(
            "softness".to_string(),
            self.base
                .parameters
                .get_parameter("softness")
                .cloned()
                .unwrap_or(ParameterValue::Float(0.1)),
        );
        params
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match name {
            "seed" => {
                let seed = value.as_float()?;
                self.seed = seed;
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            "softness" => {
                let softness = value.as_float()?;
                if softness < 0.0 || softness > 1.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Softness must be between 0 and 1".to_string(),
                    ));
                }
                self.softness = softness;
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
        Ok(frame.clone())
    }
    
    fn prepare_gpu(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) -> Result<()> {
        if self.pipeline.is_none() {
            let shader = shaders::create_effect_shader(
                device,
                shaders::transitions::DISSOLVE_TRANSITION_SHADER,
                "Soft Dissolve Transition",
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
        "Dissolve transition with soft edges"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_transition = Self::new();
        new_transition.seed = self.seed;
        new_transition.softness = self.softness;
        for (param, value) in &self.parameters() {
            let _ = new_transition.set_parameter(param, value.clone());
        }
        Box::new(new_transition)
    }
}

impl Transition for SoftDissolveTransition {
    fn apply_transition(
        &mut self,
        from_frame: &Frame,
        to_frame: &Frame,
        progress: f32,
        _time: f64,
    ) -> Result<Frame> {
        if from_frame.width != to_frame.width || from_frame.height != to_frame.height {
            return Err(crate::error::EffectError::TransitionError(
                "Frame dimensions must match".to_string(),
            ));
        }
        
        let mut output = from_frame.clone();
        let width = from_frame.width as f32;
        let height = from_frame.height as f32;
        let half_softness = self.softness * 0.5;
        
        match (from_frame.format, to_frame.format) {
            (PixelFormat::Rgba8, PixelFormat::Rgba8) => {
                for y in 0..from_frame.height {
                    for x in 0..from_frame.width {
                        let pixel_index = ((y * from_frame.width + x) * 4) as usize;
                        
                        // Generate noise threshold for this pixel
                        let noise_x = x as f32 / width;
                        let noise_y = y as f32 / height;
                        let threshold = self.random(noise_x, noise_y);
                        
                        // Use smoothstep for soft dissolve
                        let blend_factor = self.smoothstep(
                            progress - half_softness,
                            progress + half_softness,
                            threshold,
                        );
                        
                        // Blend between frames with soft edge
                        for j in 0..4 {
                            let from_val = from_frame.data[pixel_index + j] as f32;
                            let to_val = to_frame.data[pixel_index + j] as f32;
                            let result = from_val + (to_val - from_val) * blend_factor;
                            output.data[pixel_index + j] = result as u8;
                        }
                    }
                }
            }
            _ => {
                return Err(crate::error::EffectError::TransitionError(
                    "Unsupported pixel format for transition".to_string(),
                ));
            }
        }
        
        Ok(output)
    }
    
    fn duration(&self) -> f64 {
        self.base.duration
    }
    
    fn set_duration(&mut self, duration: f64) {
        self.base.duration = duration.max(0.0);
    }
    
    fn transition_type(&self) -> TransitionType {
        TransitionType::Dissolve
    }
}