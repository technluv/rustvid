#!/bin/bash
# Linux Build Script for Rust Video Editor
# Creates AppImage, deb, rpm, and snap packages

set -e

# Configuration
PROJECT_NAME="RustVideoEditor"
EXECUTABLE_NAME="rust-video-editor"
APP_NAME="Rust Video Editor"
APP_ID="com.rustvideo.editor"
PUBLISHER="RustVideoEditor Team"
DESCRIPTION="Professional video editing software built with Rust"
HOMEPAGE="https://github.com/rustvideo/editor"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_ROOT/target/release"
DIST_DIR="$PROJECT_ROOT/dist/linux"
ASSETS_DIR="$PROJECT_ROOT/assets"
DESKTOP_FILE="$APP_ID.desktop"

# Parse arguments
VERSION=""
CREATE_APPIMAGE=true
CREATE_DEB=true
CREATE_RPM=true
CREATE_SNAP=true
ARCH=$(uname -m)

while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --no-appimage)
            CREATE_APPIMAGE=false
            shift
            ;;
        --no-deb)
            CREATE_DEB=false
            shift
            ;;
        --no-rpm)
            CREATE_RPM=false
            shift
            ;;
        --no-snap)
            CREATE_SNAP=false
            shift
            ;;
        --arch)
            ARCH="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🚀 Linux Build Script for $PROJECT_NAME${NC}"
echo -e "${CYAN}=====================================${NC}"

# Get version from Cargo.toml if not provided
if [ -z "$VERSION" ]; then
    VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | sed -E 's/version = "(.*)"/\1/')
    echo -e "${GREEN}📦 Detected version: $VERSION${NC}"
fi

# Map architecture names
case "$ARCH" in
    x86_64|amd64)
        ARCH_DEB="amd64"
        ARCH_RPM="x86_64"
        ARCH_APPIMAGE="x86_64"
        ;;
    aarch64|arm64)
        ARCH_DEB="arm64"
        ARCH_RPM="aarch64"
        ARCH_APPIMAGE="aarch64"
        ;;
    armv7l|armhf)
        ARCH_DEB="armhf"
        ARCH_RPM="armv7hl"
        ARCH_APPIMAGE="armhf"
        ;;
    *)
        echo -e "${RED}❌ Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

# Clean and create output directories
echo -e "\n${YELLOW}🧹 Cleaning output directories...${NC}"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build the application
echo -e "\n${YELLOW}🔨 Building release version...${NC}"
cd "$PROJECT_ROOT"
cargo build --release

# Check if executable exists
if [ ! -f "$BUILD_DIR/$EXECUTABLE_NAME" ]; then
    echo -e "${RED}❌ Executable not found at: $BUILD_DIR/$EXECUTABLE_NAME${NC}"
    exit 1
fi

# Create desktop entry
echo -e "\n${YELLOW}📝 Creating desktop entry...${NC}"
cat > "$DIST_DIR/$DESKTOP_FILE" << EOF
[Desktop Entry]
Name=$APP_NAME
Comment=$DESCRIPTION
Exec=$EXECUTABLE_NAME %F
Icon=$APP_ID
Terminal=false
Type=Application
Categories=AudioVideo;Video;AudioVideoEditing;
MimeType=video/mp4;video/mpeg;video/quicktime;video/x-msvideo;video/x-matroska;video/webm;application/x-rustvideo-project;
Keywords=video;editor;editing;cut;trim;effects;
StartupNotify=true
StartupWMClass=$APP_ID
EOF

