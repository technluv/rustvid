#!/bin/bash
# Generate Update Manifest for Auto-Updater
# Creates a manifest file with download URLs and signatures

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"
MANIFEST_FILE="$DIST_DIR/update-manifest.json"

# Parse arguments
VERSION=""
CHANNEL="stable"
BASE_URL="https://github.com/rustvideo/editor/releases/download"
SIGN_UPDATES=false
PRIVATE_KEY=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --channel)
            CHANNEL="$2"
            shift 2
            ;;
        --base-url)
            BASE_URL="$2"
            shift 2
            ;;
        --sign)
            SIGN_UPDATES=true
            PRIVATE_KEY="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --version VERSION    Version number (required)"
            echo "  --channel CHANNEL    Release channel (default: stable)"
            echo "  --base-url URL       Base URL for downloads"
            echo "  --sign KEY_FILE      Sign manifest with private key"
            echo "  --help               Show this help message"
            exit 0
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

echo -e "${CYAN}🔄 Update Manifest Generator${NC}"
echo -e "${CYAN}===========================${NC}"

# Get version from Cargo.toml if not provided
if [ -z "$VERSION" ]; then
    VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | sed -E 's/version = "(.*)"/\1/')
    echo -e "${GREEN}📦 Detected version: $VERSION${NC}"
fi

# Function to calculate file signature
calculate_signature() {
    local file=$1
    local signature=""
    
    if [ "$SIGN_UPDATES" = true ] && [ -f "$PRIVATE_KEY" ]; then
        # Use RSA signature
        signature=$(openssl dgst -sha256 -sign "$PRIVATE_KEY" "$file" | base64 -w 0)
    else
        # Use SHA256 hash as fallback
        signature=$(sha256sum "$file" | cut -d' ' -f1)
    fi
    
    echo "$signature"
}

# Function to get file size
get_file_size() {
    local file=$1
    if [ -f "$file" ]; then
        stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null || echo "0"
    else
        echo "0"
    fi
}

# Start building manifest
echo -e "\n${YELLOW}📝 Building update manifest...${NC}"

cat > "$MANIFEST_FILE" << EOF
{
  "version": "$VERSION",
  "channel": "$CHANNEL",
  "pub_date": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "release_notes_url": "$BASE_URL/v$VERSION/RELEASE_NOTES.md",
  "platforms": {
EOF

# Process Windows builds
WINDOWS_MSI="$DIST_DIR/windows/RustVideoEditor-$VERSION-windows.msi"
WINDOWS_ZIP="$DIST_DIR/windows/RustVideoEditor-$VERSION-windows-portable.zip"

if [ -f "$WINDOWS_MSI" ]; then
    echo -e "${GREEN}✅ Found Windows MSI${NC}"
    WINDOWS_URL="$BASE_URL/v$VERSION/RustVideoEditor-$VERSION-windows-x64.msi"
    WINDOWS_SIGNATURE=$(calculate_signature "$WINDOWS_MSI")
    WINDOWS_SIZE=$(get_file_size "$WINDOWS_MSI")
    
    cat >> "$MANIFEST_FILE" << EOF
    "windows-x64": {
      "url": "$WINDOWS_URL",
      "signature": "$WINDOWS_SIGNATURE",
      "size": $WINDOWS_SIZE,
      "format": "msi",
      "notes": "Requires Windows 10 or later"
    },
EOF
fi

# Process macOS builds
MACOS_DMG="$DIST_DIR/macos/RustVideoEditor-$VERSION-macos.dmg"
MACOS_ZIP="$DIST_DIR/macos/RustVideoEditor-$VERSION-macos.zip"

if [ -f "$MACOS_DMG" ]; then
    echo -e "${GREEN}✅ Found macOS DMG${NC}"
    MACOS_URL="$BASE_URL/v$VERSION/RustVideoEditor-$VERSION-macos-universal.dmg"
    MACOS_SIGNATURE=$(calculate_signature "$MACOS_DMG")
    MACOS_SIZE=$(get_file_size "$MACOS_DMG")
    
    cat >> "$MANIFEST_FILE" << EOF
    "darwin-universal": {
      "url": "$MACOS_URL",
      "signature": "$MACOS_SIGNATURE",
      "size": $MACOS_SIZE,
      "format": "dmg",
      "notes": "Universal binary for Intel and Apple Silicon"
    },
    "darwin-x64": {
      "url": "$MACOS_URL",
      "signature": "$MACOS_SIGNATURE",
      "size": $MACOS_SIZE,
      "format": "dmg",
      "notes": "Universal binary (works on Intel Macs)"
    },
    "darwin-aarch64": {
      "url": "$MACOS_URL",
      "signature": "$MACOS_SIGNATURE",
      "size": $MACOS_SIZE,
      "format": "dmg",
      "notes": "Universal binary (works on Apple Silicon)"
    },
EOF
fi

# Process Linux builds
LINUX_APPIMAGE_X64="$DIST_DIR/linux/RustVideoEditor-$VERSION-linux-x86_64.AppImage"
LINUX_APPIMAGE_ARM64="$DIST_DIR/linux/RustVideoEditor-$VERSION-linux-aarch64.AppImage"
LINUX_DEB_X64="$DIST_DIR/linux/rustvideo_${VERSION}_amd64.deb"
LINUX_DEB_ARM64="$DIST_DIR/linux/rustvideo_${VERSION}_arm64.deb"

if [ -f "$LINUX_APPIMAGE_X64" ]; then
    echo -e "${GREEN}✅ Found Linux x64 AppImage${NC}"
    LINUX_X64_URL="$BASE_URL/v$VERSION/RustVideoEditor-$VERSION-linux-x64.AppImage"
    LINUX_X64_SIGNATURE=$(calculate_signature "$LINUX_APPIMAGE_X64")
    LINUX_X64_SIZE=$(get_file_size "$LINUX_APPIMAGE_X64")
    
    cat >> "$MANIFEST_FILE" << EOF
    "linux-x64": {
      "url": "$LINUX_X64_URL",
      "signature": "$LINUX_X64_SIGNATURE",
      "size": $LINUX_X64_SIZE,
      "format": "appimage",
      "notes": "Universal package for all Linux distributions"
    },
EOF
fi

if [ -f "$LINUX_APPIMAGE_ARM64" ]; then
    echo -e "${GREEN}✅ Found Linux ARM64 AppImage${NC}"
    LINUX_ARM64_URL="$BASE_URL/v$VERSION/RustVideoEditor-$VERSION-linux-arm64.AppImage"
    LINUX_ARM64_SIGNATURE=$(calculate_signature "$LINUX_APPIMAGE_ARM64")
    LINUX_ARM64_SIZE=$(get_file_size "$LINUX_APPIMAGE_ARM64")
    
    cat >> "$MANIFEST_FILE" << EOF
    "linux-aarch64": {
      "url": "$LINUX_ARM64_URL",
      "signature": "$LINUX_ARM64_SIGNATURE",
      "size": $LINUX_ARM64_SIZE,
      "format": "appimage",
      "notes": "For ARM64/aarch64 Linux systems"
    },
EOF
fi

# Remove trailing comma and close JSON
sed -i '$ s/,$//' "$MANIFEST_FILE" 2>/dev/null || sed -i '' '$ s/,$//' "$MANIFEST_FILE"

cat >> "$MANIFEST_FILE" << EOF
  },
  "minimum_system_version": {
    "windows": "10.0.0",
    "macos": "10.15",
    "linux": "2.17"
  },
  "urgency": "medium",
  "mandatory": false
}
EOF

