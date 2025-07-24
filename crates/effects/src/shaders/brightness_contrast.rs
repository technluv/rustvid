/// Brightness and Contrast adjustment shader
pub const BRIGHTNESS_CONTRAST_SHADER: &str = r#"
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

struct BrightnessContrastParams {
    brightness: f32,
    contrast: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(2) var<uniform> params: BrightnessContrastParams;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    var color = textureSample(input_texture, input_sampler, tex_coords);
    
    // Apply brightness (additive)
    color = vec4<f32>(
        color.rgb + vec3<f32>(params.brightness),
        color.a
    );
    
    // Apply contrast (multiplicative around 0.5)
    color = vec4<f32>(
        (color.rgb - vec3<f32>(0.5)) * params.contrast + vec3<f32>(0.5),
        color.a
    );
    
    // Clamp to valid range
    color = clamp(color, vec4<f32>(0.0), vec4<f32>(1.0));
    
    return color;
}
"#;

/// Saturation adjustment shader
pub const SATURATION_SHADER: &str = r#"
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

struct SaturationParams {
    saturation: f32,
    _padding: vec3<f32>,
}

@group(0) @binding(2) var<uniform> params: SaturationParams;

// Convert RGB to grayscale using luminance weights
fn rgb_to_gray(color: vec3<f32>) -> f32 {
    return dot(color, vec3<f32>(0.299, 0.587, 0.114));
}

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let color = textureSample(input_texture, input_sampler, tex_coords);
    
    // Calculate grayscale version
    let gray = rgb_to_gray(color.rgb);
    
    // Interpolate between grayscale and original color based on saturation
    let saturated = mix(vec3<f32>(gray), color.rgb, params.saturation);
    
    return vec4<f32>(saturated, color.a);
}
"#;

/// Combined brightness, contrast, and saturation shader
pub const BRIGHTNESS_CONTRAST_SATURATION_SHADER: &str = r#"
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

struct ColorAdjustmentParams {
    brightness: f32,
    contrast: f32,
    saturation: f32,
    _padding: f32,
}

@group(0) @binding(2) var<uniform> params: ColorAdjustmentParams;

fn rgb_to_gray(color: vec3<f32>) -> f32 {
    return dot(color, vec3<f32>(0.299, 0.587, 0.114));
}

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    var color = textureSample(input_texture, input_sampler, tex_coords);
    
    // Apply saturation
    let gray = rgb_to_gray(color.rgb);
    color = vec4<f32>(
        mix(vec3<f32>(gray), color.rgb, params.saturation),
        color.a
    );
    
    // Apply brightness
    color = vec4<f32>(
        color.rgb + vec3<f32>(params.brightness),
        color.a
    );
    
    // Apply contrast
    color = vec4<f32>(
        (color.rgb - vec3<f32>(0.5)) * params.contrast + vec3<f32>(0.5),
        color.a
    );
    
    // Clamp to valid range
    color = clamp(color, vec4<f32>(0.0), vec4<f32>(1.0));
    
    return color;
}
"#;