# Distribution Guide for Rust Video Editor

This guide covers how to distribute and install Rust Video Editor across different platforms and package managers.

## 📦 Supported Package Managers

### macOS

#### Homebrew
```bash
# Install
brew tap your-org/rust-video-editor
brew install rust-video-editor

# Update
brew upgrade rust-video-editor

# Uninstall
brew uninstall rust-video-editor
```

### Linux

#### Snap Store
```bash
# Install from Snap Store
sudo snap install rust-video-editor

# Install from local file
sudo snap install rust-video-editor_1.0.0_amd64.snap --dangerous

# Update (automatic via snapd)
sudo snap refresh rust-video-editor

# Uninstall
sudo snap remove rust-video-editor
```

#### Flatpak
```bash
# Add Flathub repository (if not already added)
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# Install
flatpak install flathub io.github.rust_video_editor

# Run
flatpak run io.github.rust_video_editor

# Update
flatpak update io.github.rust_video_editor

# Uninstall
flatpak uninstall io.github.rust_video_editor
```

#### AppImage
```bash
# Download AppImage
wget https://github.com/your-org/rust-video-editor/releases/download/v1.0.0/rust-video-editor-x86_64.AppImage

# Make executable
chmod +x rust-video-editor-x86_64.AppImage

# Run
./rust-video-editor-x86_64.AppImage
```

#### Debian/Ubuntu (.deb)
```bash
# Download .deb package
wget https://github.com/your-org/rust-video-editor/releases/download/v1.0.0/rust-video-editor_1.0.0_amd64.deb

# Install
sudo dpkg -i rust-video-editor_1.0.0_amd64.deb

# Fix dependencies if needed
sudo apt-get install -f

# Uninstall
sudo apt-get remove rust-video-editor
```

#### Fedora/RHEL (.rpm)
```bash
# Download .rpm package
wget https://github.com/your-org/rust-video-editor/releases/download/v1.0.0/rust-video-editor-1.0.0-1.x86_64.rpm

# Install
sudo rpm -i rust-video-editor-1.0.0-1.x86_64.rpm
# or
sudo dnf install rust-video-editor-1.0.0-1.x86_64.rpm

# Uninstall
sudo rpm -e rust-video-editor
# or
sudo dnf remove rust-video-editor
```

#### Arch Linux (AUR)
```bash
# Using yay
yay -S rust-video-editor

# Using paru
paru -S rust-video-editor

# Manual build
git clone https://aur.archlinux.org/rust-video-editor.git
cd rust-video-editor
makepkg -si
```

### Windows

#### Windows Package Manager (winget)
```powershell
# Install
winget install RustVideoEditor.RustVideoEditor

# Update
winget upgrade RustVideoEditor.RustVideoEditor

# Uninstall
winget uninstall RustVideoEditor.RustVideoEditor
```

#### Chocolatey
```powershell
# Install
choco install rust-video-editor

# Update
choco upgrade rust-video-editor

# Uninstall
choco uninstall rust-video-editor
```

#### Scoop
```powershell
# Add bucket
scoop bucket add rust-video-editor https://github.com/your-org/scoop-bucket

# Install
scoop install rust-video-editor

# Update
scoop update rust-video-editor

# Uninstall
scoop uninstall rust-video-editor
```

## 🔧 Manual Installation

### From Source
```bash
# Clone repository
git clone https://github.com/your-org/rust-video-editor.git
cd rust-video-editor

# Build
cargo build --release

# Install (Unix-like systems)
sudo cp target/release/rust-video-editor /usr/local/bin/
sudo cp assets/rust-video-editor.desktop /usr/share/applications/
sudo cp -r assets/icons/* /usr/share/icons/hicolor/

# Install (Windows)
# Copy rust-video-editor.exe to desired location
# Add to PATH environment variable
```

### Portable Version
Download the portable archive for your platform from the releases page:
- `rust-video-editor-portable-windows-x64.zip`
- `rust-video-editor-portable-linux-x64.tar.gz`
- `rust-video-editor-portable-macos-universal.tar.gz`

Extract and run the executable directly - no installation required.

## 📋 System Requirements

### Minimum Requirements
- **OS**: Windows 10 (1809+), macOS 10.15+, Linux (Ubuntu 20.04+)
- **CPU**: x64 processor with AVX support
- **RAM**: 8 GB
- **GPU**: Vulkan 1.2 compatible with 2GB VRAM
- **Storage**: 4 GB available space

### Recommended Requirements
- **OS**: Windows 11, macOS 13+, Linux (latest LTS)
- **CPU**: 8+ core processor with AVX2
- **RAM**: 16 GB or more
- **GPU**: NVIDIA RTX 3060 / AMD RX 6600 or better
- **Storage**: 10 GB available space on SSD

## 🔗 Dependencies

### Runtime Dependencies
- Vulkan Runtime 1.2+
- Visual C++ Redistributable 2019+ (Windows)
- GStreamer 1.20+ (all platforms)
- FFmpeg libraries
- GTK3 (Linux)

### Optional Dependencies
- NVIDIA CUDA Toolkit (for NVENC)
- AMD AMF SDK (for AMD encoding)
- Intel Media SDK (for QuickSync)

## 🚀 Post-Installation

### First Run
1. Launch Rust Video Editor
2. Complete initial setup wizard
3. Configure default directories
4. Install optional plugins/effects

### GPU Driver Setup
Ensure you have the latest GPU drivers:
- **NVIDIA**: Download from nvidia.com
- **AMD**: Download from amd.com
- **Intel**: Use OS-provided drivers

### Performance Optimization
1. Enable GPU acceleration in Preferences
2. Configure memory cache size
3. Set preview quality settings
4. Enable hardware encoding

## 🐛 Troubleshooting

### Common Issues

#### "Vulkan not found" error
- Install Vulkan runtime from LunarG
- Update GPU drivers
- Check GPU compatibility

#### Performance issues
- Verify GPU acceleration is enabled
- Increase memory cache size
- Close other GPU-intensive applications
- Check thermal throttling

#### Codec issues
- Install additional GStreamer plugins
- Verify FFmpeg is properly installed
- Check codec licensing requirements

## 📞 Support

- **Documentation**: https://docs.rust-video-editor.org
- **Issues**: https://github.com/your-org/rust-video-editor/issues
- **Discord**: https://discord.gg/rust-video-editor
- **Forum**: https://forum.rust-video-editor.org

## 🔄 Updates

### Automatic Updates
Most package managers handle updates automatically:
- Snap: Via snapd daemon
- Flatpak: Via GNOME Software or KDE Discover
- Winget/Chocolatey: Via respective update commands

### Manual Updates
1. Check current version: `rust-video-editor --version`
2. Visit releases page for latest version
3. Follow installation method for your platform

## 📝 License

Rust Video Editor is distributed under the MIT License. See LICENSE file for details.