echo -e "${GREEN}✅ Created update manifest: $MANIFEST_FILE${NC}"

# Validate JSON
if command -v jq &> /dev/null; then
    if jq . "$MANIFEST_FILE" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ JSON validation passed${NC}"
    else
        echo -e "${RED}❌ JSON validation failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️  jq not found, skipping JSON validation${NC}"
fi

# Generate signature for the manifest itself
if [ "$SIGN_UPDATES" = true ] && [ -f "$PRIVATE_KEY" ]; then
    echo -e "\n${YELLOW}🔐 Signing manifest...${NC}"
    
    MANIFEST_SIGNATURE=$(calculate_signature "$MANIFEST_FILE")
    echo "$MANIFEST_SIGNATURE" > "$MANIFEST_FILE.sig"
    
    echo -e "${GREEN}✅ Created manifest signature${NC}"
    
    # Also create a detached signature
    openssl dgst -sha256 -sign "$PRIVATE_KEY" -out "$MANIFEST_FILE.sig.bin" "$MANIFEST_FILE"
    echo -e "${GREEN}✅ Created binary signature${NC}"
fi

# Create a simplified current.json for quick version checks
CURRENT_FILE="$DIST_DIR/current.json"
cat > "$CURRENT_FILE" << EOF
{
  "stable": "$VERSION",
  "beta": "$VERSION",
  "nightly": "$VERSION"
}
EOF

echo -e "${GREEN}✅ Created current version file${NC}"

# Summary
echo -e "\n${CYAN}📊 Summary${NC}"
echo -e "${CYAN}=========${NC}"
echo -e "Version: $VERSION"
echo -e "Channel: $CHANNEL"
echo -e "Manifest: $MANIFEST_FILE"
echo -e "Platforms included:"

if [ -f "$WINDOWS_MSI" ]; then echo -e "  - Windows x64"; fi
if [ -f "$MACOS_DMG" ]; then echo -e "  - macOS Universal"; fi
if [ -f "$LINUX_APPIMAGE_X64" ]; then echo -e "  - Linux x64"; fi
if [ -f "$LINUX_APPIMAGE_ARM64" ]; then echo -e "  - Linux ARM64"; fi

echo -e "\n${CYAN}📤 Next Steps:${NC}"
echo -e "1. Upload manifest to your update server"
echo -e "2. Update CDN/mirror configurations"
echo -e "3. Test auto-updater with new manifest"
echo -e "4. Monitor update adoption rates"