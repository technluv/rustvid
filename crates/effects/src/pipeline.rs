use crate::{
    error::{EffectError, Result},
    gpu::{GpuContext, GpuTexture},
    traits::{Effect, Frame, PixelFormat},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Effect node in the pipeline
#[derive(Clone)]
pub struct EffectNode {
    pub id: Uuid,
    pub effect: Arc<RwLock<Box<dyn Effect>>>,
    pub enabled: bool,
    pub blend_mode: BlendMode,
    pub opacity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlendMode {
    Normal,
    Add,
    Multiply,
    Screen,
    Overlay,
}

impl EffectNode {
    pub fn new(effect: Box<dyn Effect>) -> Self {
        Self {
            id: Uuid::new_v4(),
            effect: Arc::new(RwLock::new(effect)),
            enabled: true,
            blend_mode: BlendMode::Normal,
            opacity: 1.0,
        }
    }
}

/// Effect pipeline for chaining multiple effects
pub struct EffectPipeline {
    effects: Vec<EffectNode>,
    gpu_context: Option<Arc<GpuContext>>,
    cache_enabled: bool,
    preview_mode: bool,
}

impl EffectPipeline {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
            gpu_context: None,
            cache_enabled: true,
            preview_mode: false,
        }
    }
    
    pub async fn with_gpu_context(gpu_context: Arc<GpuContext>) -> Result<Self> {
        Ok(Self {
            effects: Vec::new(),
            gpu_context: Some(gpu_context),
            cache_enabled: true,
            preview_mode: false,
        })
    }
    
    /// Add an effect to the pipeline
    pub fn add_effect(&mut self, effect: Box<dyn Effect>) -> Uuid {
        let node = EffectNode::new(effect);
        let id = node.id;
        self.effects.push(node);
        id
    }
    
    /// Remove an effect from the pipeline
    pub fn remove_effect(&mut self, id: Uuid) -> Result<()> {
        self.effects.retain(|node| node.id != id);
        Ok(())
    }
    
    /// Move an effect to a new position
    pub fn reorder_effect(&mut self, id: Uuid, new_index: usize) -> Result<()> {
        let current_index = self
            .effects
            .iter()
            .position(|node| node.id == id)
            .ok_or_else(|| EffectError::EffectNotFound { id })?;
        
        if new_index >= self.effects.len() {
            return Err(EffectError::PipelineError(
                "New index out of bounds".to_string(),
            ));
        }
        
        let node = self.effects.remove(current_index);
        self.effects.insert(new_index, node);
        
        Ok(())
    }
    
    /// Enable or disable an effect
    pub fn set_effect_enabled(&mut self, id: Uuid, enabled: bool) -> Result<()> {
        let node = self
            .effects
            .iter_mut()
            .find(|node| node.id == id)
            .ok_or_else(|| EffectError::EffectNotFound { id })?;
        
        node.enabled = enabled;
        Ok(())
    }
    
    /// Set effect blend mode
    pub fn set_effect_blend_mode(&mut self, id: Uuid, blend_mode: BlendMode) -> Result<()> {
        let node = self
            .effects
            .iter_mut()
            .find(|node| node.id == id)
            .ok_or_else(|| EffectError::EffectNotFound { id })?;
        
        node.blend_mode = blend_mode;
        Ok(())
    }
    
    /// Set effect opacity
    pub fn set_effect_opacity(&mut self, id: Uuid, opacity: f32) -> Result<()> {
        let node = self
            .effects
            .iter_mut()
            .find(|node| node.id == id)
            .ok_or_else(|| EffectError::EffectNotFound { id })?;
        
        node.opacity = opacity.clamp(0.0, 1.0);
        Ok(())
    }
    
    /// Process a frame through the pipeline
    pub async fn process_frame(&mut self, input: &Frame, time: f64) -> Result<Frame> {
        let mut current_frame = input.clone();
        
        // Prepare GPU resources if available
        if let Some(gpu_context) = &self.gpu_context {
            for node in &self.effects {
                if node.enabled {
                    let mut effect = node.effect.write().await;
                    if effect.requires_gpu() {
                        effect.prepare_gpu(&gpu_context.device, &gpu_context.queue)?;
                    }
                }
            }
        }
        
        // Process through each enabled effect
        for node in &self.effects {
            if !node.enabled {
                continue;
            }
            
            let mut effect = node.effect.write().await;
            let processed = effect.apply(&current_frame, time)?;
            
            // Apply blend mode and opacity
            current_frame = if node.opacity < 1.0 || node.blend_mode != BlendMode::Normal {
                blend_frames(&current_frame, &processed, node.blend_mode, node.opacity)?
            } else {
                processed
            };
        }
        
        Ok(current_frame)
    }
    
    /// Process multiple frames in parallel
    pub async fn process_frames_batch(
        &mut self,
        frames: &[Frame],
        start_time: f64,
        frame_duration: f64,
    ) -> Result<Vec<Frame>> {
        let mut tasks = Vec::new();
        
        for (i, frame) in frames.iter().enumerate() {
            let frame_clone = frame.clone();
            let time = start_time + (i as f64) * frame_duration;
            let pipeline_clone = self.clone_pipeline().await?;
            
            tasks.push(tokio::spawn(async move {
                let mut pipeline = pipeline_clone;
                pipeline.process_frame(&frame_clone, time).await
            }));
        }
        
        let mut results = Vec::new();
        for task in tasks {
            results.push(task.await.map_err(|e| {
                EffectError::PipelineError(format!("Task join error: {}", e))
            })??);
        }
        
        Ok(results)
    }
    
    /// Enable preview mode for faster processing
    pub fn set_preview_mode(&mut self, preview: bool) {
        self.preview_mode = preview;
    }
    
    /// Get all effects in the pipeline
    pub fn get_effects(&self) -> &[EffectNode] {
        &self.effects
    }
    
    /// Clear all effects
    pub fn clear(&mut self) {
        self.effects.clear();
    }
    
    /// Clone the pipeline (creates new effect instances)
    async fn clone_pipeline(&self) -> Result<Self> {
        let mut new_pipeline = Self {
            effects: Vec::new(),
            gpu_context: self.gpu_context.clone(),
            cache_enabled: self.cache_enabled,
            preview_mode: self.preview_mode,
        };
        
        for node in &self.effects {
            let effect = node.effect.read().await;
            let cloned_effect = effect.clone_effect();
            let mut new_node = EffectNode::new(cloned_effect);
            new_node.id = node.id;
            new_node.enabled = node.enabled;
            new_node.blend_mode = node.blend_mode;
            new_node.opacity = node.opacity;
            new_pipeline.effects.push(new_node);
        }
        
        Ok(new_pipeline)
    }
}

