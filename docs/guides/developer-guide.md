# Developer Guide

This guide covers everything you need to know to contribute to the Rust Video Editor project.

## Table of Contents

1. [Development Setup](#development-setup)
2. [Project Structure](#project-structure)
3. [Coding Standards](#coding-standards)
4. [Architecture Deep Dive](#architecture-deep-dive)
5. [Adding New Features](#adding-new-features)
6. [Testing](#testing)
7. [Performance Guidelines](#performance-guidelines)
8. [Debugging](#debugging)
9. [Contributing](#contributing)

## Development Setup

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Git
- FFmpeg development libraries
- Platform-specific requirements:
  - **Linux**: `sudo apt-get install libavcodec-dev libavformat-dev libavutil-dev libswscale-dev pkg-config`
  - **macOS**: `brew install ffmpeg pkg-config`
  - **Windows**: Download FFmpeg dev libraries and set `FFMPEG_DIR` environment variable

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/your-org/rust-video-editor.git
cd rust-video-editor

# Install dependencies
cargo build --all

# Run tests
cargo test --all

# Build documentation
cargo doc --all --no-deps --open
```

### Development Tools

```bash
# Install development tools
cargo install cargo-watch cargo-expand cargo-criterion

# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Watch for changes
cargo watch -x "test --all" -x "clippy --all"
```

## Project Structure

```
rust-video-editor/
├── Cargo.toml                 # Workspace configuration
├── crates/
│   ├── core/                 # Core video processing
│   │   ├── src/
│   │   │   ├── lib.rs       # Public API
│   │   │   ├── traits.rs    # Core traits
│   │   │   ├── frame.rs     # Frame implementation
│   │   │   ├── decoder/     # Decoding subsystem
│   │   │   ├── buffer/      # Buffer management
│   │   │   └── pipeline/    # Processing pipeline
│   │   └── tests/
│   ├── timeline/            # Timeline management
│   ├── effects/             # Effects system
│   ├── export/              # Export functionality
│   └── ui/                  # User interface
├── examples/                # Example applications
├── benches/                 # Performance benchmarks
└── docs/                    # Documentation
```

## Coding Standards

### Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

```rust
// Good: Use descriptive names
pub struct VideoFrame {
    width: u32,
    height: u32,
    pixel_data: Vec<u8>,
}

// Good: Document public APIs
/// Decodes a video frame from the input stream.
/// 
/// # Errors
/// 
/// Returns `VideoError::EndOfStream` when no more frames are available.
pub fn decode_frame(&mut self) -> Result<Frame, VideoError> {
    // Implementation
}

// Good: Use type aliases for clarity
type FrameBuffer = Arc<Mutex<Vec<Frame>>>;
```

### Error Handling

Use `thiserror` for error definitions:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("Invalid frame dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    
    #[error("Codec not supported: {0}")]
    UnsupportedCodec(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

### Documentation

All public APIs must be documented:

```rust
/// Applies a video effect to the given frame.
/// 
/// # Arguments
/// 
/// * `frame` - The frame to process
/// * `parameters` - Effect-specific parameters
/// 
/// # Examples
/// 
/// ```
/// use rust_video_core::{Frame, Effect};
/// 
/// let mut frame = Frame::new(1920, 1080)?;
/// let effect = BrightnessEffect::new(1.5);
/// effect.apply(&mut frame)?;
/// ```
/// 
/// # Performance
/// 
/// This operation is optimized for SIMD and will use multiple threads
/// when available.
pub fn apply(&self, frame: &mut Frame) -> Result<()> {
    // Implementation
}
```

## Architecture Deep Dive

### Core Abstractions

#### Frame System

The `Frame` struct is the fundamental unit of video data:

```rust
pub struct Frame {
    /// Raw pixel data in the specified format
    data: Vec<u8>,
    
    /// Frame metadata
    metadata: FrameMetadata,
    
    /// Buffer pool reference for memory reuse
    pool: Option<Arc<BufferPool>>,
}

impl Frame {
    /// Creates a new frame with the specified dimensions
    pub fn new(width: u32, height: u32, format: PixelFormat) -> Result<Self> {
        let size = calculate_buffer_size(width, height, format)?;
        Ok(Self {
            data: vec![0; size],
            metadata: FrameMetadata::new(width, height, format),
            pool: None,
        })
    }
    
    /// Returns frame to buffer pool if applicable
    fn drop(&mut self) {
        if let Some(pool) = &self.pool {
            pool.return_buffer(std::mem::take(&mut self.data));
        }
    }
}
```

#### Processing Pipeline

The pipeline system allows composable video processing:

```rust
pub trait Pipeline: Send + Sync {
    fn add_processor(&mut self, processor: Box<dyn VideoProcessor>);
    fn process_frame(&mut self, frame: &mut Frame) -> Result<()>;
    fn can_parallelize(&self) -> bool;
}

// Usage example
let mut pipeline = ProcessingPipeline::new();
pipeline.add_processor(Box::new(ColorCorrection::new()));
pipeline.add_processor(Box::new(Stabilization::new()));
pipeline.add_processor(Box::new(NoiseReduction::new()));

// Process frames
while let Some(mut frame) = decoder.decode_frame()? {
    pipeline.process_frame(&mut frame)?;
    encoder.encode_frame(&frame)?;
}
```

### Memory Management

#### Buffer Pool Implementation

```rust
pub struct BufferPool {
    buffers: Arc<Mutex<Vec<Vec<u8>>>>,
    size_hint: usize,
    max_buffers: usize,
}

impl BufferPool {
    pub fn acquire(&self, size: usize) -> Vec<u8> {
        let mut buffers = self.buffers.lock().unwrap();
        
        // Try to reuse existing buffer
        if let Some(mut buffer) = buffers.pop() {
            buffer.resize(size, 0);
            return buffer;
        }
        
        // Allocate new buffer if under limit
        vec![0; size]
    }
    
    pub fn return_buffer(&self, buffer: Vec<u8>) {
        let mut buffers = self.buffers.lock().unwrap();
        if buffers.len() < self.max_buffers {
            buffers.push(buffer);
        }
    }
}
```

### Async Processing

Most I/O operations are async:

```rust
pub async fn process_video(input: &str, output: &str) -> Result<()> {
    let mut decoder = FFmpegDecoder::new();
    let mut encoder = FFmpegEncoder::new();
    
    // Open streams asynchronously
    let format = decoder.open(input).await?;
    encoder.initialize(&format, output).await?;
    
    // Process in chunks for better concurrency
    let (tx, rx) = mpsc::channel(10);
    
    // Decode task
    let decode_task = tokio::spawn(async move {
        while let Some(frame) = decoder.decode_frame().await? {
            tx.send(frame).await?;
        }
        Ok::<_, VideoError>(())
    });
    
    // Encode task
    let encode_task = tokio::spawn(async move {
        while let Ok(frame) = rx.recv().await {
            encoder.encode_frame(&frame).await?;
        }
        encoder.finalize().await?;
        Ok::<_, VideoError>(())
    });
    
    // Wait for completion
    tokio::try_join!(decode_task, encode_task)?;
    Ok(())
}
```

## Adding New Features

### Example: Adding a New Video Effect

1. **Define the effect trait implementation:**

```rust
// In crates/effects/src/custom_effect.rs
use crate::{Effect, EffectError, Parameter, ParameterValue};
use rust_video_core::Frame;

pub struct VintageEffect {
    intensity: f32,
    grain_amount: f32,
    vignette: bool,
}

impl VintageEffect {
    pub fn new() -> Self {
        Self {
            intensity: 0.5,
            grain_amount: 0.3,
            vignette: true,
        }
    }
    
    fn apply_sepia(&self, frame: &mut Frame) -> Result<(), EffectError> {
        // Sepia tone implementation
        for pixel in frame.pixels_mut() {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            
            pixel[0] = ((r * 0.393 + g * 0.769 + b * 0.189) * self.intensity) as u8;
            pixel[1] = ((r * 0.349 + g * 0.686 + b * 0.168) * self.intensity) as u8;
            pixel[2] = ((r * 0.272 + g * 0.534 + b * 0.131) * self.intensity) as u8;
        }
        Ok(())
    }
}

impl Effect for VintageEffect {
    fn name(&self) -> &str {
        "Vintage"
    }
    
    fn process(&mut self, input: &mut dyn Any) -> Result<(), EffectError> {
        let frame = input.downcast_mut::<Frame>()
            .ok_or_else(|| EffectError::InvalidInput)?;
            
        self.apply_sepia(frame)?;
        
        if self.grain_amount > 0.0 {
            self.apply_grain(frame)?;
        }
        
        if self.vignette {
            self.apply_vignette(frame)?;
        }
        
        Ok(())
    }
    
    fn get_parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter {
                name: "intensity".to_string(),
                value: ParameterValue::Float(self.intensity),
                min: Some(ParameterValue::Float(0.0)),
                max: Some(ParameterValue::Float(1.0)),
            },
            Parameter {
                name: "grain_amount".to_string(),
                value: ParameterValue::Float(self.grain_amount),
                min: Some(ParameterValue::Float(0.0)),
                max: Some(ParameterValue::Float(1.0)),
            },
            Parameter {
                name: "vignette".to_string(),
                value: ParameterValue::Boolean(self.vignette),
                min: None,
                max: None,
            },
        ]
    }
}
```

2. **Write tests:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vintage_effect() {
        let mut effect = VintageEffect::new();
        let mut frame = Frame::new(100, 100, PixelFormat::RGB24).unwrap();
        
        // Fill with test data
        frame.fill_with_color(255, 255, 255);
        
        // Apply effect
        effect.process(&mut frame).unwrap();
        
        // Verify sepia was applied
        let first_pixel = &frame.pixels()[0..3];
        assert!(first_pixel[0] > first_pixel[2]); // Red channel should be higher
    }
}
```

3. **Register the effect:**

```rust
// In crates/effects/src/registry.rs
pub fn register_builtin_effects(registry: &mut EffectRegistry) {
    registry.register("brightness", || Box::new(BrightnessEffect::new()));
    registry.register("contrast", || Box::new(ContrastEffect::new()));
    registry.register("vintage", || Box::new(VintageEffect::new())); // New!
}
```

## Testing

### Unit Tests

Test individual components in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frame_creation() {
        let frame = Frame::new(1920, 1080, PixelFormat::RGB24).unwrap();
        assert_eq!(frame.width(), 1920);
        assert_eq!(frame.height(), 1080);
        assert_eq!(frame.pixel_format(), PixelFormat::RGB24);
    }
    
    #[test]
    fn test_invalid_dimensions() {
        let result = Frame::new(0, 1080, PixelFormat::RGB24);
        assert!(result.is_err());
    }
}
```

### Integration Tests

Test component interactions:

```rust
// In tests/pipeline_integration.rs
use rust_video_editor::*;

#[tokio::test]
async fn test_full_pipeline() {
    let input = "tests/fixtures/sample.mp4";
    let output = "tests/output/processed.mp4";
    
    let mut pipeline = ProcessingPipeline::new();
    pipeline.add_effect(Box::new(BrightnessEffect::new(1.2)));
    pipeline.add_effect(Box::new(ContrastEffect::new(1.1)));
    
    let result = process_video(input, output, pipeline).await;
    assert!(result.is_ok());
    
    // Verify output exists and is valid
    assert!(std::path::Path::new(output).exists());
}
```

### Benchmarks

Performance testing with Criterion:

```rust
// In benches/frame_processing.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_video_core::{Frame, PixelFormat};

fn benchmark_frame_creation(c: &mut Criterion) {
    c.bench_function("frame_creation_1080p", |b| {
        b.iter(|| {
            Frame::new(
                black_box(1920),
                black_box(1080),
                black_box(PixelFormat::RGB24)
            )
        })
    });
}

fn benchmark_pixel_iteration(c: &mut Criterion) {
    let frame = Frame::new(1920, 1080, PixelFormat::RGB24).unwrap();
    
    c.bench_function("pixel_iteration_1080p", |b| {
        b.iter(|| {
            for pixel in frame.pixels() {
                black_box(pixel);
            }
        })
    });
}

criterion_group!(benches, benchmark_frame_creation, benchmark_pixel_iteration);
criterion_main!(benches);
```

## Performance Guidelines

### SIMD Optimization

Use SIMD for pixel operations:

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn brightness_simd(pixels: &mut [u8], factor: f32) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        if is_x86_feature_detected!("avx2") {
            brightness_avx2(pixels, factor);
        } else if is_x86_feature_detected!("sse2") {
            brightness_sse2(pixels, factor);
        } else {
            brightness_scalar(pixels, factor);
        }
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn brightness_avx2(pixels: &mut [u8], factor: f32) {
    let factor_vec = _mm256_set1_ps(factor);
    
    for chunk in pixels.chunks_exact_mut(32) {
        // Load 32 bytes
        let data = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
        
        // Convert to float, multiply, convert back
        // ... AVX2 operations ...
        
        // Store result
        _mm256_storeu_si256(chunk.as_mut_ptr() as *mut __m256i, result);
    }
}
```

### Parallel Processing

Use Rayon for parallelization:

```rust
use rayon::prelude::*;

pub fn parallel_process_frames(frames: &mut [Frame], effect: &dyn Effect) {
    frames.par_iter_mut().for_each(|frame| {
        effect.process(frame).expect("Effect processing failed");
    });
}
```

### Memory Optimization

Minimize allocations:

```rust
// Bad: Allocates on every call
fn get_pixels(&self) -> Vec<u8> {
    self.data.clone()
}

// Good: Returns reference
fn pixels(&self) -> &[u8] {
    &self.data
}

// Good: Reuse buffers
fn process_with_buffer(&mut self, buffer: &mut Vec<u8>) {
    buffer.clear();
    buffer.extend_from_slice(&self.data);
    // Process buffer...
}
```

## Debugging

### Logging

Use `tracing` for structured logging:

```rust
use tracing::{debug, info, warn, error, instrument};

#[instrument(skip(frame))]
pub fn process_frame(frame: &mut Frame, effect_name: &str) -> Result<()> {
    debug!("Processing frame with effect: {}", effect_name);
    
    let start = std::time::Instant::now();
    
    match apply_effect(frame, effect_name) {
        Ok(()) => {
            info!(
                effect = effect_name,
                duration_ms = start.elapsed().as_millis(),
                "Effect applied successfully"
            );
            Ok(())
        }
        Err(e) => {
            error!(
                effect = effect_name,
                error = ?e,
                "Failed to apply effect"
            );
            Err(e)
        }
    }
}
```

### Debug Builds

Enable debug features:

```toml
[features]
debug = ["frame-dumps", "pipeline-trace", "memory-stats"]

# In code
#[cfg(feature = "frame-dumps")]
fn dump_frame_to_file(frame: &Frame, path: &str) {
    // Save frame for debugging
}
```

## Contributing

### Workflow

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Commit** changes: `git commit -m 'Add amazing feature'`
4. **Push** to branch: `git push origin feature/amazing-feature`
5. **Open** a Pull Request

### PR Guidelines

- Include tests for new functionality
- Update documentation
- Follow coding standards
- Add changelog entry
- Ensure CI passes

### Code Review Process

1. Automated checks (CI)
2. Code review by maintainers
3. Address feedback
4. Merge when approved

### Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create release PR
4. Tag release after merge
5. Publish to crates.io

Remember: Good code is maintainable code. Prioritize clarity and correctness over cleverness.