# Create AppImage
if [ "$CREATE_APPIMAGE" = true ]; then
    echo -e "\n${YELLOW}📦 Creating AppImage...${NC}"
    
    APPDIR="$DIST_DIR/AppDir"
    mkdir -p "$APPDIR/usr/bin"
    mkdir -p "$APPDIR/usr/share/applications"
    mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"
    mkdir -p "$APPDIR/usr/share/metainfo"
    mkdir -p "$APPDIR/usr/lib"
    
    # Copy executable
    cp "$BUILD_DIR/$EXECUTABLE_NAME" "$APPDIR/usr/bin/"
    chmod +x "$APPDIR/usr/bin/$EXECUTABLE_NAME"
    
    # Copy desktop file
    cp "$DIST_DIR/$DESKTOP_FILE" "$APPDIR/usr/share/applications/"
    
    # Copy icon
    if [ -f "$ASSETS_DIR/icon.png" ]; then
        cp "$ASSETS_DIR/icon.png" "$APPDIR/usr/share/icons/hicolor/256x256/apps/$APP_ID.png"
        cp "$ASSETS_DIR/icon.png" "$APPDIR/$APP_ID.png"
    else
        echo -e "${YELLOW}⚠️  Icon not found, creating placeholder${NC}"
        convert -size 256x256 xc:blue "$APPDIR/usr/share/icons/hicolor/256x256/apps/$APP_ID.png" 2>/dev/null || true
        cp "$APPDIR/usr/share/icons/hicolor/256x256/apps/$APP_ID.png" "$APPDIR/$APP_ID.png"
    fi
    
    # Create AppStream metadata
    cat > "$APPDIR/usr/share/metainfo/$APP_ID.appdata.xml" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>$APP_ID</id>
  <metadata_license>CC0-1.0</metadata_license>
  <project_license>MIT</project_license>
  <name>$APP_NAME</name>
  <summary>$DESCRIPTION</summary>
  <description>
    <p>Rust Video Editor is a professional-grade video editing application built with Rust for maximum performance and reliability.</p>
    <p>Features:</p>
    <ul>
      <li>Multi-track timeline editing</li>
      <li>Real-time preview with GPU acceleration</li>
      <li>Professional color grading tools</li>
      <li>Advanced audio editing capabilities</li>
      <li>Wide format support via FFmpeg</li>
      <li>Plugin system for extensibility</li>
    </ul>
  </description>
  <url type="homepage">$HOMEPAGE</url>
  <launchable type="desktop-id">$APP_ID.desktop</launchable>
  <provides>
    <binary>$EXECUTABLE_NAME</binary>
  </provides>
  <releases>
    <release version="$VERSION" date="$(date -I)"/>
  </releases>
  <screenshots>
    <screenshot type="default">
      <caption>Main editing interface</caption>
      <image>https://example.com/screenshot1.png</image>
    </screenshot>
  </screenshots>
</component>
EOF
    
    # Copy FFmpeg libraries if they exist
    FFMPEG_DIR="$PROJECT_ROOT/ffmpeg/linux"
    if [ -d "$FFMPEG_DIR" ]; then
        echo -e "${YELLOW}📋 Copying FFmpeg libraries...${NC}"
        cp -r "$FFMPEG_DIR"/lib* "$APPDIR/usr/lib/" 2>/dev/null || true
    fi
    
    # Create AppRun script
    cat > "$APPDIR/AppRun" << 'EOF'
#!/bin/bash
HERE="$(dirname "$(readlink -f "${0}")")"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
exec "${HERE}/usr/bin/rust-video-editor" "$@"
EOF
    chmod +x "$APPDIR/AppRun"
    
    # Download appimagetool if not present
    APPIMAGETOOL="$DIST_DIR/appimagetool-$ARCH_APPIMAGE.AppImage"
    if [ ! -f "$APPIMAGETOOL" ]; then
        echo -e "${YELLOW}⬇️  Downloading appimagetool...${NC}"
        wget -q "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-$ARCH_APPIMAGE.AppImage" \
            -O "$APPIMAGETOOL"
        chmod +x "$APPIMAGETOOL"
    fi
    
    # Create AppImage
    APPIMAGE_NAME="$PROJECT_NAME-$VERSION-linux-$ARCH_APPIMAGE.AppImage"
    "$APPIMAGETOOL" "$APPDIR" "$DIST_DIR/$APPIMAGE_NAME"
    
    if [ -f "$DIST_DIR/$APPIMAGE_NAME" ]; then
        echo -e "${GREEN}✅ Created AppImage: $APPIMAGE_NAME${NC}"
        echo -e "   Size: $(du -h "$DIST_DIR/$APPIMAGE_NAME" | cut -f1)"
    else
        echo -e "${RED}❌ Failed to create AppImage${NC}"
    fi
    
    # Clean up
    rm -rf "$APPDIR"
