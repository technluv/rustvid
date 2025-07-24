# API Reference

This section provides complete API documentation for all public interfaces in the Rust Video Editor.

## Crate Overview

The project is organized into several crates, each providing specific functionality:

### Core Crate (`rust_video_core`)
The foundation of the video processing system.

- [Traits](core/traits.md) - Core interfaces for video processing
- [Frame](core/frame.md) - Frame representation and manipulation
- [Decoder](core/decoder.md) - Video decoding interfaces
- [Buffer](core/buffer.md) - High-performance frame buffering
- [Pipeline](core/pipeline.md) - Video processing pipeline
- [Error Handling](core/errors.md) - Error types and handling

### Timeline Crate (`video_editor_timeline`)
Timeline management and editing operations.

- [Timeline](timeline/timeline.md) - Main timeline structure
- [Track](timeline/track.md) - Track management
- [Clip](timeline/clip.md) - Clip representation
- [Transitions](timeline/transitions.md) - Transition effects

### Effects Crate (`video_editor_effects`)
Video and audio effects system.

- [Effect Trait](effects/trait.md) - Core effect interface
- [Built-in Effects](effects/builtin.md) - Standard effects library
- [Parameters](effects/parameters.md) - Effect parameter system
- [Chain](effects/chain.md) - Effect chaining and composition

### Export Crate (`video_editor_export`)
Rendering and export functionality.

- [Export Engine](export/engine.md) - Main export system
- [Settings](export/settings.md) - Export configuration
- [Progress](export/progress.md) - Progress tracking
- [Formats](export/formats.md) - Supported export formats

### UI Crate (`video_editor_ui`)
User interface abstractions.

- [Application State](ui/state.md) - UI state management
- [Configuration](ui/config.md) - UI configuration
- [Themes](ui/themes.md) - Theming system

## Common Patterns

### Error Handling

All crates follow a consistent error handling pattern:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Description: {0}")]
    SpecificError(String),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ModuleError>;
```

### Async Operations

Many operations are async, particularly in the export and processing pipelines:

```rust
async fn process_video(input: &str) -> Result<()> {
    let decoder = FFmpegDecoder::new();
    decoder.open(input).await?;
    // Process frames...
    Ok(())
}
```

### Builder Pattern

Complex objects use the builder pattern for construction:

```rust
let frame = FrameBuilder::new()
    .width(1920)
    .height(1080)
    .pixel_format(PixelFormat::RGB24)
    .build()?;
```

## Thread Safety

Most types implement `Send + Sync` for use in concurrent contexts:

```rust
pub trait VideoProcessor: Send + Sync {
    // Methods...
}
```

## Memory Management

The project uses efficient memory management strategies:

- Buffer pooling for frame data
- Reference counting for shared resources
- Lazy allocation where possible

## Performance Considerations

- Use `rayon` for parallel processing
- Leverage SIMD operations where available
- Minimize allocations in hot paths
- Use zero-copy operations when possible

## Examples

See the [examples](../examples/) directory for complete working examples of:

- Basic video decoding
- Timeline manipulation
- Effect application
- Export operations

## Version Compatibility

This documentation corresponds to version 0.1.0 of the Rust Video Editor.

API stability is not yet guaranteed. Breaking changes may occur in minor version updates until 1.0.0.