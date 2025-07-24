# Effects Crate

A comprehensive video effects system for the Rust Video Editor, providing GPU-accelerated effects, transitions, and real-time processing capabilities.

## Features

### 🎨 Effects System
- **GPU-Accelerated Processing**: Uses wgpu for high-performance GPU effects
- **Effect Pipeline**: Chain multiple effects with blend modes and opacity control
- **Real-time Preview**: Low-latency preview system for interactive editing
- **Keyframe Animation**: Animate effect parameters over time with interpolation
- **CPU Fallback**: Automatic fallback to CPU processing when GPU is unavailable

### 📊 Built-in Effects

#### Blur Effects
- **Box Blur**: Fast, simple blur effect
- **Gaussian Blur**: High-quality blur with customizable radius and sigma

#### Color Correction
- **Brightness/Contrast**: Adjust image brightness and contrast
- **Saturation**: Control color saturation
- **Color Correction**: Complete HSL adjustments with temperature and tint
- **Levels**: Input/output levels with gamma correction

### 🔄 Transitions
- **Fade**: Simple linear fade between frames
- **Dissolve**: Noise-based dissolve with optional soft edges
- **Wipe**: Directional wipe transitions (left, right, up, down)
- **Circular Wipe**: Expanding circular transition

### ⚡ Performance Features
- **Batch Processing**: Process multiple frames in parallel
- **Pipeline Optimization**: Efficient effect chaining
- **Memory Management**: Smart resource allocation
- **Benchmarking**: Comprehensive performance testing

## Quick Start

```rust
use effects::{
    filters::FilterFactory,
    gpu::GpuContext,
    pipeline::EffectPipeline,
    traits::*,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize GPU context
    let gpu_context = Arc::new(GpuContext::new().await?);
    
    // Create effect pipeline
    let mut pipeline = EffectPipeline::with_gpu_context(gpu_context).await?;
    
    // Add effects
    let blur = FilterFactory::create_blur(5.0)?;
    pipeline.add_effect(blur);
    
    let color_correction = FilterFactory::create_color_correction(
        0.1,  // hue shift
        1.2,  // saturation
        0.0,  // lightness
        0.2,  // temperature
        -0.1, // tint
    )?;
    pipeline.add_effect(color_correction);
    
    // Create a frame
    let frame = Frame {
        width: 1920,
        height: 1080,
        data: vec![128; 1920 * 1080 * 4],
        format: PixelFormat::Rgba8,
        timestamp: 0.0,
    };
    
    // Process the frame
    let processed = pipeline.process_frame(&frame, 0.0).await?;
    
    Ok(())
}
```

## Architecture

### Core Traits

- **`Effect`**: Base trait for all video effects
- **`Filter`**: Trait for single-input effects (blur, color correction, etc.)
- **`Transition`**: Trait for two-input effects (fades, dissolves, etc.)
- **`Keyframable`**: Trait for effects that support keyframe animation
- **`Previewable`**: Trait for effects that support real-time preview

### GPU Integration

The effects system uses wgpu for GPU acceleration:

- **WGSL Shaders**: Modern shader language for cross-platform compatibility
- **Compute Shaders**: For complex processing operations
- **Texture Management**: Efficient GPU memory usage
- **Pipeline Optimization**: Minimize GPU state changes

### Effect Pipeline

```
Input Frame → Effect 1 → Effect 2 → Effect 3 → Output Frame
              ↓         ↓         ↓
           Blend Mode  Opacity  Parameters
```

- **Sequential Processing**: Effects are applied in order
- **Blend Modes**: Normal, Add, Multiply, Screen, Overlay
- **Opacity Control**: Per-effect opacity from 0.0 to 1.0
- **Parameter Animation**: Keyframe-based parameter changes

## Examples

### Basic Effect Usage

```rust
// Create a blur effect
let mut blur = FilterFactory::create_blur(10.0)?;

// Apply to a frame
let processed = blur.apply(&frame, 0.0)?;
```

### Keyframe Animation