fi

# Create Debian package
if [ "$CREATE_DEB" = true ]; then
    echo -e "\n${YELLOW}📦 Creating Debian package...${NC}"
    
    DEB_DIR="$DIST_DIR/debian"
    DEB_NAME="${PROJECT_NAME,,}_${VERSION}_${ARCH_DEB}"
    mkdir -p "$DEB_DIR/$DEB_NAME/DEBIAN"
    mkdir -p "$DEB_DIR/$DEB_NAME/usr/bin"
    mkdir -p "$DEB_DIR/$DEB_NAME/usr/share/applications"
    mkdir -p "$DEB_DIR/$DEB_NAME/usr/share/icons/hicolor/256x256/apps"
    mkdir -p "$DEB_DIR/$DEB_NAME/usr/share/doc/${PROJECT_NAME,,}"
    
    # Copy files
    cp "$BUILD_DIR/$EXECUTABLE_NAME" "$DEB_DIR/$DEB_NAME/usr/bin/"
    chmod +x "$DEB_DIR/$DEB_NAME/usr/bin/$EXECUTABLE_NAME"
    cp "$DIST_DIR/$DESKTOP_FILE" "$DEB_DIR/$DEB_NAME/usr/share/applications/"
    
    if [ -f "$ASSETS_DIR/icon.png" ]; then
        cp "$ASSETS_DIR/icon.png" "$DEB_DIR/$DEB_NAME/usr/share/icons/hicolor/256x256/apps/$APP_ID.png"
    fi
    
    # Create control file
    INSTALLED_SIZE=$(du -sk "$DEB_DIR/$DEB_NAME" | cut -f1)
    cat > "$DEB_DIR/$DEB_NAME/DEBIAN/control" << EOF
Package: ${PROJECT_NAME,,}
Version: $VERSION
Section: video
Priority: optional
Architecture: $ARCH_DEB
Depends: libc6, libgcc1, libstdc++6, libgtk-3-0, libcairo2, libpango-1.0-0
Recommends: ffmpeg
Installed-Size: $INSTALLED_SIZE
Maintainer: $PUBLISHER <support@rustvideo.com>
Homepage: $HOMEPAGE
Description: $DESCRIPTION
 Rust Video Editor is a professional-grade video editing application
 built with Rust for maximum performance and reliability.
 .
 Features include multi-track timeline editing, real-time preview,
 color grading, audio editing, and extensive format support.
EOF
    
    # Create postinst script
    cat > "$DEB_DIR/$DEB_NAME/DEBIAN/postinst" << 'EOF'
#!/bin/sh
set -e

# Update icon cache
if [ -x /usr/bin/gtk-update-icon-cache ]; then
    /usr/bin/gtk-update-icon-cache -f -t /usr/share/icons/hicolor
fi

# Update desktop database
if [ -x /usr/bin/update-desktop-database ]; then
    /usr/bin/update-desktop-database -q
fi

exit 0
EOF
    chmod 755 "$DEB_DIR/$DEB_NAME/DEBIAN/postinst"
    
    # Create postrm script
    cat > "$DEB_DIR/$DEB_NAME/DEBIAN/postrm" << 'EOF'
#!/bin/sh
set -e

# Update icon cache
if [ -x /usr/bin/gtk-update-icon-cache ]; then
    /usr/bin/gtk-update-icon-cache -f -t /usr/share/icons/hicolor
