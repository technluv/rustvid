# Release Guide for Rust Video Editor

This guide covers the complete release process for Rust Video Editor, including building, packaging, signing, and distributing the application.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Version Management](#version-management)
3. [Building Releases](#building-releases)
4. [Code Signing](#code-signing)
5. [Creating Releases](#creating-releases)
6. [Auto-Updater](#auto-updater)
7. [Distribution](#distribution)
8. [Post-Release](#post-release)

## Prerequisites

### Development Environment

- **Rust**: Latest stable version
- **Git**: For version control
- **GitHub CLI**: For automated releases (`gh`)

### Platform-Specific Requirements

#### Windows
- Windows 10 or later
- Visual Studio 2019+ with C++ tools
- WiX Toolset 3.11+ (for MSI creation)
- Windows SDK (for code signing)

#### macOS
- macOS 10.15 or later
- Xcode Command Line Tools
- Apple Developer Account (for signing)

#### Linux
- Ubuntu 20.04+ or equivalent
- Build essentials (`gcc`, `make`, etc.)
- `rpm-build` for RPM packages
- `dpkg-dev` for DEB packages

## Version Management

### Semantic Versioning

We follow semantic versioning (MAJOR.MINOR.PATCH):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Updating Version

1. Update version in `Cargo.toml`:
   ```toml
   [package]
   version = "1.2.3"
   ```

2. Update version in `Cargo.lock`:
   ```bash
   cargo update -p rust-video-editor
   ```

3. Commit version changes:
   ```bash
   git add Cargo.toml Cargo.lock
   git commit -m "chore: bump version to 1.2.3"
   ```

## Building Releases

### Quick Build (All Platforms)

```bash
# Build for all available platforms
./scripts/build-all.sh --version 1.2.3

# Build in parallel (faster)
./scripts/build-all.sh --version 1.2.3 --parallel

# Build specific platform
./scripts/build-all.sh --version 1.2.3 --platform windows
```

### Platform-Specific Builds

#### Windows
```powershell
# Basic build
.\scripts\build-windows.ps1 -Version "1.2.3"

# With code signing
.\scripts\build-windows.ps1 -Version "1.2.3" -SignCode -CertPath "cert.pfx" -CertPassword "password"

# Portable only (no installer)
.\scripts\build-windows.ps1 -Version "1.2.3" -CreateInstaller:$false
```

#### macOS
```bash
# Basic build
./scripts/build-macos.sh --version 1.2.3

# With code signing and notarization
./scripts/build-macos.sh --version 1.2.3 \
  --sign "Developer ID Application: Your Name (TEAMID)" \
  --notarize "apple@id.com" "app-specific-password"
```

#### Linux
```bash
# Build all package types
./scripts/build-linux.sh --version 1.2.3

# Specific architecture
./scripts/build-linux.sh --version 1.2.3 --arch aarch64

# Specific package type
./scripts/build-linux.sh --version 1.2.3 --no-snap --no-rpm
```

## Code Signing

### Setup Code Signing

Run the interactive setup script:
```bash
./scripts/sign-code.sh
```

### Environment Variables

Set these before building:

#### Windows
```bash
export WINDOWS_CERTIFICATE_PATH="/path/to/cert.pfx"
export WINDOWS_CERTIFICATE_PASSWORD="password"
```

#### macOS
```bash
export MACOS_CERTIFICATE_PATH="/path/to/cert.p12"
export MACOS_CERTIFICATE_PWD="password"
export MACOS_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)"
export MACOS_NOTARIZATION_APPLE_ID="your@apple.id"
export MACOS_NOTARIZATION_PWD="app-specific-password"
```

#### Linux
```bash
export LINUX_GPG_KEY="your-gpg-key-id"
export LINUX_GPG_PASSPHRASE="passphrase"
```

## Creating Releases

### Automated Release (Recommended)

1. Create and push a version tag:
   ```bash
   git tag v1.2.3
   git push origin v1.2.3
   ```

2. GitHub Actions will automatically:
   - Build for all platforms
   - Sign the binaries
   - Create a draft release
   - Upload all artifacts
   - Generate checksums
   - Publish the release

### Manual Release

1. Build all platforms:
   ```bash
   ./scripts/build-all.sh --version 1.2.3 --parallel
   ```

2. Create release on GitHub:
   ```bash
   gh release create v1.2.3 \
     --title "Rust Video Editor v1.2.3" \
     --notes-file RELEASE_NOTES.md \
     --draft
   ```

3. Upload artifacts:
   ```bash
   gh release upload v1.2.3 dist/*/*.{msi,zip,dmg,AppImage,deb,rpm,tar.gz}
   ```

4. Upload checksums:
   ```bash
   gh release upload v1.2.3 dist/checksums-all.txt
   ```

5. Publish release:
   ```bash
   gh release edit v1.2.3 --draft=false
   ```

## Auto-Updater

### Generate Update Manifest

After building, generate the update manifest:
```bash
./scripts/generate-update-manifest.sh --version 1.2.3
```

With signing:
```bash
./scripts/generate-update-manifest.sh --version 1.2.3 --sign private-key.pem
```

### Deploy Update Manifest

1. Upload to your update server:
   ```bash
   scp dist/update-manifest.json user@server:/var/www/updates/
   ```

2. Or use GitHub releases:
   ```bash
   gh release upload v1.2.3 dist/update-manifest.json
   ```

### Update Server Configuration

Configure your web server to serve update manifests:

```nginx
location /update {
    root /var/www/updates;
    add_header Access-Control-Allow-Origin *;
    add_header Cache-Control "no-cache, no-store, must-revalidate";
}
```

## Distribution

### Package Repositories

#### Windows
- Microsoft Store (optional)
- Chocolatey: `choco push rustvideo.1.2.3.nupkg`
- Scoop: Update manifest in scoop bucket

#### macOS
- Mac App Store (optional)
- Homebrew: Update formula in homebrew tap
- MacPorts: Update portfile

#### Linux
- Flathub: Update flatpak manifest
- Snap Store: `snapcraft upload rustvideo_1.2.3_amd64.snap`
- AUR: Update PKGBUILD

### CDN Distribution

Upload to CDN for faster downloads:
```bash
aws s3 sync dist/ s3://releases.rustvideo.com/v1.2.3/ --acl public-read
```

## Post-Release

### Monitoring

1. **Download Statistics**: Monitor GitHub release downloads
2. **Auto-Update Adoption**: Track update manifest requests
3. **Error Reports**: Monitor crash reports and issues

### Communication

1. **Release Announcement**:
   - Blog post
   - Social media
   - Mailing list
   - Discord/Forum

2. **Documentation Updates**:
   - Update website
   - Update user guide
   - Update API docs

3. **Community**:
   - Respond to feedback
   - Track bug reports
   - Plan next release

### Hotfix Process

For critical bugs:

1. Create hotfix branch:
   ```bash
   git checkout -b hotfix/1.2.4 v1.2.3
   ```

2. Fix the issue and test

3. Bump patch version:
   ```bash
   # Update to 1.2.4 in Cargo.toml
   ```

4. Build and release:
   ```bash
   ./scripts/build-all.sh --version 1.2.4 --parallel
   git tag v1.2.4
   git push origin v1.2.4
   ```

## Troubleshooting

### Common Issues

#### Windows Build Fails
- Ensure Visual Studio is installed with C++ tools
- Check WiX Toolset installation
- Verify certificate format (PFX)

#### macOS Notarization Fails
- Check Apple Developer account status
- Verify app-specific password
- Ensure all binaries are signed

#### Linux Package Errors
- Install required package tools (`dpkg-dev`, `rpm-build`)
- Check file permissions
- Verify desktop file validity

### Build Logs

All builds create logs in `dist/logs/`:
- `build-windows.log`
- `build-macos.log`
- `build-linux.log`

Check these for detailed error messages.

## Security Considerations

1. **Private Keys**: Never commit signing keys to git
2. **Passwords**: Use environment variables or secure vaults
3. **Checksums**: Always provide SHA256 checksums
4. **HTTPS**: Use HTTPS for all download links
5. **Verification**: Document how users can verify signatures

## Appendix

### Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Run full test suite
- [ ] Build for all platforms
- [ ] Test installers on clean systems
- [ ] Sign all binaries
- [ ] Generate update manifest
- [ ] Create GitHub release
- [ ] Upload all artifacts
- [ ] Publish release
- [ ] Update website
- [ ] Send announcements
- [ ] Monitor feedback

### Useful Commands

```bash
# Check version consistency
grep -r "version" --include="*.toml" --include="*.json"

# List all artifacts
find dist -name "*.msi" -o -name "*.dmg" -o -name "*.AppImage" -o -name "*.deb" -o -name "*.rpm"

# Calculate all checksums
find dist -type f -exec sha256sum {} \; > checksums.txt

# Verify signatures (macOS)
codesign -dv --verbose=4 "Rust Video Editor.app"

# Verify signatures (Windows)
signtool verify /pa /v "RustVideoEditor.exe"
```