```rust
// Create brightness/contrast effect
let mut bc = FilterFactory::create_brightness_contrast(0.0, 1.0)?;

// Add keyframes for brightness animation
bc.add_keyframe(0.0, "brightness", ParameterValue::Float(0.0))?;
bc.add_keyframe(2.0, "brightness", ParameterValue::Float(0.5))?;

// Process at different times
let frame_at_1s = bc.apply(&frame, 1.0)?; // Interpolated brightness
```

### Transitions

```rust
use effects::transitions::fade::FadeTransition;

let mut fade = FadeTransition::new();
fade.set_duration(1.5);

let result = fade.apply_transition(&frame1, &frame2, 0.5, 0.75)?;
```

### Batch Processing

```rust
let frames = vec![frame1, frame2, frame3];
let results = pipeline.process_frames_batch(&frames, 0.0, 1.0/30.0).await?;
```

## Shader Development

### WGSL Shader Structure

```wgsl
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;

struct EffectParams {
    parameter1: f32,
    parameter2: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(2) var<uniform> params: EffectParams;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let color = textureSample(input_texture, input_sampler, tex_coords);
    
    // Apply effect processing
    return color;
}
```

### Adding Custom Effects

1. **Implement the Effect Trait**:
```rust
pub struct MyCustomEffect {
    base: BaseEffect,
    // ... custom fields
}

impl Effect for MyCustomEffect {
    // Implement required methods
}
```

2. **Create WGSL Shader**:
```rust
pub const MY_EFFECT_SHADER: &str = r#"
    // Your WGSL shader code here
"#;
```

3. **Register with Factory**:
```rust
registry.register("my_effect", || Box::new(MyCustomEffect::new()));
```

## Performance

### Benchmarks

Run benchmarks with:
```bash
cargo bench --package effects
```

### Optimization Tips

1. **Use GPU acceleration** for complex effects
2. **Batch process frames** when possible
3. **Reuse effect instances** to avoid recreation overhead
4. **Use preview mode** for real-time interaction
5. **Profile with criterion** to identify bottlenecks

### Performance Characteristics

- **Box Blur (1920×1080)**: ~2ms CPU, ~0.5ms GPU
- **Gaussian Blur (1920×1080)**: ~8ms CPU, ~1ms GPU
- **Color Correction (1920×1080)**: ~3ms CPU, ~0.3ms GPU
- **Fade Transition (1920×1080)**: ~1ms CPU, ~0.2ms GPU

## Testing

Run tests with:
```bash
cargo test --package effects
```

### Test Coverage

- ✅ Effect creation and parameter validation
- ✅ Pipeline processing and blend modes
- ✅ Keyframe animation and interpolation
- ✅ Transition effects
- ✅ Batch processing
- ✅ GPU context initialization
- ✅ Error handling

## Dependencies

- **wgpu**: GPU acceleration and shader execution
- **glam**: Mathematics library for vectors and matrices
- **image**: Image processing utilities
- **tokio**: Async runtime for parallel processing
- **bytemuck**: Safe casting for GPU buffers
- **serde**: Serialization for effect parameters

## Platform Support

- ✅ **Windows**: Direct3D 12, Vulkan
- ✅ **macOS**: Metal, Vulkan (via MoltenVK)
- ✅ **Linux**: Vulkan, OpenGL ES
- ✅ **Web**: WebGPU (experimental)

## Integration

The effects crate is designed to integrate with the video engine:

```rust
// Video engine integration
impl VideoEngine {
    pub fn apply_effects(&mut self, effects_pipeline: &mut EffectPipeline) {
        // Process frames through effects pipeline
    }
}
```

## Future Enhancements

- **Audio Effects**: Extend system to support audio processing
- **3D Effects**: Support for 3D transformations and camera effects
- **Plugin System**: Dynamic loading of custom effects
- **Render Graph**: More sophisticated effect ordering and optimization
- **Hardware Decoding**: Integration with hardware video decoders

## License

This crate is part of the Rust Video Editor project and follows the same licensing terms (MIT OR Apache-2.0).