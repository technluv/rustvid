/// Color correction shader with HSL adjustments
pub const COLOR_CORRECTION_SHADER: &str = r#"
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

struct ColorCorrectionParams {
    hue_shift: f32,
    saturation_mult: f32,
    lightness_add: f32,
    temperature: f32, // -1 to 1, negative = cooler, positive = warmer
    tint: f32,        // -1 to 1, negative = green, positive = magenta
    _padding: vec3<f32>,
}

@group(0) @binding(2) var<uniform> params: ColorCorrectionParams;

// Convert RGB to HSL
fn rgb_to_hsl(color: vec3<f32>) -> vec3<f32> {
    let c_max = max(max(color.r, color.g), color.b);
    let c_min = min(min(color.r, color.g), color.b);
    let delta = c_max - c_min;
    
    var h: f32 = 0.0;
    var s: f32 = 0.0;
    let l: f32 = (c_max + c_min) * 0.5;
    
    if (delta > 0.0) {
        s = delta / (1.0 - abs(2.0 * l - 1.0));
        
        if (c_max == color.r) {
            h = ((color.g - color.b) / delta + select(6.0, 0.0, color.g >= color.b)) / 6.0;
        } else if (c_max == color.g) {
            h = ((color.b - color.r) / delta + 2.0) / 6.0;
        } else {
            h = ((color.r - color.g) / delta + 4.0) / 6.0;
        }
    }
    
    return vec3<f32>(h, s, l);
}

// Convert HSL to RGB
fn hsl_to_rgb(hsl: vec3<f32>) -> vec3<f32> {
    let h = hsl.x;
    let s = hsl.y;
    let l = hsl.z;
    
    if (s == 0.0) {
        return vec3<f32>(l);
    }
    
    let q = select(l * (1.0 + s), l + s - l * s, l < 0.5);
    let p = 2.0 * l - q;
    
    var rgb: vec3<f32>;
    rgb.r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    rgb.g = hue_to_rgb(p, q, h);
    rgb.b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    
    return rgb;
}

fn hue_to_rgb(p: f32, q: f32, t_in: f32) -> f32 {
    var t = t_in;
    if (t < 0.0) { t = t + 1.0; }
    if (t > 1.0) { t = t - 1.0; }
    
    if (t < 1.0 / 6.0) {
        return p + (q - p) * 6.0 * t;
    } else if (t < 1.0 / 2.0) {
        return q;
    } else if (t < 2.0 / 3.0) {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    } else {
        return p;
    }
}

// Apply temperature adjustment
fn apply_temperature(color: vec3<f32>, temperature: f32) -> vec3<f32> {
    var result = color;
    
    if (temperature != 0.0) {
        // Warm (increase red, decrease blue)
        if (temperature > 0.0) {
            result.r = result.r + temperature * 0.1;
            result.b = result.b - temperature * 0.1;
        }
        // Cool (decrease red, increase blue)
        else {
            result.r = result.r + temperature * 0.1;
            result.b = result.b - temperature * 0.1;
        }
    }
    
    return clamp(result, vec3<f32>(0.0), vec3<f32>(1.0));
}

// Apply tint adjustment
fn apply_tint(color: vec3<f32>, tint: f32) -> vec3<f32> {
    var result = color;
    
    if (tint != 0.0) {
        // Magenta (increase red and blue)
        if (tint > 0.0) {
            result.r = result.r + tint * 0.05;
            result.b = result.b + tint * 0.05;
            result.g = result.g - tint * 0.05;
        }
        // Green (increase green)
        else {
            result.g = result.g - tint * 0.05;
            result.r = result.r + tint * 0.025;
            result.b = result.b + tint * 0.025;
        }
    }
    
    return clamp(result, vec3<f32>(0.0), vec3<f32>(1.0));
}

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let color = textureSample(input_texture, input_sampler, tex_coords);
    
    // Convert to HSL
    var hsl = rgb_to_hsl(color.rgb);
    
    // Apply hue shift
    hsl.x = fract(hsl.x + params.hue_shift);
    
    // Apply saturation
    hsl.y = clamp(hsl.y * params.saturation_mult, 0.0, 1.0);
    
    // Apply lightness
    hsl.z = clamp(hsl.z + params.lightness_add, 0.0, 1.0);
    
    // Convert back to RGB
    var rgb = hsl_to_rgb(hsl);
    
    // Apply temperature
    rgb = apply_temperature(rgb, params.temperature);
    
    // Apply tint
    rgb = apply_tint(rgb, params.tint);
    
    return vec4<f32>(rgb, color.a);
}
"#;

/// Levels adjustment shader
pub const LEVELS_SHADER: &str = r#"
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

struct LevelsParams {
    input_black: f32,
    input_white: f32,
    gamma: f32,
    output_black: f32,
    output_white: f32,
    _padding: vec3<f32>,
}

@group(0) @binding(2) var<uniform> params: LevelsParams;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    var color = textureSample(input_texture, input_sampler, tex_coords);
    
    // Normalize input levels
    color = vec4<f32>(
        (color.rgb - vec3<f32>(params.input_black)) / (params.input_white - params.input_black),
        color.a
    );
    
    // Apply gamma correction
    color = vec4<f32>(
        pow(color.rgb, vec3<f32>(1.0 / params.gamma)),
        color.a
    );
    
    // Apply output levels
    color = vec4<f32>(
        color.rgb * (params.output_white - params.output_black) + vec3<f32>(params.output_black),
        color.a
    );
    
    // Clamp to valid range
    color = clamp(color, vec4<f32>(0.0), vec4<f32>(1.0));
    
    return color;
}
"#;