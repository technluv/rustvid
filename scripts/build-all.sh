#!/bin/bash
# Master Build Script for Rust Video Editor
# Builds for all platforms in parallel or sequentially

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LOG_DIR="$PROJECT_ROOT/dist/logs"

# Parse arguments
VERSION=""
PARALLEL=false
PLATFORMS=("windows" "macos" "linux")
SIGN_CODE=false
CLEAN_FIRST=true

while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --parallel)
            PARALLEL=true
            shift
            ;;
        --platform)
            PLATFORMS=("$2")
            shift 2
            ;;
        --sign)
            SIGN_CODE=true
            shift
            ;;
        --no-clean)
            CLEAN_FIRST=false
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --version VERSION    Set version number"
            echo "  --parallel           Build platforms in parallel"
            echo "  --platform PLATFORM  Build specific platform (windows/macos/linux)"
            echo "  --sign               Enable code signing"
            echo "  --no-clean           Don't clean before building"
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

echo -e "${CYAN}🚀 Master Build Script for Rust Video Editor${NC}"
echo -e "${CYAN}==========================================${NC}"

# Get version from Cargo.toml if not provided
if [ -z "$VERSION" ]; then
    VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | sed -E 's/version = "(.*)"/\1/')
    echo -e "${GREEN}📦 Detected version: $VERSION${NC}"
fi

# Create log directory
mkdir -p "$LOG_DIR"

# Clean if requested
if [ "$CLEAN_FIRST" = true ]; then
    echo -e "\n${YELLOW}🧹 Cleaning previous builds...${NC}"
    rm -rf "$PROJECT_ROOT/dist"
    cargo clean
fi

# Function to build for a specific platform
build_platform() {
    local platform=$1
    local log_file="$LOG_DIR/build-$platform.log"
    
    echo -e "${YELLOW}🔨 Building for $platform...${NC}"
    
    case $platform in
        windows)
            if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
                powershell.exe -ExecutionPolicy Bypass -File "$SCRIPT_DIR/build-windows.ps1" \
                    -Version "$VERSION" > "$log_file" 2>&1
            else
                echo -e "${YELLOW}⚠️  Windows builds require Windows OS${NC}" | tee -a "$log_file"
                return 1
            fi
            ;;
        macos)
            if [[ "$OSTYPE" == "darwin"* ]]; then
                "$SCRIPT_DIR/build-macos.sh" --version "$VERSION" > "$log_file" 2>&1
            else
                echo -e "${YELLOW}⚠️  macOS builds require macOS${NC}" | tee -a "$log_file"
                return 1
            fi
            ;;
        linux)
            "$SCRIPT_DIR/build-linux.sh" --version "$VERSION" > "$log_file" 2>&1
            ;;
        *)
            echo -e "${RED}❌ Unknown platform: $platform${NC}" | tee -a "$log_file"
            return 1
            ;;
    esac
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ $platform build completed${NC}"
        return 0
    else
        echo -e "${RED}❌ $platform build failed (see $log_file)${NC}"
        return 1
    fi
}

# Function to check platform availability
check_platform() {
    local platform=$1
    
    case $platform in
        windows)
            if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
                return 0
            fi
            ;;
        macos)
            if [[ "$OSTYPE" == "darwin"* ]]; then
                return 0
            fi
            ;;
        linux)
            return 0
            ;;
    esac
    
    return 1
}