fi

# Update desktop database
if [ -x /usr/bin/update-desktop-database ]; then
    /usr/bin/update-desktop-database -q
fi

exit 0
EOF
    chmod 755 "$DEB_DIR/$DEB_NAME/DEBIAN/postrm"
    
    # Copy documentation
    if [ -f "$PROJECT_ROOT/README.md" ]; then
        cp "$PROJECT_ROOT/README.md" "$DEB_DIR/$DEB_NAME/usr/share/doc/${PROJECT_NAME,,}/"
    fi
    if [ -f "$PROJECT_ROOT/LICENSE" ]; then
        cp "$PROJECT_ROOT/LICENSE" "$DEB_DIR/$DEB_NAME/usr/share/doc/${PROJECT_NAME,,}/copyright"
    fi
    
    # Build package
    dpkg-deb --build "$DEB_DIR/$DEB_NAME"
    mv "$DEB_DIR/$DEB_NAME.deb" "$DIST_DIR/"
    
    if [ -f "$DIST_DIR/$DEB_NAME.deb" ]; then
        echo -e "${GREEN}✅ Created Debian package: $DEB_NAME.deb${NC}"
        echo -e "   Size: $(du -h "$DIST_DIR/$DEB_NAME.deb" | cut -f1)"
    else
        echo -e "${RED}❌ Failed to create Debian package${NC}"
    fi
    
    # Clean up
    rm -rf "$DEB_DIR"
fi

# Create RPM package
if [ "$CREATE_RPM" = true ] && command -v rpmbuild &> /dev/null; then
    echo -e "\n${YELLOW}📦 Creating RPM package...${NC}"
    
    RPM_DIR="$DIST_DIR/rpm"
    mkdir -p "$RPM_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
    
    # Create tarball for RPM
    TARBALL_DIR="$RPM_DIR/SOURCES/${PROJECT_NAME,,}-$VERSION"
    mkdir -p "$TARBALL_DIR"
    cp "$BUILD_DIR/$EXECUTABLE_NAME" "$TARBALL_DIR/"
    cp "$DIST_DIR/$DESKTOP_FILE" "$TARBALL_DIR/"
    if [ -f "$ASSETS_DIR/icon.png" ]; then
        cp "$ASSETS_DIR/icon.png" "$TARBALL_DIR/$APP_ID.png"
    fi
    
    cd "$RPM_DIR/SOURCES"
    tar czf "${PROJECT_NAME,,}-$VERSION.tar.gz" "${PROJECT_NAME,,}-$VERSION"
    cd - > /dev/null
    
    # Create spec file
    cat > "$RPM_DIR/SPECS/${PROJECT_NAME,,}.spec" << EOF
Name:           ${PROJECT_NAME,,}
Version:        $VERSION
Release:        1%{?dist}
Summary:        $DESCRIPTION
License:        MIT
URL:            $HOMEPAGE
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  gcc
Requires:       gtk3, cairo, pango
Recommends:     ffmpeg

%description
Rust Video Editor is a professional-grade video editing application
built with Rust for maximum performance and reliability.

Features include multi-track timeline editing, real-time preview,
color grading, audio editing, and extensive format support.

%prep
%setup -q

%build
# Binary is pre-built

%install
rm -rf \$RPM_BUILD_ROOT
mkdir -p \$RPM_BUILD_ROOT%{_bindir}
mkdir -p \$RPM_BUILD_ROOT%{_datadir}/applications
mkdir -p \$RPM_BUILD_ROOT%{_datadir}/icons/hicolor/256x256/apps

install -m 755 $EXECUTABLE_NAME \$RPM_BUILD_ROOT%{_bindir}/
install -m 644 $DESKTOP_FILE \$RPM_BUILD_ROOT%{_datadir}/applications/
install -m 644 $APP_ID.png \$RPM_BUILD_ROOT%{_datadir}/icons/hicolor/256x256/apps/

