#!/bin/bash
# macOS Build Script for Rust Video Editor
# Creates DMG installer and app bundle

set -e

# Configuration
PROJECT_NAME="RustVideoEditor"
EXECUTABLE_NAME="rust-video-editor"
APP_NAME="Rust Video Editor"
BUNDLE_ID="com.rustvideo.editor"
PUBLISHER="RustVideoEditor Team"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_ROOT/target/release"
DIST_DIR="$PROJECT_ROOT/dist/macos"
ASSETS_DIR="$PROJECT_ROOT/assets"
APP_BUNDLE="$DIST_DIR/$APP_NAME.app"

# Parse arguments
VERSION=""
SIGN_CODE=false
DEVELOPER_ID=""
NOTARIZE=false
APPLE_ID=""
APPLE_PASSWORD=""
CREATE_DMG=true
CREATE_ZIP=true

while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --sign)
            SIGN_CODE=true
            DEVELOPER_ID="$2"
            shift 2
            ;;
        --notarize)
            NOTARIZE=true
            APPLE_ID="$2"
            APPLE_PASSWORD="$3"
            shift 3
            ;;
        --no-dmg)
            CREATE_DMG=false
            shift
            ;;
        --no-zip)
            CREATE_ZIP=false
            shift
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

echo -e "${CYAN}🚀 macOS Build Script for $PROJECT_NAME${NC}"
echo -e "${CYAN}=====================================${NC}"

# Get version from Cargo.toml if not provided
if [ -z "$VERSION" ]; then
    VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | sed -E 's/version = "(.*)"/\1/')
    echo -e "${GREEN}📦 Detected version: $VERSION${NC}"
fi

# Clean and create output directories
echo -e "\n${YELLOW}🧹 Cleaning output directories...${NC}"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build the application
echo -e "\n${YELLOW}🔨 Building release version...${NC}"
cd "$PROJECT_ROOT"
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Create universal binary
echo -e "\n${YELLOW}🔧 Creating universal binary...${NC}"
lipo -create \
    "$PROJECT_ROOT/target/aarch64-apple-darwin/release/$EXECUTABLE_NAME" \
    "$PROJECT_ROOT/target/x86_64-apple-darwin/release/$EXECUTABLE_NAME" \
    -output "$BUILD_DIR/$EXECUTABLE_NAME-universal"

# Create app bundle structure
echo -e "\n${YELLOW}📁 Creating app bundle...${NC}"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"
mkdir -p "$APP_BUNDLE/Contents/Frameworks"

# Copy executable
cp "$BUILD_DIR/$EXECUTABLE_NAME-universal" "$APP_BUNDLE/Contents/MacOS/$EXECUTABLE_NAME"
chmod +x "$APP_BUNDLE/Contents/MacOS/$EXECUTABLE_NAME"

# Create Info.plist
cat > "$APP_BUNDLE/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>$APP_NAME</string>
    <key>CFBundleExecutable</key>
    <string>$EXECUTABLE_NAME</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>$BUNDLE_ID</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
    <key>NSCameraUsageDescription</key>
    <string>This app requires camera access for video recording.</string>
    <key>NSMicrophoneUsageDescription</key>
    <string>This app requires microphone access for audio recording.</string>
    <key>NSPhotoLibraryUsageDescription</key>
    <string>This app requires photo library access to import media.</string>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeExtensions</key>
            <array>
                <string>mp4</string>
                <string>mov</string>
                <string>avi</string>
                <string>mkv</string>
                <string>webm</string>
            </array>
            <key>CFBundleTypeIconFile</key>
            <string>VideoDocument</string>
            <key>CFBundleTypeName</key>
            <string>Video File</string>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>LSHandlerRank</key>
            <string>Default</string>
        </dict>
        <dict>
            <key>CFBundleTypeExtensions</key>
            <array>
                <string>rve</string>
                <string>rvproj</string>
            </array>
            <key>CFBundleTypeIconFile</key>
            <string>ProjectDocument</string>
            <key>CFBundleTypeName</key>
            <string>Rust Video Editor Project</string>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>LSHandlerRank</key>
            <string>Owner</string>
        </dict>
    </array>
</dict>
</plist>
EOF

