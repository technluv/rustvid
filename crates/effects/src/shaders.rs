pub mod blur;
pub mod brightness_contrast;
pub mod color_correction;
pub mod transitions;

use crate::error::{EffectError, Result};
use std::collections::HashMap;

/// Shader loader and manager
pub struct ShaderManager {
    shaders: HashMap<String, wgpu::ShaderModule>,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }
    
    pub fn load_shader(
        &mut self,
        device: &wgpu::Device,
        name: &str,
        source: &str,
    ) -> Result<&wgpu::ShaderModule> {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        
        self.shaders.insert(name.to_string(), module);
        
        self.shaders
            .get(name)
            .ok_or_else(|| EffectError::ShaderCompilationError("Failed to store shader".to_string()))
    }
    
    pub fn get_shader(&self, name: &str) -> Option<&wgpu::ShaderModule> {
        self.shaders.get(name)
    }
    
    pub fn load_builtin_shaders(&mut self, device: &wgpu::Device) -> Result<()> {
        // Load base vertex shader
        self.load_shader(device, "base_vertex", BASE_VERTEX_SHADER)?;
        
        // Load effect shaders
        self.load_shader(device, "passthrough", PASSTHROUGH_SHADER)?;
        self.load_shader(device, "blur", blur::BLUR_SHADER)?;
        self.load_shader(device, "brightness_contrast", brightness_contrast::BRIGHTNESS_CONTRAST_SHADER)?;
        self.load_shader(device, "color_correction", color_correction::COLOR_CORRECTION_SHADER)?;
        
        // Load transition shaders
        self.load_shader(device, "fade_transition", transitions::FADE_TRANSITION_SHADER)?;
        self.load_shader(device, "dissolve_transition", transitions::DISSOLVE_TRANSITION_SHADER)?;
        self.load_shader(device, "wipe_transition", transitions::WIPE_TRANSITION_SHADER)?;
        
        Ok(())
    }
}

/// Base vertex shader used by all effects
pub const BASE_VERTEX_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Generate fullscreen triangle
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>((x + 1.0) * 0.5, (1.0 - y) * 0.5);
    
    return out;
}
"#;

/// Simple passthrough shader
pub const PASSTHROUGH_SHADER: &str = r#"
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(input_texture, input_sampler, tex_coords);
}
"#;

/// Shader compilation utilities
pub fn compile_shader_from_source(
    device: &wgpu::Device,
    source: &str,
    label: &str,
) -> Result<wgpu::ShaderModule> {
    Ok(device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    }))
}

/// Combine vertex and fragment shaders
pub fn create_effect_shader(
    device: &wgpu::Device,
    fragment_source: &str,
    label: &str,
) -> Result<wgpu::ShaderModule> {
    let combined_source = format!("{}\n\n{}", BASE_VERTEX_SHADER, fragment_source);
    compile_shader_from_source(device, &combined_source, label)
}