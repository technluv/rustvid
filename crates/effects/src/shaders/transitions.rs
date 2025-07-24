/// Fade transition shader
pub const FADE_TRANSITION_SHADER: &str = r#"
@group(0) @binding(0) var from_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var to_texture: texture_2d<f32>;

struct TransitionParams {
    progress: f32,
    _padding: vec3<f32>,
}

@group(0) @binding(3) var<uniform> params: TransitionParams;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let from_color = textureSample(from_texture, texture_sampler, tex_coords);
    let to_color = textureSample(to_texture, texture_sampler, tex_coords);
    
    // Simple linear interpolation
    return mix(from_color, to_color, params.progress);
}
"#;

/// Dissolve transition shader
pub const DISSOLVE_TRANSITION_SHADER: &str = r#"
@group(0) @binding(0) var from_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var to_texture: texture_2d<f32>;

struct TransitionParams {
    progress: f32,
    seed: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(3) var<uniform> params: TransitionParams;

// Simple pseudo-random function
fn random(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co.xy, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let from_color = textureSample(from_texture, texture_sampler, tex_coords);
    let to_color = textureSample(to_texture, texture_sampler, tex_coords);
    
    // Generate random threshold based on position and seed
    let threshold = random(tex_coords + vec2<f32>(params.seed));
    
    // Use step function for dissolve effect
    let mask = step(threshold, params.progress);
    
    return mix(from_color, to_color, mask);
}
"#;

/// Wipe transition shader (multiple directions)
pub const WIPE_TRANSITION_SHADER: &str = r#"
@group(0) @binding(0) var from_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var to_texture: texture_2d<f32>;

struct TransitionParams {
    progress: f32,
    direction: f32, // 0: left to right, 1: right to left, 2: top to bottom, 3: bottom to top
    softness: f32,  // Edge softness (0.0 = hard edge, 1.0 = very soft)
    _padding: f32,
}

@group(0) @binding(3) var<uniform> params: TransitionParams;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let from_color = textureSample(from_texture, texture_sampler, tex_coords);
    let to_color = textureSample(to_texture, texture_sampler, tex_coords);
    
    var edge_position: f32;
    
    // Calculate edge position based on direction
    if (params.direction < 0.5) {
        // Left to right
        edge_position = tex_coords.x;
    } else if (params.direction < 1.5) {
        // Right to left
        edge_position = 1.0 - tex_coords.x;
    } else if (params.direction < 2.5) {
        // Top to bottom
        edge_position = tex_coords.y;
    } else {
        // Bottom to top
        edge_position = 1.0 - tex_coords.y;
    }
    
    // Calculate transition with optional soft edge
    let transition = smoothstep(
        params.progress - params.softness * 0.5,
        params.progress + params.softness * 0.5,
        edge_position
    );
    
    return mix(to_color, from_color, transition);
}
"#;

/// Slide transition shader
pub const SLIDE_TRANSITION_SHADER: &str = r#"
@group(0) @binding(0) var from_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var to_texture: texture_2d<f32>;

struct TransitionParams {
    progress: f32,
    direction: vec2<f32>, // Direction vector for slide
    _padding: f32,
}

@group(0) @binding(3) var<uniform> params: TransitionParams;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    // Calculate offset positions
    let from_coords = tex_coords + params.direction * params.progress;
    let to_coords = tex_coords + params.direction * (params.progress - 1.0);
    
    // Sample with clamping
    var from_color = vec4<f32>(0.0);
    var to_color = vec4<f32>(0.0);
    
    if (from_coords.x >= 0.0 && from_coords.x <= 1.0 && 
        from_coords.y >= 0.0 && from_coords.y <= 1.0) {
        from_color = textureSample(from_texture, texture_sampler, from_coords);
    }
    
    if (to_coords.x >= 0.0 && to_coords.x <= 1.0 && 
        to_coords.y >= 0.0 && to_coords.y <= 1.0) {
        to_color = textureSample(to_texture, texture_sampler, to_coords);
    }
    
    // Composite based on which texture is visible
    if (to_color.a > 0.0) {
        return to_color;
    } else {
        return from_color;
    }
}
"#;

/// Zoom transition shader
pub const ZOOM_TRANSITION_SHADER: &str = r#"
@group(0) @binding(0) var from_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var to_texture: texture_2d<f32>;

struct TransitionParams {
    progress: f32,
    zoom_center: vec2<f32>,
    _padding: f32,
}

@group(0) @binding(3) var<uniform> params: TransitionParams;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let from_color = textureSample(from_texture, texture_sampler, tex_coords);
    
    // Calculate zoomed coordinates for the incoming frame
    let zoom_factor = 1.0 / (1.0 - params.progress * 0.8); // Zoom from 1.0 to 5.0
    let centered = tex_coords - params.zoom_center;
    let zoomed = centered * zoom_factor + params.zoom_center;
    
    var to_color = vec4<f32>(0.0);
    if (zoomed.x >= 0.0 && zoomed.x <= 1.0 && 
        zoomed.y >= 0.0 && zoomed.y <= 1.0) {
        to_color = textureSample(to_texture, texture_sampler, zoomed);
    }
    
    // Fade between the two
    return mix(from_color, to_color, params.progress);
}
"#;