# Copy icon
if [ -f "$ASSETS_DIR/icon.icns" ]; then
    cp "$ASSETS_DIR/icon.icns" "$APP_BUNDLE/Contents/Resources/AppIcon.icns"
else
    echo -e "${YELLOW}⚠️  Icon file not found, using default${NC}"
    # Create a simple icon using iconutil if available
    if command -v iconutil &> /dev/null; then
        ICONSET_DIR="$DIST_DIR/AppIcon.iconset"
        mkdir -p "$ICONSET_DIR"
        
        # Create icon sizes (you would normally have proper icons)
        for size in 16 32 64 128 256 512; do
            sips -z $size $size "$ASSETS_DIR/icon.png" --out "$ICONSET_DIR/icon_${size}x${size}.png" 2>/dev/null || true
            sips -z $((size*2)) $((size*2)) "$ASSETS_DIR/icon.png" --out "$ICONSET_DIR/icon_${size}x${size}@2x.png" 2>/dev/null || true
        done
        
        iconutil -c icns "$ICONSET_DIR" -o "$APP_BUNDLE/Contents/Resources/AppIcon.icns"
        rm -rf "$ICONSET_DIR"
    fi
fi

# Copy FFmpeg binaries if they exist
FFMPEG_DIR="$PROJECT_ROOT/ffmpeg/macos"
if [ -d "$FFMPEG_DIR" ]; then
    echo -e "${YELLOW}📋 Copying FFmpeg binaries...${NC}"
    cp -r "$FFMPEG_DIR"/* "$APP_BUNDLE/Contents/Frameworks/" 2>/dev/null || true
fi

# Copy other resources
if [ -d "$ASSETS_DIR" ]; then
    echo -e "${YELLOW}🎨 Copying assets...${NC}"
    cp -r "$ASSETS_DIR" "$APP_BUNDLE/Contents/Resources/"
fi

# Create entitlements file
cat > "$DIST_DIR/entitlements.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.cs.allow-jit</key>
    <true/>
    <key>com.apple.security.cs.allow-unsigned-executable-memory</key>
    <true/>
    <key>com.apple.security.device.camera</key>
    <true/>
    <key>com.apple.security.device.microphone</key>
    <true/>
    <key>com.apple.security.files.user-selected.read-write</key>
    <true/>
    <key>com.apple.security.network.client</key>
    <true/>
    <key>com.apple.security.network.server</key>
    <true/>
</dict>
</plist>
EOF

# Sign the app bundle if requested
if [ "$SIGN_CODE" = true ] && [ -n "$DEVELOPER_ID" ]; then
    echo -e "\n${YELLOW}🔐 Signing app bundle...${NC}"
    
    # Sign frameworks first
    find "$APP_BUNDLE/Contents/Frameworks" -type f -perm +111 -exec \
        codesign --force --sign "$DEVELOPER_ID" --timestamp {} \;
    
    # Sign the main executable
    codesign --force --sign "$DEVELOPER_ID" --timestamp \
        --entitlements "$DIST_DIR/entitlements.plist" \
        --options runtime \
        "$APP_BUNDLE/Contents/MacOS/$EXECUTABLE_NAME"
    
    # Sign the entire bundle
    codesign --force --sign "$DEVELOPER_ID" --timestamp \
        --entitlements "$DIST_DIR/entitlements.plist" \
        --options runtime \
        "$APP_BUNDLE"
    
    # Verify signature
    codesign --verify --deep --strict "$APP_BUNDLE"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Code signing successful${NC}"
    else
        echo -e "${RED}❌ Code signing verification failed${NC}"
        exit 1
    fi
fi

# Create ZIP archive
if [ "$CREATE_ZIP" = true ]; then
    echo -e "\n${YELLOW}📦 Creating ZIP archive...${NC}"
    cd "$DIST_DIR"
    ZIP_NAME="$PROJECT_NAME-$VERSION-macos.zip"
    ditto -c -k --keepParent "$APP_NAME.app" "$ZIP_NAME"
    
    echo -e "${GREEN}✅ Created ZIP archive: $ZIP_NAME${NC}"
    echo -e "   Size: $(du -h "$ZIP_NAME" | cut -f1)"
fi

# Create DMG
if [ "$CREATE_DMG" = true ]; then
    echo -e "\n${YELLOW}📀 Creating DMG installer...${NC}"
    DMG_NAME="$PROJECT_NAME-$VERSION-macos.dmg"
    DMG_TEMP="$DIST_DIR/temp.dmg"
    DMG_FINAL="$DIST_DIR/$DMG_NAME"
    VOLUME_NAME="$APP_NAME $VERSION"
    
    # Create temporary DMG
    hdiutil create -size 200m -fs HFS+ -volname "$VOLUME_NAME" "$DMG_TEMP"
    
    # Mount temporary DMG
    MOUNT_POINT=$(hdiutil attach -nobrowse "$DMG_TEMP" | grep "/Volumes" | awk '{print $3}')
    
    # Copy app to DMG
    cp -r "$APP_BUNDLE" "$MOUNT_POINT/"
    
    # Create Applications symlink
    ln -s /Applications "$MOUNT_POINT/Applications"
    
    # Create background and .DS_Store for nice layout
    mkdir "$MOUNT_POINT/.background"
    
    # Create a simple DMG background script
    cat > "$MOUNT_POINT/.background/dmg-background.png" << 'EOF'
# This would normally be a PNG image
# For now, we'll skip the background
EOF
    
    # Set DMG window properties using AppleScript
    osascript << EOF
tell application "Finder"
    tell disk "$VOLUME_NAME"
        open
        set current view of container window to icon view
        set toolbar visible of container window to false
        set statusbar visible of container window to false
        set the bounds of container window to {400, 100, 900, 500}
        set viewOptions to the icon view options of container window
        set arrangement of viewOptions to not arranged
        set icon size of viewOptions to 72
        set position of item "$APP_NAME.app" of container window to {150, 200}
        set position of item "Applications" of container window to {350, 200}
        update without registering applications
        delay 2
        close
    end tell
end tell
EOF
    
    # Unmount temporary DMG
    hdiutil detach "$MOUNT_POINT"
    
    # Convert to compressed DMG
    hdiutil convert "$DMG_TEMP" -format UDZO -o "$DMG_FINAL"
    rm "$DMG_TEMP"
    
    # Sign DMG if requested
    if [ "$SIGN_CODE" = true ] && [ -n "$DEVELOPER_ID" ]; then
        echo -e "${YELLOW}🔐 Signing DMG...${NC}"
        codesign --force --sign "$DEVELOPER_ID" "$DMG_FINAL"
    fi
    
    echo -e "${GREEN}✅ Created DMG installer: $DMG_NAME${NC}"
    echo -e "   Size: $(du -h "$DMG_FINAL" | cut -f1)"
fi

# Notarize if requested
if [ "$NOTARIZE" = true ] && [ -n "$APPLE_ID" ] && [ -n "$APPLE_PASSWORD" ]; then
    echo -e "\n${YELLOW}🏛️  Notarizing app...${NC}"
    
    # Create ZIP for notarization
    NOTARIZE_ZIP="$DIST_DIR/notarize.zip"
    ditto -c -k --keepParent "$APP_BUNDLE" "$NOTARIZE_ZIP"
    
    # Submit for notarization
    xcrun altool --notarize-app \
        --primary-bundle-id "$BUNDLE_ID" \
        --username "$APPLE_ID" \
        --password "$APPLE_PASSWORD" \
        --file "$NOTARIZE_ZIP"
    
    echo -e "${YELLOW}⏳ Notarization submitted. Check status with: xcrun altool --notarization-history 0 -u $APPLE_ID${NC}"
    
    # Clean up
    rm "$NOTARIZE_ZIP"
fi

# Create checksums
echo -e "\n${YELLOW}🔒 Creating checksums...${NC}"
cd "$DIST_DIR"
shasum -a 256 *.zip *.dmg 2>/dev/null > "checksums-macos.txt" || true

echo -e "${GREEN}✅ Created checksums file${NC}"

# Summary
echo -e "\n${CYAN}📊 Build Summary${NC}"
echo -e "${CYAN}================${NC}"
echo -e "Version: $VERSION"
echo -e "Output Directory: $DIST_DIR"
echo -e "\nArtifacts:"

for file in *.zip *.dmg *.txt; do
    if [ -f "$file" ]; then
        echo -e "  - $file ($(du -h "$file" | cut -f1))"
    fi
done

echo -e "\n${GREEN}✅ macOS build completed successfully!${NC}"