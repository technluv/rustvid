# 🎬 Rust Video Editor

A high-performance, open-source video editor built with Rust and Tauri. Designed as a privacy-focused alternative to cloud-based video editors like Veed.io.

## ✨ Features

### 🚀 **Performance**
- **Hardware-accelerated decoding** via FFmpeg with GPU support
- **Real-time effects processing** using GPU compute shaders
- **Memory-efficient buffering** with smart caching and pooling
- **Multi-threaded architecture** for optimal CPU utilization

### 🎨 **Professional Video Editing**
- **Multi-track timeline** with drag-and-drop editing
- **Real-time preview** with scrubbing and playback controls
- **GPU-accelerated effects** (blur, color correction, levels)
- **Professional transitions** (fade, dissolve, wipe, circular)
- **Keyframe animation** for dynamic effect parameters

### 📹 **Format Support**
- **Input**: MP4, AVI, MOV, WebM, and more
- **Output**: H.264, H.265/HEVC, VP9, AV1
- **Hardware encoding** (NVENC, QuickSync, AMF, VideoToolbox)
- **Export presets** for YouTube, Vimeo, social media platforms

### 🔒 **Privacy & Open Source**
- **100% local processing** - no cloud uploads required
- **Open source** MIT license
- **Cross-platform** support (Linux, Windows, macOS)
- **No telemetry** or user tracking

## 📦 Installation

### Download Pre-built Releases

The easiest way to get started is to download a pre-built release for your platform:

[![Windows](https://img.shields.io/badge/Windows-0078D6?logo=windows&logoColor=white)](https://github.com/your-org/rust-video-editor/releases/latest/download/rust-video-editor_windows_x64.msi)
[![macOS](https://img.shields.io/badge/macOS-000000?logo=apple&logoColor=white)](https://github.com/your-org/rust-video-editor/releases/latest/download/rust-video-editor_macos.dmg)
[![Linux](https://img.shields.io/badge/Linux-FCC624?logo=linux&logoColor=black)](https://github.com/your-org/rust-video-editor/releases/latest/download/rust-video-editor_linux.AppImage)

**Windows**: Download `.msi` installer and run
**macOS**: Download `.dmg`, open, and drag to Applications
**Linux**: Download `.AppImage`, make executable with `chmod +x`, and run

### Build from Source

#### Prerequisites
- Rust 1.70+ with Cargo
- Node.js 18+ with npm
- FFmpeg development libraries

#### Platform-specific Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install libavcodec-dev libavformat-dev libavutil-dev libswscale-dev \
                     libavfilter-dev libavdevice-dev pkg-config build-essential
```

**macOS:**
```bash
brew install ffmpeg pkg-config
```

**Windows:**
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
- Download [FFmpeg dev libraries](https://www.gyan.dev/ffmpeg/builds/)
- Set `FFMPEG_DIR` environment variable

#### Build Instructions

```bash
# Clone the repository
git clone https://github.com/your-org/rust-video-editor
cd rust-video-editor

# Install frontend dependencies
cd crates/ui/ui && npm install && cd ../../..

# Build and run in development mode
cd crates/ui
cargo tauri dev

# Build for production
cargo tauri build
```

## 🚀 Quick Start

### 5-Minute Tutorial

1. **Launch the editor** - Double-click the app icon or run from terminal
2. **Create a project** - `Ctrl/Cmd+N` → Choose 1080p 30fps → Create
3. **Import media** - Drag videos/images into the Media Browser
4. **Edit timeline** - Drag clips to timeline, trim by dragging edges
5. **Add effects** - Select clip → Browse Effects → Double-click to apply
6. **Export video** - `Ctrl/Cmd+E` → Choose preset → Export

See our [Quick Start Guide](docs/QUICK_START.md) for detailed instructions.

## 📖 Documentation

- [User Manual](docs/USER_MANUAL.md) - Complete guide with screenshots
- [Quick Start Guide](docs/QUICK_START.md) - Get editing in 5 minutes
- [API Documentation](https://docs.rs/rust-video-editor) - For developers
- [Architecture Guide](docs/ARCHITECTURE.md) - System design details

## 🏗️ Architecture

The editor is built as a modular Rust workspace with the following crates:

```
rust-video-editor/
├── crates/
│   ├── core/          # Video processing engine
│   ├── timeline/      # Timeline data structures
│   ├── effects/       # GPU-accelerated effects
│   ├── export/        # Video encoding pipeline
│   └── ui/           # Tauri + React frontend
├── frontend/         # React components and UI
└── tests/            # Integration tests
```

### Core Components

- **Video Engine**: FFmpeg-based decoder with frame management
- **Effects System**: GPU-accelerated effects using wgpu
- **Timeline**: Multi-track editing with precise timing
- **Export Pipeline**: Hardware-accelerated encoding
- **UI**: React-based interface with Tauri backend

## 🎯 Usage Examples

### Basic Video Processing

```rust
use rust_video_core::{
    pipeline::{VideoPipeline, PipelineConfig},
    traits::PixelFormat,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = PipelineConfig {
        target_pixel_format: PixelFormat::RGB24,
        ..Default::default()
    };
    
    let mut pipeline = VideoPipeline::new(config);
    pipeline.open_file("input.mp4").await?;
    pipeline.play().await?;
    
    while let Some(frame) = pipeline.get_frame().await {
        // Process frame for display
    }
    
    Ok(())
}
```

### Adding Effects

```rust
use rust_video_effects::{
    pipeline::EffectPipeline,
    effects::FilterFactory,
};

let mut effect_pipeline = EffectPipeline::new().await?;

// Add blur effect
let blur = FilterFactory::create_blur(5.0)?;
effect_pipeline.add_effect(blur);

// Add color correction
let color = FilterFactory::create_color_correction(1.2, 1.1, 1.0)?;
effect_pipeline.add_effect(color);

// Process frame
let processed = effect_pipeline.process_frame(&frame, timestamp).await?;
```

### Export Video

```rust
use rust_video_export::{
    presets::ExportPresets,
    job::{ExportJob, JobManager},
};

let preset = ExportPresets::youtube_1080p();
let job = ExportJob::new("input.mp4", "output.mp4", preset)?;

let mut manager = JobManager::new();
manager.submit_job(job).await?;

// Monitor progress
while let Some(progress) = manager.get_progress().await {
    println!("Progress: {:.1}%", progress.percentage);
}
```

## 🔧 Development

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests  
cargo test --test integration_tests

# Benchmarks
cargo bench

# Generate test fixtures
python tests/fixtures/create_test_videos.py
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Security audit
cargo audit

# Check unused dependencies
cargo udeps
```

## 📊 Performance

### Benchmarks

- **Frame Processing**: 300MB/s sustained throughput
- **Cache Performance**: 95%+ hit rate for sequential access
- **Effect Rendering**: Real-time for HD content
- **Export Speed**: Faster than real-time for most codecs
- **Memory Usage**: ~200MB for HD timeline

### System Requirements

- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 2GB installation, temp space for processing
- **GPU**: Optional but recommended for effects
- **CPU**: Multi-core recommended for export

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Run the full test suite
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **FFmpeg** for video processing capabilities
- **Tauri** for cross-platform desktop framework
- **wgpu** for GPU acceleration
- **React** for the user interface

## 🚀 Roadmap

### Completed ✅
- Core video engine with FFmpeg
- GPU-accelerated effects system
- Multi-track timeline interface
- Hardware-accelerated export
- Comprehensive test suite

### In Progress 🔄
- Final UI polish and accessibility
- Additional effect presets
- Performance optimizations

### Planned 📋
- Audio editing capabilities
- Plugin system for custom effects
- Collaborative editing features
- Mobile app (React Native)

---

**Built with ❤️ in Rust | Privacy-focused | Open Source | Cross-platform**

For questions, issues, or feature requests, please open an issue on GitHub.