# Make scripts executable
chmod +x "$SCRIPT_DIR"/*.sh 2>/dev/null || true

# Determine which platforms can be built
AVAILABLE_PLATFORMS=()
for platform in "${PLATFORMS[@]}"; do
    if check_platform "$platform"; then
        AVAILABLE_PLATFORMS+=("$platform")
    else
        echo -e "${YELLOW}⚠️  Skipping $platform (not available on this OS)${NC}"
    fi
done

if [ ${#AVAILABLE_PLATFORMS[@]} -eq 0 ]; then
    echo -e "${RED}❌ No platforms available to build${NC}"
    exit 1
fi

echo -e "\n${CYAN}📋 Build Configuration${NC}"
echo -e "${CYAN}=====================${NC}"
echo -e "Version: $VERSION"
echo -e "Platforms: ${AVAILABLE_PLATFORMS[*]}"
echo -e "Parallel: $PARALLEL"
echo -e "Sign Code: $SIGN_CODE"
echo -e "Clean First: $CLEAN_FIRST"

# Build platforms
echo -e "\n${CYAN}🏗️  Starting builds...${NC}"
START_TIME=$(date +%s)

if [ "$PARALLEL" = true ] && [ ${#AVAILABLE_PLATFORMS[@]} -gt 1 ]; then
    # Build in parallel
    echo -e "${YELLOW}⚡ Building platforms in parallel...${NC}"
    
    # Array to store background job PIDs
    declare -a PIDS
    
    for platform in "${AVAILABLE_PLATFORMS[@]}"; do
        build_platform "$platform" &
        PIDS+=($!)
    done
    
    # Wait for all builds to complete
    FAILED_BUILDS=()
    for i in "${!PIDS[@]}"; do
        wait "${PIDS[$i]}"
        if [ $? -ne 0 ]; then
            FAILED_BUILDS+=("${AVAILABLE_PLATFORMS[$i]}")
        fi
    done
    
    if [ ${#FAILED_BUILDS[@]} -gt 0 ]; then
        echo -e "${RED}❌ Failed builds: ${FAILED_BUILDS[*]}${NC}"
        exit 1
    fi
else
    # Build sequentially
    echo -e "${YELLOW}📦 Building platforms sequentially...${NC}"
    
    FAILED_BUILDS=()
    for platform in "${AVAILABLE_PLATFORMS[@]}"; do
        if ! build_platform "$platform"; then
            FAILED_BUILDS+=("$platform")
        fi
    done
    
    if [ ${#FAILED_BUILDS[@]} -gt 0 ]; then
        echo -e "${RED}❌ Failed builds: ${FAILED_BUILDS[*]}${NC}"
        exit 1
    fi
fi

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

# Create combined checksums file
echo -e "\n${YELLOW}🔒 Creating combined checksums...${NC}"
CHECKSUM_FILE="$PROJECT_ROOT/dist/checksums-all.txt"
echo "# Rust Video Editor v$VERSION - Build Checksums" > "$CHECKSUM_FILE"
echo "# Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")" >> "$CHECKSUM_FILE"
echo "" >> "$CHECKSUM_FILE"

for platform in "${AVAILABLE_PLATFORMS[@]}"; do
    PLATFORM_CHECKSUM="$PROJECT_ROOT/dist/$platform/checksums-$platform.txt"
    if [ -f "$PLATFORM_CHECKSUM" ]; then
        echo "## $platform" >> "$CHECKSUM_FILE"
        cat "$PLATFORM_CHECKSUM" >> "$CHECKSUM_FILE"
        echo "" >> "$CHECKSUM_FILE"
    fi
done

# Create release manifest
echo -e "\n${YELLOW}📋 Creating release manifest...${NC}"
MANIFEST_FILE="$PROJECT_ROOT/dist/manifest.json"
cat > "$MANIFEST_FILE" << EOF
{
  "version": "$VERSION",
  "build_date": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "platforms": {
EOF

FIRST=true
for platform in "${AVAILABLE_PLATFORMS[@]}"; do
    if [ "$FIRST" = true ]; then
        FIRST=false
    else
        echo "," >> "$MANIFEST_FILE"
    fi
    
    echo -n "    \"$platform\": {" >> "$MANIFEST_FILE"
    echo -n "\"artifacts\": [" >> "$MANIFEST_FILE"
    
    # List artifacts for this platform
    ARTIFACTS=""
    case $platform in
        windows)
            ARTIFACTS=$(find "$PROJECT_ROOT/dist/windows" -name "*.zip" -o -name "*.msi" 2>/dev/null | xargs -n1 basename)
            ;;
        macos)
            ARTIFACTS=$(find "$PROJECT_ROOT/dist/macos" -name "*.zip" -o -name "*.dmg" 2>/dev/null | xargs -n1 basename)
            ;;
        linux)
            ARTIFACTS=$(find "$PROJECT_ROOT/dist/linux" -name "*.AppImage" -o -name "*.deb" -o -name "*.rpm" -o -name "*.tar.gz" 2>/dev/null | xargs -n1 basename)
            ;;
    esac
    
    FIRST_ARTIFACT=true
    while IFS= read -r artifact; do
        if [ -n "$artifact" ]; then
            if [ "$FIRST_ARTIFACT" = true ]; then
                FIRST_ARTIFACT=false
            else
                echo -n ", " >> "$MANIFEST_FILE"
            fi
            echo -n "\"$artifact\"" >> "$MANIFEST_FILE"
        fi
    done <<< "$ARTIFACTS"
    
    echo -n "]}" >> "$MANIFEST_FILE"
done

cat >> "$MANIFEST_FILE" << EOF

  },
  "build_duration_seconds": $DURATION
}
EOF

# Summary
echo -e "\n${CYAN}📊 Build Summary${NC}"
echo -e "${CYAN}================${NC}"
echo -e "Version: $VERSION"
echo -e "Build Duration: $DURATION seconds"
echo -e "Platforms Built: ${AVAILABLE_PLATFORMS[*]}"
echo -e "\n${CYAN}📦 Artifacts:${NC}"

TOTAL_SIZE=0
for platform in "${AVAILABLE_PLATFORMS[@]}"; do
    echo -e "\n${YELLOW}$platform:${NC}"
    PLATFORM_DIR="$PROJECT_ROOT/dist/$platform"
    
    if [ -d "$PLATFORM_DIR" ]; then
        find "$PLATFORM_DIR" -type f \( -name "*.zip" -o -name "*.msi" -o -name "*.dmg" -o -name "*.AppImage" -o -name "*.deb" -o -name "*.rpm" -o -name "*.snap" -o -name "*.tar.gz" \) -exec ls -lh {} \; | while read -r line; do
            SIZE=$(echo "$line" | awk '{print $5}')
            FILE=$(echo "$line" | awk '{print $9}' | xargs basename)
            echo -e "  - $FILE ($SIZE)"
        done
        
        # Add to total size
        PLATFORM_SIZE=$(du -sk "$PLATFORM_DIR" | cut -f1)
        TOTAL_SIZE=$((TOTAL_SIZE + PLATFORM_SIZE))
    fi
done

echo -e "\n${CYAN}Total Size: $(echo "scale=2; $TOTAL_SIZE / 1024" | bc) MB${NC}"
echo -e "\n${CYAN}📄 Additional Files:${NC}"
echo -e "  - Checksums: $CHECKSUM_FILE"
echo -e "  - Manifest: $MANIFEST_FILE"
echo -e "  - Build Logs: $LOG_DIR/"

echo -e "\n${GREEN}✅ All builds completed successfully!${NC}"
echo -e "\n${CYAN}📤 Next Steps:${NC}"
echo -e "1. Review the artifacts in dist/"
echo -e "2. Test the packages on their respective platforms"
echo -e "3. Upload to GitHub Releases or your distribution platform"
echo -e "4. Update the auto-updater configuration with new version info"