#!/bin/bash

# Installation Verification Script for Rust Video Editor
# This script checks if Rust Video Editor is properly installed and all dependencies are met

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ERRORS=0
WARNINGS=0

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Functions
print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
    ((ERRORS++))
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

print_info() {
    echo -e "ℹ $1"
}

check_command() {
    if command -v "$1" >/dev/null 2>&1; then
        print_success "$2 found: $(command -v "$1")"
        return 0
    else
        print_error "$2 not found"
        return 1
    fi
}

check_library() {
    if ldconfig -p | grep -q "$1"; then
        print_success "$2 library found"
        return 0
    else
        print_error "$2 library not found"
        return 1
    fi
}

# Header
echo "================================================"
echo "Rust Video Editor Installation Verification"
echo "================================================"
echo ""

# Check for Rust Video Editor executable
echo "Checking Rust Video Editor installation..."
if check_command "rust-video-editor" "Rust Video Editor"; then
    VERSION=$(rust-video-editor --version 2>/dev/null || echo "unknown")
    print_info "Version: $VERSION"
else
    print_info "Trying common installation paths..."
    COMMON_PATHS=(
        "/usr/local/bin/rust-video-editor"
        "/usr/bin/rust-video-editor"
        "$HOME/.local/bin/rust-video-editor"
        "/snap/bin/rust-video-editor"
        "/var/lib/flatpak/exports/bin/io.github.rust_video_editor"
        "$HOME/.var/app/io.github.rust_video_editor/current/active/files/bin/rust-video-editor"
    )
    
    FOUND=false
    for path in "${COMMON_PATHS[@]}"; do
        if [ -x "$path" ]; then
            print_success "Found at: $path"
            FOUND=true
            break
        fi
    done
    
    if [ "$FOUND" = false ]; then
        print_error "Rust Video Editor executable not found in common paths"
    fi
fi

echo ""
echo "Checking system dependencies..."

# Check for Vulkan
echo ""
echo "Graphics/Vulkan:"
if check_command "vulkaninfo" "Vulkan tools"; then
    GPU_INFO=$(vulkaninfo 2>/dev/null | grep "deviceName" | head -n1 | cut -d'=' -f2 | xargs)
    if [ -n "$GPU_INFO" ]; then
        print_info "GPU: $GPU_INFO"
    fi
else
    print_warning "vulkaninfo not found - GPU info unavailable"
fi

if check_library "libvulkan" "Vulkan runtime"; then
    VULKAN_VERSION=$(pkg-config --modversion vulkan 2>/dev/null || echo "unknown")
    print_info "Vulkan version: $VULKAN_VERSION"
fi

# Check for GStreamer
echo ""
echo "Media Framework:"
if check_command "gst-inspect-1.0" "GStreamer"; then
    GST_VERSION=$(gst-inspect-1.0 --version | grep "GStreamer" | awk '{print $2}')
    print_info "GStreamer version: $GST_VERSION"
    
    # Check essential GStreamer plugins
    PLUGINS=("playback" "videoconvert" "videoscale" "x264" "vpx")
    for plugin in "${PLUGINS[@]}"; do
        if gst-inspect-1.0 "$plugin" >/dev/null 2>&1; then
            print_success "GStreamer plugin '$plugin' available"
        else
            print_warning "GStreamer plugin '$plugin' not found"
        fi
    done
fi

# Check for FFmpeg libraries
echo ""
echo "FFmpeg Libraries:"
FFMPEG_LIBS=("libavcodec" "libavformat" "libavutil" "libswscale")
for lib in "${FFMPEG_LIBS[@]}"; do
    check_library "$lib" "$lib"
done

# Check for GTK (Linux only)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo ""
    echo "GUI Framework:"
    if pkg-config --exists gtk+-3.0 2>/dev/null; then
        GTK_VERSION=$(pkg-config --modversion gtk+-3.0)
        print_success "GTK3 found: version $GTK_VERSION"
    else
        print_error "GTK3 not found"
    fi
fi

# Check for audio system
echo ""
echo "Audio System:"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if command -v pulseaudio >/dev/null 2>&1 || command -v pipewire >/dev/null 2>&1; then
        if command -v pulseaudio >/dev/null 2>&1; then
            print_success "PulseAudio found"
        fi
        if command -v pipewire >/dev/null 2>&1; then
            print_success "PipeWire found"
        fi
    else
        print_warning "No audio system (PulseAudio/PipeWire) found"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    print_success "CoreAudio available (macOS)"
fi

# Check for optional features
echo ""
echo "Optional Features:"

# Hardware encoding support
if lspci 2>/dev/null | grep -qi nvidia || nvidia-smi >/dev/null 2>&1; then
    print_info "NVIDIA GPU detected - NVENC may be available"
fi

if lspci 2>/dev/null | grep -qi "amd\|radeon" || [ -d "/sys/class/drm/card0/device/driver/amdgpu" ]; then
    print_info "AMD GPU detected - AMF encoding may be available"
fi

if lspci 2>/dev/null | grep -qi intel || [ -d "/sys/class/drm/card0/device/driver/i915" ]; then
    print_info "Intel GPU detected - QuickSync may be available"
fi

# Check configuration files
echo ""
echo "Configuration:"
CONFIG_PATHS=(
    "$HOME/.config/rust-video-editor"
    "$HOME/.local/share/rust-video-editor"
    "/etc/rust-video-editor"
)

for path in "${CONFIG_PATHS[@]}"; do
    if [ -d "$path" ]; then
        print_success "Config directory found: $path"
    fi
done

# Summary
echo ""
echo "================================================"
echo "Verification Summary"
echo "================================================"

if [ $ERRORS -eq 0 ]; then
    if [ $WARNINGS -eq 0 ]; then
        echo -e "${GREEN}All checks passed!${NC} Rust Video Editor should work correctly."
    else
        echo -e "${GREEN}Installation verified${NC} with $WARNINGS warning(s)."
        echo "Some optional features may not be available."
    fi
else
    echo -e "${RED}Verification failed${NC} with $ERRORS error(s) and $WARNINGS warning(s)."
    echo "Please install missing dependencies and try again."
    echo ""
    echo "For detailed installation instructions, visit:"
    echo "https://github.com/your-org/rust-video-editor/wiki/Installation"
fi

echo ""
echo "For troubleshooting, run: rust-video-editor --diagnose"

exit $ERRORS