%post
/usr/bin/update-desktop-database &> /dev/null || :
/usr/bin/gtk-update-icon-cache /usr/share/icons/hicolor &> /dev/null || :

%postun
/usr/bin/update-desktop-database &> /dev/null || :
/usr/bin/gtk-update-icon-cache /usr/share/icons/hicolor &> /dev/null || :

%files
%{_bindir}/$EXECUTABLE_NAME
%{_datadir}/applications/$DESKTOP_FILE
%{_datadir}/icons/hicolor/256x256/apps/$APP_ID.png

%changelog
* $(date "+%a %b %d %Y") $PUBLISHER <support@rustvideo.com> - $VERSION-1
- Initial RPM release
EOF
    
    # Build RPM
    rpmbuild --define "_topdir $RPM_DIR" -ba "$RPM_DIR/SPECS/${PROJECT_NAME,,}.spec"
    
    # Copy RPM to dist
    find "$RPM_DIR/RPMS" -name "*.rpm" -exec cp {} "$DIST_DIR/" \;
    
    RPM_FILE=$(find "$DIST_DIR" -name "*$VERSION*.rpm" | head -n1)
    if [ -n "$RPM_FILE" ]; then
        echo -e "${GREEN}✅ Created RPM package: $(basename "$RPM_FILE")${NC}"
        echo -e "   Size: $(du -h "$RPM_FILE" | cut -f1)"
    else
        echo -e "${RED}❌ Failed to create RPM package${NC}"
    fi
    
    # Clean up
    rm -rf "$RPM_DIR"
else
    if [ "$CREATE_RPM" = true ]; then
        echo -e "${YELLOW}⚠️  rpmbuild not found, skipping RPM creation${NC}"
    fi
fi

# Create Snap package
if [ "$CREATE_SNAP" = true ] && command -v snapcraft &> /dev/null; then
    echo -e "\n${YELLOW}📦 Creating Snap package...${NC}"
    
    SNAP_DIR="$DIST_DIR/snap"
    mkdir -p "$SNAP_DIR"
    
    # Create snapcraft.yaml
    cat > "$SNAP_DIR/snapcraft.yaml" << EOF
name: ${PROJECT_NAME,,}
version: '$VERSION'
summary: $DESCRIPTION
description: |
  Rust Video Editor is a professional-grade video editing application
  built with Rust for maximum performance and reliability.
  
  Features include multi-track timeline editing, real-time preview,
  color grading, audio editing, and extensive format support.

base: core20
confinement: strict
grade: stable

apps:
  ${PROJECT_NAME,,}:
    command: bin/$EXECUTABLE_NAME
    desktop: share/applications/$DESKTOP_FILE
    plugs:
      - home
      - removable-media
      - desktop
      - desktop-legacy
      - x11
      - wayland
      - opengl
      - audio-playback
      - audio-record
      - camera
      - network
      - network-bind

parts:
  ${PROJECT_NAME,,}:
    plugin: dump
    source: .
    stage-packages:
      - libgtk-3-0
      - libcairo2
      - libpango-1.0-0
      - ffmpeg
    override-build: |
      mkdir -p \$SNAPCRAFT_PART_INSTALL/bin
      mkdir -p \$SNAPCRAFT_PART_INSTALL/share/applications
      mkdir -p \$SNAPCRAFT_PART_INSTALL/share/icons/hicolor/256x256/apps
      
      cp $BUILD_DIR/$EXECUTABLE_NAME \$SNAPCRAFT_PART_INSTALL/bin/
      cp $DIST_DIR/$DESKTOP_FILE \$SNAPCRAFT_PART_INSTALL/share/applications/
      
      if [ -f "$ASSETS_DIR/icon.png" ]; then
        cp "$ASSETS_DIR/icon.png" \$SNAPCRAFT_PART_INSTALL/share/icons/hicolor/256x256/apps/$APP_ID.png
      fi
