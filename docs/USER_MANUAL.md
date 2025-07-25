# 🎬 Rust Video Editor - User Manual

## Table of Contents

1. [Getting Started](#getting-started)
2. [Installation](#installation)
3. [Feature Walkthrough](#feature-walkthrough)
4. [Keyboard Shortcuts](#keyboard-shortcuts)
5. [Troubleshooting](#troubleshooting)
6. [FAQ](#faq)

---

## Getting Started

Welcome to Rust Video Editor - a powerful, open-source video editing application that combines professional features with blazing-fast performance. Whether you're creating content for social media, YouTube, or professional projects, this editor provides all the tools you need.

### Key Features

- 🚀 **Hardware-accelerated processing** for real-time performance
- 🎨 **GPU-powered effects** with instant preview
- 🎬 **Multi-track timeline** with drag-and-drop editing
- 🔊 **Audio editing** with waveform visualization
- 💾 **Multiple export formats** including H.264, H.265, VP9, and AV1
- 🔒 **Privacy-focused** - all processing happens locally

### System Requirements

**Minimum Requirements:**
- OS: Windows 10+, macOS 10.15+, or Linux (Ubuntu 20.04+)
- RAM: 8GB
- Storage: 2GB available space
- Graphics: Any GPU with OpenGL 3.3 support

**Recommended Requirements:**
- OS: Latest version of Windows 11, macOS, or Linux
- RAM: 16GB or more
- Storage: 10GB available space (for cache and projects)
- Graphics: Dedicated GPU with 4GB+ VRAM for best performance

---

## Installation

### Windows

1. Download the latest `.msi` installer from the [releases page](https://github.com/yourusername/rust-video-editor/releases)
2. Double-click the installer and follow the setup wizard
3. Launch from Start Menu or Desktop shortcut

### macOS

1. Download the latest `.dmg` file from the [releases page](https://github.com/yourusername/rust-video-editor/releases)
2. Open the DMG and drag the app to your Applications folder
3. On first launch, right-click and select "Open" to bypass Gatekeeper

### Linux

#### AppImage (Recommended)
```bash
# Download the AppImage
wget https://github.com/yourusername/rust-video-editor/releases/latest/download/rust-video-editor.AppImage

# Make it executable
chmod +x rust-video-editor.AppImage

# Run the application
./rust-video-editor.AppImage
```

#### Debian/Ubuntu (.deb)
```bash
# Download the .deb package
wget https://github.com/yourusername/rust-video-editor/releases/latest/download/rust-video-editor.deb

# Install the package
sudo dpkg -i rust-video-editor.deb

# Fix any dependency issues
sudo apt-get install -f
```

#### Build from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/rust-video-editor.git
cd rust-video-editor

# Install dependencies
# Ubuntu/Debian:
sudo apt-get install libavcodec-dev libavformat-dev libavutil-dev libswscale-dev

# Build and run
cargo tauri build
```

---

## Feature Walkthrough

### 1. Project Management

![Project Creation](screenshots/project-creation.png)
*Screenshot placeholder: Project creation dialog*

**Creating a New Project:**
1. Click **File → New Project** or press `Ctrl+N` (Cmd+N on macOS)
2. Choose project settings:
   - Resolution: 720p, 1080p, 4K, or custom
   - Frame rate: 24, 25, 30, 60 fps
   - Audio sample rate: 44.1kHz or 48kHz
3. Click "Create" to start editing

**Opening Existing Projects:**
- Click **File → Open Project** or press `Ctrl+O`
- Projects are saved as `.rvep` files (Rust Video Editor Project)

### 2. Importing Media

![Media Import](screenshots/media-import.png)
*Screenshot placeholder: Media browser with imported files*

**Supported Formats:**
- Video: MP4, MOV, AVI, WebM, MKV
- Audio: MP3, WAV, AAC, OGG, FLAC
- Images: JPG, PNG, WebP, GIF

**Import Methods:**
1. **Drag and Drop**: Drag files directly into the media browser
2. **Import Dialog**: Click the import button or press `Ctrl+I`
3. **Folder Import**: Import entire folders with **File → Import Folder**

### 3. Timeline Editing

![Timeline Interface](screenshots/timeline-interface.png)
*Screenshot placeholder: Multi-track timeline with clips*

**Basic Operations:**
- **Add to Timeline**: Drag media from browser to timeline
- **Move Clips**: Click and drag clips to reposition
- **Trim Clips**: Hover over clip edges and drag to trim
- **Split Clips**: Position playhead and press `S`
- **Delete Clips**: Select and press `Delete`

**Timeline Navigation:**
- **Zoom**: `Ctrl + Mouse Wheel` or `+/-` keys
- **Pan**: Middle mouse button drag or hold `Space` and drag
- **Snap to Grid**: Toggle with `G` key
- **Magnetic Timeline**: Clips automatically snap together

### 4. Effects and Transitions

![Effects Panel](screenshots/effects-panel.png)
*Screenshot placeholder: Effects browser with categories*

**Applying Effects:**
1. Select a clip on the timeline
2. Browse effects in the Effects panel
3. Double-click or drag effect onto clip
4. Adjust parameters in the Properties panel

**Built-in Effects:**
- **Color Correction**: Brightness, Contrast, Saturation, Hue
- **Blur & Sharpen**: Gaussian Blur, Motion Blur, Sharpen
- **Stylize**: Black & White, Sepia, Vintage, Film Grain
- **Transform**: Scale, Rotate, Position, Crop
- **Audio**: EQ, Compressor, Reverb, Noise Reduction

**Transitions:**
- **Basic**: Fade, Dissolve, Dip to Black
- **Wipes**: Linear, Radial, Clock
- **3D**: Cube, Flip, Page Turn
- **Custom**: Import custom transition presets

### 5. Audio Editing

![Audio Waveform](screenshots/audio-editing.png)
*Screenshot placeholder: Audio track with waveform*

**Audio Features:**
- **Waveform Display**: Visual representation of audio
- **Volume Keyframes**: Click to add volume points
- **Audio Effects**: Apply EQ, compression, and more
- **Sync Tools**: Automatically sync audio to video
- **Ducking**: Auto-lower music during dialogue

### 6. Text and Titles

![Text Editor](screenshots/text-editor.png)
*Screenshot placeholder: Text editing interface*

**Adding Text:**
1. Click the Text tool or press `T`
2. Click on preview to place text
3. Type your text and style it
4. Animate with keyframes

**Text Features:**
- Multiple fonts and styles
- Drop shadows and outlines
- Motion presets (typewriter, fade, slide)
- 3D text with depth
- Import custom fonts

### 7. Keyframe Animation

![Keyframe Editor](screenshots/keyframe-editor.png)
*Screenshot placeholder: Keyframe animation curves*

**Creating Animations:**
1. Select property to animate (position, scale, opacity, etc.)
2. Move playhead to start position
3. Click keyframe button to add keyframe
4. Move playhead and change value
5. Add more keyframes as needed

**Keyframe Types:**
- Linear: Constant speed
- Ease In/Out: Smooth acceleration
- Bezier: Custom curves
- Hold: Instant changes

### 8. Export Settings

![Export Dialog](screenshots/export-dialog.png)
*Screenshot placeholder: Export settings interface*

**Export Presets:**
- **YouTube**: H.264, 1080p/4K, optimized bitrate
- **Vimeo**: H.264, high quality, proper metadata
- **Instagram**: Square/vertical, 60-second clips
- **Twitter**: Compressed for quick upload
- **Professional**: ProRes, DNxHD for further editing

**Custom Export:**
1. Choose codec: H.264, H.265, VP9, AV1
2. Set resolution and frame rate
3. Adjust bitrate (CBR/VBR)
4. Configure audio settings
5. Add metadata (title, author, copyright)

---

## Keyboard Shortcuts

### General
| Action | Windows/Linux | macOS |
|--------|--------------|-------|
| New Project | Ctrl+N | Cmd+N |
| Open Project | Ctrl+O | Cmd+O |
| Save Project | Ctrl+S | Cmd+S |
| Export Video | Ctrl+E | Cmd+E |
| Undo | Ctrl+Z | Cmd+Z |
| Redo | Ctrl+Y | Cmd+Shift+Z |

### Timeline
| Action | Shortcut |
|--------|----------|
| Play/Pause | Space |
| Next Frame | → |
| Previous Frame | ← |
| Jump 10 frames | Shift+→/← |
| Split Clip | S |
| Delete Clip | Delete |
| Select All | Ctrl/Cmd+A |
| Copy | Ctrl/Cmd+C |
| Paste | Ctrl/Cmd+V |

### Tools
| Action | Shortcut |
|--------|----------|
| Selection Tool | V |
| Razor Tool | C |
| Text Tool | T |
| Hand Tool | H |
| Zoom Tool | Z |

### View
| Action | Shortcut |
|--------|----------|
| Zoom In | + or Ctrl+= |
| Zoom Out | - or Ctrl+- |
| Fit Timeline | \ |
| Toggle Fullscreen | F11 |
| Toggle Effects | F5 |
| Toggle Audio Meters | F6 |

---

## Troubleshooting

### Common Issues

#### 1. Application Won't Launch
- **Windows**: Install Visual C++ Redistributables
- **macOS**: Check Security & Privacy settings
- **Linux**: Install required dependencies with package manager

#### 2. Slow Performance
- Enable GPU acceleration in Settings
- Close other applications
- Reduce preview quality during editing
- Clear cache in Settings → Storage

#### 3. Import Issues
- Ensure codecs are supported
- Try converting file with HandBrake
- Check file permissions
- Verify file isn't corrupted

#### 4. Export Failures
- Check available disk space
- Ensure output path is writable
- Try different codec
- Reduce export settings

#### 5. Audio Sync Problems
- Right-click clip → "Sync Audio"
- Manually adjust with Alt+←/→
- Check project frame rate matches source

### Performance Optimization

1. **Enable Hardware Acceleration**
   - Settings → Performance → GPU Acceleration
   - Select your GPU if multiple available

2. **Proxy Editing**
   - For 4K footage, generate proxies
   - Right-click clips → Generate Proxies
   - Toggle proxy mode with P key

3. **Cache Settings**
   - Increase RAM cache in Settings
   - Set cache location to fast SSD
   - Clear old cache regularly

---

## FAQ

### General Questions

**Q: Is Rust Video Editor free?**
A: Yes! It's completely free and open source under the MIT license.

**Q: Can I use it commercially?**
A: Absolutely. There are no restrictions on commercial use.

**Q: Does it require internet?**
A: No, all processing happens locally. Internet is only needed for updates.

**Q: How does it compare to Adobe Premiere?**
A: While not as feature-rich as Premiere, it offers excellent performance and all essential editing tools for most projects.

### Technical Questions

**Q: What's the maximum resolution?**
A: Supports up to 8K resolution, limited by your hardware.

**Q: Can I use plugins?**
A: Yes, there's a plugin API for custom effects and tools.

**Q: Is my GPU supported?**
A: Any GPU with OpenGL 3.3+ works. NVIDIA, AMD, and Intel GPUs are all supported.

**Q: How do I report bugs?**
A: File issues on our GitHub repository with detailed reproduction steps.

### Workflow Questions

**Q: Can I import Premiere/Final Cut projects?**
A: Not directly, but you can export as XML and convert.

**Q: Does it support RAW video?**
A: Yes, through FFmpeg. Most camera RAW formats are supported.

**Q: Can multiple people edit simultaneously?**
A: Not built-in, but project files can be version controlled with Git.

**Q: How do I backup projects?**
A: Projects are single `.rvep` files. Just copy to backup location.

---

## Getting Help

- **Documentation**: https://docs.rust-video-editor.com
- **Community Forum**: https://forum.rust-video-editor.com
- **Discord Server**: https://discord.gg/rust-video-editor
- **GitHub Issues**: https://github.com/yourusername/rust-video-editor/issues
- **Email Support**: support@rust-video-editor.com

---

*Last updated: January 2025*
*Version: 1.0.0*