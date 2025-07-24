# Plugin Development Guide

This guide explains how to create plugins for the Rust Video Editor, extending its functionality with custom effects, transitions, and tools.

## Table of Contents

1. [Plugin Architecture](#plugin-architecture)
2. [Getting Started](#getting-started)
3. [Plugin Types](#plugin-types)
4. [Creating Your First Plugin](#creating-your-first-plugin)
5. [Plugin API Reference](#plugin-api-reference)
6. [Advanced Topics](#advanced-topics)
7. [Testing Plugins](#testing-plugins)
8. [Publishing Plugins](#publishing-plugins)

## Plugin Architecture

### Overview

Plugins in Rust Video Editor are dynamic libraries that implement specific traits. They are loaded at runtime and integrate seamlessly with the editor.

```
┌─────────────────────────────────┐
│      Rust Video Editor          │
├─────────────────────────────────┤
│        Plugin Host API          │
├─────────┬───────────┬───────────┤
│ Effect  │Transition │   Tool    │
│ Plugins │ Plugins   │  Plugins  │
└─────────┴───────────┴───────────┘
```

### Plugin Loading Process

1. Editor scans plugin directories
2. Loads `.so`/`.dll`/`.dylib` files
3. Validates plugin metadata
4. Registers plugin with appropriate subsystem

## Getting Started

### Prerequisites

- Rust 1.75+
- Rust Video Editor Plugin SDK
- Basic understanding of Rust FFI

### Plugin Template

```bash
# Install the plugin template
cargo install rust-video-editor-plugin-template

# Create a new plugin project
cargo generate --git https://github.com/rust-video-editor/plugin-template
cd my-awesome-plugin

# Build the plugin
cargo build --release
```

### Project Structure

```
my-awesome-plugin/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Plugin entry point
│   ├── effect.rs       # Effect implementation
│   └── parameters.rs   # Parameter definitions
├── assets/             # Plugin resources
└── tests/             # Plugin tests
```

## Plugin Types

### 1. Effect Plugins

Transform video/audio data:

```rust
use rust_video_editor_plugin::{EffectPlugin, Frame, Result};

pub struct MyEffect {
    // Effect state
}

impl EffectPlugin for MyEffect {
    fn name(&self) -> &str {
        "My Awesome Effect"
    }
    
    fn process_frame(&mut self, frame: &mut Frame) -> Result<()> {
        // Process the frame
        Ok(())
    }
}
```

### 2. Transition Plugins

Handle transitions between clips:

```rust
use rust_video_editor_plugin::{TransitionPlugin, Frame, Result};

pub struct MyTransition {
    // Transition state
}

impl TransitionPlugin for MyTransition {
    fn name(&self) -> &str {
        "My Smooth Transition"
    }
    
    fn transition(
        &mut self,
        from: &Frame,
        to: &Frame,
        progress: f32,
        output: &mut Frame,
    ) -> Result<()> {
        // Blend frames based on progress (0.0 to 1.0)
        Ok(())
    }
}
```

### 3. Tool Plugins

Add new tools to the editor:

```rust
use rust_video_editor_plugin::{ToolPlugin, ToolContext, Result};

pub struct MyTool {
    // Tool state
}

impl ToolPlugin for MyTool {
    fn name(&self) -> &str {
        "My Custom Tool"
    }
    
    fn icon(&self) -> &[u8] {
        include_bytes!("../assets/tool-icon.png")
    }
    
    fn on_activate(&mut self, context: &mut ToolContext) -> Result<()> {
        // Tool activated
        Ok(())
    }
}
```

## Creating Your First Plugin

### Step 1: Set Up the Project

```toml
# Cargo.toml
[package]
name = "glitch-effect"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
rust-video-editor-plugin = "0.1"
```

### Step 2: Implement the Plugin

```rust
// src/lib.rs
use rust_video_editor_plugin::*;

pub struct GlitchEffect {
    intensity: f32,
    block_size: u32,
}

impl GlitchEffect {
    fn new() -> Self {
        Self {
            intensity: 0.5,
            block_size: 16,
        }
    }
    
    fn apply_glitch(&self, frame: &mut Frame) -> Result<()> {
        let width = frame.width();
        let height = frame.height();
        let pixels = frame.pixels_mut();
        
        // Create random glitch blocks
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for _ in 0..(self.intensity * 100.0) as usize {
            let x = rng.gen_range(0..width - self.block_size);
            let y = rng.gen_range(0..height - self.block_size);
            let offset = rng.gen_range(0..50) as i32;
            
            // Shift pixel blocks
            for dy in 0..self.block_size {
                for dx in 0..self.block_size {
                    let src_x = x + dx;
                    let src_y = y + dy;
                    let dst_x = (src_x as i32 + offset).clamp(0, width as i32 - 1) as u32;
                    
                    let src_idx = (src_y * width + src_x) as usize * 3;
                    let dst_idx = (src_y * width + dst_x) as usize * 3;
                    
                    if src_idx + 3 <= pixels.len() && dst_idx + 3 <= pixels.len() {
                        pixels[dst_idx..dst_idx + 3].copy_from_slice(&pixels[src_idx..src_idx + 3]);
                    }
                }
            }
        }
        
        Ok(())
    }
}

impl EffectPlugin for GlitchEffect {
    fn name(&self) -> &str {
        "Digital Glitch"
    }
    
    fn description(&self) -> &str {
        "Creates digital distortion and glitch effects"
    }
    
    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
    
    fn author(&self) -> &str {
        "Your Name"
    }
    
    fn process_frame(&mut self, frame: &mut Frame) -> Result<()> {
        self.apply_glitch(frame)
    }
    
    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter {
                id: "intensity",
                name: "Glitch Intensity",
                value: ParameterValue::Float(self.intensity),
                min: Some(ParameterValue::Float(0.0)),
                max: Some(ParameterValue::Float(1.0)),
            },
            Parameter {
                id: "block_size",
                name: "Block Size",
                value: ParameterValue::Integer(self.block_size as i32),
                min: Some(ParameterValue::Integer(4)),
                max: Some(ParameterValue::Integer(64)),
            },
        ]
    }
    
    fn set_parameter(&mut self, id: &str, value: ParameterValue) -> Result<()> {
        match (id, value) {
            ("intensity", ParameterValue::Float(v)) => {
                self.intensity = v.clamp(0.0, 1.0);
                Ok(())
            }
            ("block_size", ParameterValue::Integer(v)) => {
                self.block_size = v.clamp(4, 64) as u32;
                Ok(())
            }
            _ => Err(PluginError::InvalidParameter(id.to_string())),
        }
    }
}

// Plugin entry point
#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn EffectPlugin> {
    Box::new(GlitchEffect::new())
}

// Plugin metadata
#[no_mangle]
pub extern "C" fn plugin_metadata() -> PluginMetadata {
    PluginMetadata {
        api_version: PLUGIN_API_VERSION,
        plugin_type: PluginType::Effect,
        name: "Digital Glitch",
        version: env!("CARGO_PKG_VERSION"),
        author: "Your Name",
    }
}
```

### Step 3: Build and Install

```bash
# Build the plugin
cargo build --release

# Copy to plugin directory
cp target/release/libglitch_effect.so ~/.config/rust-video-editor/plugins/

# Or use the install script
./install.sh
```

## Plugin API Reference

### Core Types

```rust
/// Represents a video frame
pub struct Frame {
    pub fn width(&self) -> u32;
    pub fn height(&self) -> u32;
    pub fn pixel_format(&self) -> PixelFormat;
    pub fn pixels(&self) -> &[u8];
    pub fn pixels_mut(&mut self) -> &mut [u8];
    pub fn timestamp(&self) -> Duration;
}

/// Parameter value types
pub enum ParameterValue {
    Float(f32),
    Integer(i32),
    Boolean(bool),
    String(String),
    Color(u32),
    Choice(usize),
}

/// Parameter definition
pub struct Parameter {
    pub id: &'static str,
    pub name: &'static str,
    pub value: ParameterValue,
    pub min: Option<ParameterValue>,
    pub max: Option<ParameterValue>,
    pub choices: Option<Vec<&'static str>>,
}
```

### GPU Acceleration

For GPU-accelerated effects:

```rust
use rust_video_editor_plugin::gpu::*;

pub struct GpuEffect {
    shader: ShaderProgram,
}

impl GpuEffect {
    fn new(context: &GpuContext) -> Result<Self> {
        let shader = ShaderProgram::new(
            context,
            include_str!("vertex.glsl"),
            include_str!("fragment.glsl"),
        )?;
        
        Ok(Self { shader })
    }
}

impl GpuEffectPlugin for GpuEffect {
    fn process_frame_gpu(
        &mut self,
        input: &GpuTexture,
        output: &mut GpuTexture,
        context: &GpuContext,
    ) -> Result<()> {
        self.shader.use_program();
        self.shader.set_uniform("intensity", 0.5f32);
        
        context.render_fullscreen_quad(input, output);
        Ok(())
    }
}
```

### Audio Processing

For audio effects:

```rust
use rust_video_editor_plugin::audio::*;

pub struct ReverbEffect {
    delay_lines: Vec<DelayLine>,
}

impl AudioEffectPlugin for ReverbEffect {
    fn process_audio(
        &mut self,
        input: &[f32],
        output: &mut [f32],
        sample_rate: u32,
    ) -> Result<()> {
        // Apply reverb algorithm
        Ok(())
    }
}
```

## Advanced Topics

### Multi-threaded Processing

```rust
use rayon::prelude::*;

impl EffectPlugin for ParallelEffect {
    fn process_frame(&mut self, frame: &mut Frame) -> Result<()> {
        let width = frame.width() as usize;
        let height = frame.height() as usize;
        let pixels = frame.pixels_mut();
        
        // Process rows in parallel
        pixels
            .par_chunks_mut(width * 3)
            .enumerate()
            .for_each(|(y, row)| {
                self.process_row(row, y);
            });
        
        Ok(())
    }
}
```

### State Persistence

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct EffectState {
    user_presets: Vec<Preset>,
    last_used_settings: Settings,
}

impl StatefulPlugin for MyEffect {
    fn save_state(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(&self.state)?)
    }
    
    fn load_state(&mut self, data: &[u8]) -> Result<()> {
        self.state = serde_json::from_slice(data)?;
        Ok(())
    }
}
```

### Custom UI

```rust
impl UiPlugin for MyEffect {
    fn create_ui(&self) -> Box<dyn PluginUi> {
        Box::new(MyEffectUi::new())
    }
}

struct MyEffectUi {
    // UI state
}

impl PluginUi for MyEffectUi {
    fn draw(&mut self, ctx: &mut UiContext) {
        ctx.heading("My Effect Controls");
        
        ctx.horizontal(|ui| {
            ui.label("Intensity:");
            ui.slider(&mut self.intensity, 0.0..=1.0);
        });
        
        if ctx.button("Reset").clicked() {
            self.reset_to_defaults();
        }
    }
}
```

## Testing Plugins

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_effect_processing() {
        let mut effect = GlitchEffect::new();
        let mut frame = Frame::new_test(100, 100, PixelFormat::RGB24);
        
        // Fill with test pattern
        frame.fill_checkerboard();
        
        // Apply effect
        let result = effect.process_frame(&mut frame);
        assert!(result.is_ok());
        
        // Verify frame was modified
        assert_ne!(frame.pixels(), Frame::new_test(100, 100, PixelFormat::RGB24).pixels());
    }
}
```

### Integration Tests

```rust
#[test]
fn test_plugin_loading() {
    let plugin_path = "target/release/libmy_plugin.so";
    let loader = PluginLoader::new();
    
    let plugin = loader.load_plugin(plugin_path).unwrap();
    assert_eq!(plugin.name(), "My Awesome Effect");
}
```

### Performance Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_effect(c: &mut Criterion) {
    let mut effect = GlitchEffect::new();
    let mut frame = Frame::new_test(1920, 1080, PixelFormat::RGB24);
    
    c.bench_function("glitch_effect_1080p", |b| {
        b.iter(|| {
            effect.process_frame(black_box(&mut frame)).unwrap();
        })
    });
}

criterion_group!(benches, benchmark_effect);
criterion_main!(benches);
```

## Publishing Plugins

### Packaging

```toml
# Cargo.toml metadata
[package.metadata.plugin]
category = "effects/distortion"
tags = ["glitch", "digital", "distortion"]
preview = "assets/preview.gif"
icon = "assets/icon.png"
```

### Distribution

1. **Plugin Registry**: Submit to the official plugin registry
2. **GitHub Releases**: Use GitHub releases with binaries
3. **Cargo**: Publish as a Cargo crate

### Best Practices

1. **Versioning**: Follow semantic versioning
2. **Documentation**: Include comprehensive docs
3. **Examples**: Provide usage examples
4. **Performance**: Optimize for real-time processing
5. **Compatibility**: Test on all platforms
6. **Error Handling**: Handle errors gracefully
7. **Resources**: Clean up resources properly

### Plugin Manifest

```json
{
    "name": "Digital Glitch",
    "version": "1.0.0",
    "api_version": "0.1",
    "author": "Your Name",
    "description": "Creates digital distortion effects",
    "category": "effects/distortion",
    "platforms": ["windows", "macos", "linux"],
    "min_editor_version": "0.5.0",
    "dependencies": [],
    "files": {
        "windows": "glitch_effect.dll",
        "macos": "libglitch_effect.dylib",
        "linux": "libglitch_effect.so"
    }
}
```

## Resources

- [Plugin API Documentation](https://docs.rustvideoeditor.com/plugin-api)
- [Example Plugins](https://github.com/rust-video-editor/example-plugins)
- [Plugin Development Forum](https://forum.rustvideoeditor.com/plugins)
- [Video Tutorials](https://youtube.com/rustvideoeditor-plugins)

Happy plugin development!