EOF
    
    # Build snap
    cd "$SNAP_DIR"
    snapcraft --destructive-mode
    
    SNAP_FILE=$(find "$SNAP_DIR" -name "*.snap" | head -n1)
    if [ -n "$SNAP_FILE" ]; then
        mv "$SNAP_FILE" "$DIST_DIR/"
        echo -e "${GREEN}✅ Created Snap package: $(basename "$SNAP_FILE")${NC}"
        echo -e "   Size: $(du -h "$DIST_DIR/$(basename "$SNAP_FILE")" | cut -f1)"
    else
        echo -e "${RED}❌ Failed to create Snap package${NC}"
    fi
    
    cd - > /dev/null
    rm -rf "$SNAP_DIR"
else
    if [ "$CREATE_SNAP" = true ]; then
        echo -e "${YELLOW}⚠️  snapcraft not found, skipping Snap creation${NC}"
    fi
fi

# Create portable tarball
echo -e "\n${YELLOW}📦 Creating portable tarball...${NC}"
PORTABLE_DIR="$DIST_DIR/${PROJECT_NAME,,}-$VERSION-linux-$ARCH"
mkdir -p "$PORTABLE_DIR"

cp "$BUILD_DIR/$EXECUTABLE_NAME" "$PORTABLE_DIR/"
cp "$DIST_DIR/$DESKTOP_FILE" "$PORTABLE_DIR/"

if [ -f "$PROJECT_ROOT/README.md" ]; then
    cp "$PROJECT_ROOT/README.md" "$PORTABLE_DIR/"
fi
if [ -f "$PROJECT_ROOT/LICENSE" ]; then
    cp "$PROJECT_ROOT/LICENSE" "$PORTABLE_DIR/"
fi

# Create run script
cat > "$PORTABLE_DIR/run.sh" << 'EOF'
#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export LD_LIBRARY_PATH="$SCRIPT_DIR/lib:$LD_LIBRARY_PATH"
exec "$SCRIPT_DIR/rust-video-editor" "$@"
EOF
chmod +x "$PORTABLE_DIR/run.sh"

# Copy FFmpeg if available
if [ -d "$FFMPEG_DIR" ]; then
    mkdir -p "$PORTABLE_DIR/lib"
    cp -r "$FFMPEG_DIR"/lib* "$PORTABLE_DIR/lib/" 2>/dev/null || true
fi

cd "$DIST_DIR"
tar czf "${PROJECT_NAME,,}-$VERSION-linux-$ARCH.tar.gz" "$(basename "$PORTABLE_DIR")"
rm -rf "$PORTABLE_DIR"

echo -e "${GREEN}✅ Created portable tarball: ${PROJECT_NAME,,}-$VERSION-linux-$ARCH.tar.gz${NC}"
echo -e "   Size: $(du -h "${PROJECT_NAME,,}-$VERSION-linux-$ARCH.tar.gz" | cut -f1)"

# Create checksums
echo -e "\n${YELLOW}🔒 Creating checksums...${NC}"
cd "$DIST_DIR"
sha256sum *.AppImage *.deb *.rpm *.snap *.tar.gz 2>/dev/null > "checksums-linux.txt" || true

echo -e "${GREEN}✅ Created checksums file${NC}"

# Summary
echo -e "\n${CYAN}📊 Build Summary${NC}"
echo -e "${CYAN}================${NC}"
echo -e "Version: $VERSION"
echo -e "Architecture: $ARCH"
echo -e "Output Directory: $DIST_DIR"
echo -e "\nArtifacts:"

for file in *.AppImage *.deb *.rpm *.snap *.tar.gz *.txt; do
    if [ -f "$file" ]; then
        echo -e "  - $file ($(du -h "$file" | cut -f1))"
    fi
done

echo -e "\n${GREEN}✅ Linux build completed successfully!${NC}"