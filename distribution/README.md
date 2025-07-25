# Rust Video Editor Distribution

This directory contains all the configuration files and scripts needed to distribute Rust Video Editor across various package managers and platforms.

## Directory Structure

```
distribution/
├── homebrew/           # macOS Homebrew formula
├── snap/               # Linux Snap package configuration
├── flatpak/            # Linux Flatpak manifest
├── winget/             # Windows Package Manager manifest
├── chocolatey/         # Windows Chocolatey package
├── scripts/            # Installation verification scripts
└── docs/               # Distribution documentation
```

## Quick Start

### Building Packages

1. **Homebrew (macOS)**
   ```bash
   # Test the formula locally
   brew install --build-from-source ./homebrew/rust-video-editor.rb
   
   # Submit to homebrew-core after testing
   ```

2. **Snap (Linux)**
   ```bash
   # Build snap package
   snapcraft --use-lxd
   
   # Test locally
   sudo snap install rust-video-editor_*.snap --dangerous
   
   # Upload to Snap Store
   snapcraft upload rust-video-editor_*.snap
   ```

3. **Flatpak (Linux)**
   ```bash
   # Build flatpak
   flatpak-builder --force-clean build-dir flatpak/io.github.rust_video_editor.yaml
   
   # Test locally
   flatpak-builder --user --install --force-clean build-dir flatpak/io.github.rust_video_editor.yaml
   
   # Submit to Flathub
   ```

4. **Windows Package Manager**
   ```powershell
   # Validate manifest
   winget validate winget/rust-video-editor.yaml
   
   # Submit via GitHub PR to microsoft/winget-pkgs
   ```

5. **Chocolatey (Windows)**
   ```powershell
   # Pack the package
   choco pack chocolatey/rust-video-editor.nuspec
   
   # Test locally
   choco install rust-video-editor -source .
   
   # Push to Chocolatey Community Repository
   choco push rust-video-editor.1.0.0.nupkg --source https://push.chocolatey.org/
   ```

## Verification Scripts

The `scripts/` directory contains platform-specific verification scripts:

- **Linux/macOS**: `verify-install.sh`
- **Windows**: `verify-install.ps1`

These scripts check for:
- Rust Video Editor installation
- Required dependencies
- GPU drivers and Vulkan support
- Optional features

## Updating Package Versions

When releasing a new version:

1. Update version numbers in all package configurations
2. Update SHA256 checksums for downloadable files
3. Update changelog/release notes
4. Test packages locally before submission
5. Submit updates to respective package repositories

## Package Manager Links

- **Homebrew**: https://brew.sh
- **Snap Store**: https://snapcraft.io
- **Flathub**: https://flathub.org
- **Windows Package Manager**: https://github.com/microsoft/winget-pkgs
- **Chocolatey**: https://chocolatey.org

## Notes

- Always test packages locally before submission
- Follow each package manager's guidelines and best practices
- Keep package descriptions consistent across platforms
- Ensure all dependencies are properly declared
- Update checksums for every new release

For more information, see the [Distribution Guide](docs/DISTRIBUTION.md).