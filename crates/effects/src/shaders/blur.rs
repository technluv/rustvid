/// Gaussian blur shader
pub const BLUR_SHADER: &str = r#"
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

struct BlurParams {
    radius: f32,
    sigma: f32,
    direction: vec2<f32>, // (1,0) for horizontal, (0,1) for vertical
}

@group(0) @binding(2) var<uniform> params: BlurParams;

// Gaussian weight calculation
fn gaussian_weight(offset: f32, sigma: f32) -> f32 {
    let s = sigma * sigma;
    return exp(-0.5 * offset * offset / s) / sqrt(2.0 * 3.14159265359 * s);
}

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let texture_size = vec2<f32>(textureDimensions(input_texture));
    let texel_size = 1.0 / texture_size;
    
    var color = vec4<f32>(0.0);
    var total_weight = 0.0;
    
    let radius = i32(params.radius);
    
    // Sample pixels in the blur direction
    for (var i = -radius; i <= radius; i = i + 1) {
        let offset = vec2<f32>(f32(i)) * params.direction;
        let sample_coords = tex_coords + offset * texel_size;
        
        // Ensure we're sampling within texture bounds
        if (sample_coords.x >= 0.0 && sample_coords.x <= 1.0 &&
            sample_coords.y >= 0.0 && sample_coords.y <= 1.0) {
            
            let weight = gaussian_weight(f32(i), params.sigma);
            color = color + textureSample(input_texture, input_sampler, sample_coords) * weight;
            total_weight = total_weight + weight;
        }
    }
    
    // Normalize the result
    if (total_weight > 0.0) {
        color = color / total_weight;
    }
    
    return color;
}
"#;

/// Box blur shader (faster but lower quality)
pub const BOX_BLUR_SHADER: &str = r#"
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

struct BlurParams {
    radius: f32,
    _padding: vec3<f32>,
}

@group(0) @binding(2) var<uniform> params: BlurParams;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let texture_size = vec2<f32>(textureDimensions(input_texture));
    let texel_size = 1.0 / texture_size;
    
    var color = vec4<f32>(0.0);
    var sample_count = 0;
    
    let radius = i32(params.radius);
    
    // Sample pixels in a box pattern
    for (var x = -radius; x <= radius; x = x + 1) {
        for (var y = -radius; y <= radius; y = y + 1) {
            let offset = vec2<f32>(f32(x), f32(y));
            let sample_coords = tex_coords + offset * texel_size;
            
            // Ensure we're sampling within texture bounds
            if (sample_coords.x >= 0.0 && sample_coords.x <= 1.0 &&
                sample_coords.y >= 0.0 && sample_coords.y <= 1.0) {
                
                color = color + textureSample(input_texture, input_sampler, sample_coords);
                sample_count = sample_count + 1;
            }
        }
    }
    
    // Average the samples
    if (sample_count > 0) {
        color = color / f32(sample_count);
    }
    
    return color;
}
"#;