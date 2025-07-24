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
struct FadeParams {
    progress: f32,
    _padding: [f32; 3],
}

/// Simple fade transition
pub struct FadeTransition {
    base: BaseTransition,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
}

impl FadeTransition {
    pub fn new() -> Self {
        let base = BaseTransition::new("Fade", true);
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
        }
    }
}

impl Effect for FadeTransition {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        // Fade transition has no user-configurable parameters
        &std::collections::HashMap::new()
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        // No parameters to set for basic fade
        Err(crate::error::EffectError::InvalidParameter(format!(
            "Unknown parameter: {}",
            name
        )))
    }
    
    fn apply(&mut self, frame: &Frame, _time: f64) -> Result<Frame> {
        // This method should not be called for transitions
        // Use apply_transition instead
        Ok(frame.clone())
    }
    
    fn prepare_gpu(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) -> Result<()> {
        if self.pipeline.is_none() {
            let shader = shaders::create_effect_shader(
                device,
                shaders::transitions::FADE_TRANSITION_SHADER,
                "Fade Transition",
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
        "Simple linear fade transition between two frames"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        Box::new(Self::new())
    }
}

impl Transition for FadeTransition {
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
        
        match (from_frame.format, to_frame.format) {
            (PixelFormat::Rgba8, PixelFormat::Rgba8) => {
                for i in (0..from_frame.data.len()).step_by(4) {
                    // Linear interpolation between the two frames
                    for j in 0..4 {
                        let from_val = from_frame.data[i + j] as f32;
                        let to_val = to_frame.data[i + j] as f32;
                        let interpolated = from_val + (to_val - from_val) * progress;
                        output.data[i + j] = interpolated.clamp(0.0, 255.0) as u8;
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
        TransitionType::Fade
    }
}

/// Fade to black transition
pub struct FadeToBlackTransition {
    base: BaseTransition,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
}

impl FadeToBlackTransition {
    pub fn new() -> Self {
        let base = BaseTransition::new("Fade to Black", true);
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
        }
    }
}

impl Effect for FadeToBlackTransition {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        &std::collections::HashMap::new()
    }
    
    fn set_parameter(&mut self, name: &str, _value: ParameterValue) -> Result<()> {
        Err(crate::error::EffectError::InvalidParameter(format!(
            "Unknown parameter: {}",
            name
        )))
    }
    
    fn apply(&mut self, frame: &Frame, _time: f64) -> Result<Frame> {
        Ok(frame.clone())
    }
    
    fn prepare_gpu(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) -> Result<()> {
        if self.pipeline.is_none() {
            let shader = shaders::create_effect_shader(
                device,
                shaders::transitions::FADE_TRANSITION_SHADER,
                "Fade to Black",
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
        "Fade to black transition"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        Box::new(Self::new())
    }
}

impl Transition for FadeToBlackTransition {
    fn apply_transition(
        &mut self,
        from_frame: &Frame,
        _to_frame: &Frame,
        progress: f32,
        _time: f64,
    ) -> Result<Frame> {
        let mut output = from_frame.clone();
        let alpha = 1.0 - progress;
        
        match from_frame.format {
            PixelFormat::Rgba8 => {
                for i in (0..from_frame.data.len()).step_by(4) {
                    // Fade RGB channels to black
                    for j in 0..3 {
                        let val = from_frame.data[i + j] as f32;
                        output.data[i + j] = (val * alpha) as u8;
                    }
                    // Preserve alpha channel
                    output.data[i + 3] = from_frame.data[i + 3];
                }
            }
            _ => {
                return Err(crate::error::EffectError::TransitionError(
                    "Unsupported pixel format".to_string(),
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
        TransitionType::Fade
    }
}