/// Blend two frames together
fn blend_frames(
    base: &Frame,
    overlay: &Frame,
    blend_mode: BlendMode,
    opacity: f32,
) -> Result<Frame> {
    if base.width != overlay.width || base.height != overlay.height {
        return Err(EffectError::PipelineError(
            "Frame dimensions must match for blending".to_string(),
        ));
    }
    
    let mut result = base.clone();
    
    match base.format {
        PixelFormat::Rgba8 => {
            for i in (0..base.data.len()).step_by(4) {
                let base_r = base.data[i] as f32 / 255.0;
                let base_g = base.data[i + 1] as f32 / 255.0;
                let base_b = base.data[i + 2] as f32 / 255.0;
                let base_a = base.data[i + 3] as f32 / 255.0;
                
                let overlay_r = overlay.data[i] as f32 / 255.0;
                let overlay_g = overlay.data[i + 1] as f32 / 255.0;
                let overlay_b = overlay.data[i + 2] as f32 / 255.0;
                let overlay_a = overlay.data[i + 3] as f32 / 255.0;
                
                let (r, g, b) = match blend_mode {
                    BlendMode::Normal => (overlay_r, overlay_g, overlay_b),
                    BlendMode::Add => (
                        (base_r + overlay_r).min(1.0),
                        (base_g + overlay_g).min(1.0),
                        (base_b + overlay_b).min(1.0),
                    ),
                    BlendMode::Multiply => (
                        base_r * overlay_r,
                        base_g * overlay_g,
                        base_b * overlay_b,
                    ),
                    BlendMode::Screen => (
                        1.0 - (1.0 - base_r) * (1.0 - overlay_r),
                        1.0 - (1.0 - base_g) * (1.0 - overlay_g),
                        1.0 - (1.0 - base_b) * (1.0 - overlay_b),
                    ),
                    BlendMode::Overlay => (
                        if base_r < 0.5 {
                            2.0 * base_r * overlay_r
                        } else {
                            1.0 - 2.0 * (1.0 - base_r) * (1.0 - overlay_r)
                        },
                        if base_g < 0.5 {
                            2.0 * base_g * overlay_g
                        } else {
                            1.0 - 2.0 * (1.0 - base_g) * (1.0 - overlay_g)
                        },
                        if base_b < 0.5 {
                            2.0 * base_b * overlay_b
                        } else {
                            1.0 - 2.0 * (1.0 - base_b) * (1.0 - overlay_b)
                        },
                    ),
                };
                
                // Apply opacity
                let final_r = base_r * (1.0 - opacity) + r * opacity;
                let final_g = base_g * (1.0 - opacity) + g * opacity;
                let final_b = base_b * (1.0 - opacity) + b * opacity;
                let final_a = base_a.max(overlay_a * opacity);
                
                result.data[i] = (final_r * 255.0) as u8;
                result.data[i + 1] = (final_g * 255.0) as u8;
                result.data[i + 2] = (final_b * 255.0) as u8;
                result.data[i + 3] = (final_a * 255.0) as u8;
            }
        }
        _ => {
            return Err(EffectError::PipelineError(
                "Unsupported pixel format for blending".to_string(),
            ));
        }
    }
    
    Ok(result)
}