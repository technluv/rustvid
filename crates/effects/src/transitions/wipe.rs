use crate::{
    error::Result,
    gpu::{EffectPipeline, GpuBuffer},
    parameters::ParameterValue,
    shaders,
    traits::*,
    transitions::BaseTransition,
};
use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct WipeParams {
    progress: f32,
    direction: f32, // 0: left to right, 1: right to left, 2: top to bottom, 3: bottom to top
    softness: f32,
    _padding: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum WipeDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl WipeDirection {
    fn to_float(&self) -> f32 {
        match self {
            WipeDirection::LeftToRight => 0.0,
            WipeDirection::RightToLeft => 1.0,
            WipeDirection::TopToBottom => 2.0,
            WipeDirection::BottomToTop => 3.0,
        }
    }
}

/// Wipe transition
pub struct WipeTransition {
    base: BaseTransition,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
    direction: WipeDirection,
    softness: f32,
}

impl WipeTransition {
    pub fn new(direction: WipeDirection) -> Self {
        let mut base = BaseTransition::new("Wipe", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("softness", ParameterValue::Float(0.0));
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
            direction,
            softness: 0.0,
        }
    }
    
    pub fn new_left() -> Self {
        Self::new(WipeDirection::LeftToRight)
    }
    
    pub fn new_right() -> Self {
        Self::new(WipeDirection::RightToLeft)
    }
    
    pub fn new_up() -> Self {
        Self::new(WipeDirection::TopToBottom)
    }
    
    pub fn new_down() -> Self {
        Self::new(WipeDirection::BottomToTop)
    }
    
    fn smoothstep(&self, edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

impl Effect for WipeTransition {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        let mut params = HashMap::new();
        params.insert(
            "softness".to_string(),
            self.base
                .parameters
                .get_parameter("softness")
                .cloned()
                .unwrap_or(ParameterValue::Float(0.0)),
        );
        params
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match name {
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
        // This method should not be called for transitions
        Ok(frame.clone())
    }
    
    fn prepare_gpu(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue) -> Result<()> {
        if self.pipeline.is_none() {
            let shader = shaders::create_effect_shader(
                device,
                shaders::transitions::WIPE_TRANSITION_SHADER,
                "Wipe Transition",
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
        "Directional wipe transition with optional soft edge"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_transition = Self::new(self.direction);
        new_transition.softness = self.softness;
        if let Some(softness) = self.base.parameters.get_parameter("softness") {
            new_transition
                .base
                .parameters
                .set_parameter("softness", softness.clone());
        }
        Box::new(new_transition)
    }
}

impl Transition for WipeTransition {
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
                        
                        // Calculate edge position based on direction
                        let edge_position = match self.direction {
                            WipeDirection::LeftToRight => x as f32 / width,
                            WipeDirection::RightToLeft => 1.0 - (x as f32 / width),
                            WipeDirection::TopToBottom => y as f32 / height,
                            WipeDirection::BottomToTop => 1.0 - (y as f32 / height),
                        };
                        
                        // Calculate transition factor with optional soft edge
                        let transition_factor = if self.softness > 0.0 {
                            1.0 - self.smoothstep(
                                progress - half_softness,
                                progress + half_softness,
                                edge_position,
                            )
                        } else {
                            // Hard edge
                            if edge_position < progress {
                                0.0
                            } else {
                                1.0
                            }
                        };
                        
                        // Blend between frames
                        for j in 0..4 {
                            let from_val = from_frame.data[pixel_index + j] as f32;
                            let to_val = to_frame.data[pixel_index + j] as f32;
                            let result = to_val + (from_val - to_val) * transition_factor;
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
        TransitionType::Wipe
    }
}

/// Circular wipe transition
pub struct CircularWipeTransition {
    base: BaseTransition,
    pipeline: Option<EffectPipeline>,
    params_buffer: Option<GpuBuffer>,
    center: Vec2,
    softness: f32,
}

impl CircularWipeTransition {
    pub fn new() -> Self {
        let mut base = BaseTransition::new("Circular Wipe", true);
        
        // Set default parameters
        base.parameters
            .set_parameter("center_x", ParameterValue::Float(0.5));
        base.parameters
            .set_parameter("center_y", ParameterValue::Float(0.5));
        base.parameters
            .set_parameter("softness", ParameterValue::Float(0.1));
        
        Self {
            base,
            pipeline: None,
            params_buffer: None,
            center: Vec2::new(0.5, 0.5),
            softness: 0.1,
        }
    }
    
    fn smoothstep(&self, edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

impl Effect for CircularWipeTransition {
    fn id(&self) -> Uuid {
        self.base.id
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn parameters(&self) -> &HashMap<String, ParameterValue> {
        let mut params = HashMap::new();
        params.insert("center_x".to_string(), ParameterValue::Float(self.center.x));
        params.insert("center_y".to_string(), ParameterValue::Float(self.center.y));
        params.insert("softness".to_string(), ParameterValue::Float(self.softness));
        params
    }
    
    fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
        match name {
            "center_x" => {
                let x = value.as_float()?;
                if x < 0.0 || x > 1.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Center X must be between 0 and 1".to_string(),
                    ));
                }
                self.center.x = x;
                self.base.parameters.set_parameter(name, value);
                Ok(())
            }
            "center_y" => {
                let y = value.as_float()?;
                if y < 0.0 || y > 1.0 {
                    return Err(crate::error::EffectError::InvalidParameter(
                        "Center Y must be between 0 and 1".to_string(),
                    ));
                }
                self.center.y = y;
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
                shaders::transitions::WIPE_TRANSITION_SHADER,
                "Circular Wipe Transition",
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
        "Circular wipe transition expanding from center"
    }
    
    fn clone_effect(&self) -> Box<dyn Effect> {
        let mut new_transition = Self::new();
        new_transition.center = self.center;
        new_transition.softness = self.softness;
        for (param, value) in &self.parameters() {
            let _ = new_transition.set_parameter(param, value.clone());
        }
        Box::new(new_transition)
    }
}

impl Transition for CircularWipeTransition {
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
        let max_radius = ((width * width + height * height).sqrt()) / 2.0;
        let current_radius = progress * max_radius;
        let half_softness = self.softness * max_radius * 0.5;
        
        match (from_frame.format, to_frame.format) {
            (PixelFormat::Rgba8, PixelFormat::Rgba8) => {
                for y in 0..from_frame.height {
                    for x in 0..from_frame.width {
                        let pixel_index = ((y * from_frame.width + x) * 4) as usize;
                        
                        // Calculate distance from center
                        let px = x as f32 / width;
                        let py = y as f32 / height;
                        let dx = px - self.center.x;
                        let dy = py - self.center.y;
                        let distance = (dx * dx + dy * dy).sqrt() * max_radius;
                        
                        // Calculate transition factor with soft edge
                        let transition_factor = if self.softness > 0.0 {
                            1.0 - self.smoothstep(
                                current_radius - half_softness,
                                current_radius + half_softness,
                                distance,
                            )
                        } else {
                            // Hard edge
                            if distance < current_radius {
                                0.0
                            } else {
                                1.0
                            }
                        };
                        
                        // Blend between frames
                        for j in 0..4 {
                            let from_val = from_frame.data[pixel_index + j] as f32;
                            let to_val = to_frame.data[pixel_index + j] as f32;
                            let result = to_val + (from_val - to_val) * transition_factor;
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
        TransitionType::Wipe
    }
}