#!/bin/bash

echo "🖥️  Rust Video Editor - Hardware Compatibility Checker"
echo "===================================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Initialize scores
CPU_SCORE=0
RAM_SCORE=0
GPU_SCORE=0
STORAGE_SCORE=0

# Check CPU
echo "🔍 Checking CPU..."
if command -v lscpu &> /dev/null; then
    CPU_INFO=$(lscpu | grep "Model name" | sed 's/Model name://g' | xargs)
    CPU_CORES=$(nproc)
    CPU_FREQ=$(lscpu | grep "CPU MHz" | head -1 | awk '{print $3}')
    
    echo "   CPU: $CPU_INFO"
    echo "   Cores: $CPU_CORES"
    
    if [ $CPU_CORES -ge 8 ]; then
        echo -e "   ${GREEN}✅ Excellent for professional use${NC}"
        CPU_SCORE=3
    elif [ $CPU_CORES -ge 4 ]; then
        echo -e "   ${GREEN}✅ Good for 1080p/4K editing${NC}"
        CPU_SCORE=2
    elif [ $CPU_CORES -ge 2 ]; then
        echo -e "   ${YELLOW}⚠️  Minimum specs - basic editing only${NC}"
        CPU_SCORE=1
    else
        echo -e "   ${RED}❌ Below minimum requirements${NC}"
        CPU_SCORE=0
    fi
elif [ "$(uname)" == "Darwin" ]; then
    CPU_INFO=$(sysctl -n machdep.cpu.brand_string)
    CPU_CORES=$(sysctl -n hw.ncpu)
    echo "   CPU: $CPU_INFO"
    echo "   Cores: $CPU_CORES"
    
    if [[ $CPU_INFO == *"M1"* ]] || [[ $CPU_INFO == *"M2"* ]] || [[ $CPU_INFO == *"M3"* ]]; then
        echo -e "   ${GREEN}✅ Apple Silicon - Excellent performance${NC}"
        CPU_SCORE=3
    elif [ $CPU_CORES -ge 4 ]; then
        echo -e "   ${GREEN}✅ Good for video editing${NC}"
        CPU_SCORE=2
    else
        echo -e "   ${YELLOW}⚠️  Basic editing only${NC}"
        CPU_SCORE=1
    fi
fi

echo ""

# Check RAM
echo "🔍 Checking RAM..."
if [ "$(uname)" == "Linux" ]; then
    TOTAL_RAM=$(free -g | awk '/^Mem:/{print $2}')
elif [ "$(uname)" == "Darwin" ]; then
    TOTAL_RAM=$(($(sysctl -n hw.memsize) / 1024 / 1024 / 1024))
fi

echo "   Total RAM: ${TOTAL_RAM}GB"

if [ $TOTAL_RAM -ge 32 ]; then
    echo -e "   ${GREEN}✅ Excellent - Professional 4K/8K editing${NC}"
    RAM_SCORE=3
elif [ $TOTAL_RAM -ge 16 ]; then
    echo -e "   ${GREEN}✅ Recommended - Smooth 1080p/4K editing${NC}"
    RAM_SCORE=2
elif [ $TOTAL_RAM -ge 8 ]; then
    echo -e "   ${YELLOW}⚠️  Adequate - 1080p editing${NC}"
    RAM_SCORE=1
elif [ $TOTAL_RAM -ge 4 ]; then
    echo -e "   ${YELLOW}⚠️  Minimum - Basic 720p editing only${NC}"
    RAM_SCORE=1
else
    echo -e "   ${RED}❌ Below minimum (4GB required)${NC}"
    RAM_SCORE=0
fi

echo ""

# Check GPU
echo "🔍 Checking GPU..."
GPU_FOUND=false

# Check for NVIDIA
if command -v nvidia-smi &> /dev/null; then
    GPU_INFO=$(nvidia-smi --query-gpu=name --format=csv,noheader | head -1)
    GPU_MEM=$(nvidia-smi --query-gpu=memory.total --format=csv,noheader,nounits | head -1)
    echo "   NVIDIA GPU: $GPU_INFO ($GPU_MEM MB)"
    echo -e "   ${GREEN}✅ Hardware acceleration available${NC}"
    GPU_FOUND=true
    GPU_SCORE=3
fi

# Check for AMD
if lspci | grep -i "VGA.*AMD" &> /dev/null; then
    GPU_INFO=$(lspci | grep -i "VGA.*AMD" | sed 's/.*: //g')
    echo "   AMD GPU: $GPU_INFO"
    echo -e "   ${GREEN}✅ Hardware acceleration available${NC}"
    GPU_FOUND=true
    GPU_SCORE=2
fi

# Check for Intel
if lspci | grep -i "VGA.*Intel" &> /dev/null; then
    GPU_INFO=$(lspci | grep -i "VGA.*Intel" | sed 's/.*: //g')
    echo "   Intel GPU: $GPU_INFO"
    echo -e "   ${YELLOW}⚠️  Basic GPU acceleration${NC}"
    GPU_FOUND=true
    GPU_SCORE=1
fi

# macOS GPU check
if [ "$(uname)" == "Darwin" ]; then
    GPU_INFO=$(system_profiler SPDisplaysDataType | grep "Chipset Model" | sed 's/.*: //g' | head -1)
    echo "   GPU: $GPU_INFO"
    if [[ $GPU_INFO == *"M1"* ]] || [[ $GPU_INFO == *"M2"* ]] || [[ $GPU_INFO == *"M3"* ]]; then
        echo -e "   ${GREEN}✅ Apple Silicon GPU - Excellent${NC}"
        GPU_SCORE=3
    else
        echo -e "   ${GREEN}✅ Metal acceleration available${NC}"
        GPU_SCORE=2
    fi
    GPU_FOUND=true
fi

if [ "$GPU_FOUND" = false ]; then
    echo -e "   ${YELLOW}⚠️  No dedicated GPU detected - CPU rendering only${NC}"
    GPU_SCORE=0
fi

echo ""

# Check Storage
echo "🔍 Checking Storage..."
STORAGE_TYPE="Unknown"
if [ -f /sys/block/sda/queue/rotational ]; then
    ROT=$(cat /sys/block/sda/queue/rotational)
    if [ "$ROT" -eq 0 ]; then
        STORAGE_TYPE="SSD"
        echo -e "   ${GREEN}✅ SSD detected - Good for video editing${NC}"
        STORAGE_SCORE=2
    else
        STORAGE_TYPE="HDD"
        echo -e "   ${YELLOW}⚠️  HDD detected - Consider upgrading to SSD${NC}"
        STORAGE_SCORE=1
    fi
fi

# Check NVMe
if ls /dev/nvme* &> /dev/null; then
    echo -e "   ${GREEN}✅ NVMe SSD detected - Excellent performance${NC}"
    STORAGE_TYPE="NVMe"
    STORAGE_SCORE=3
fi

# Check available space
AVAILABLE_SPACE=$(df -h . | awk 'NR==2 {print $4}')
echo "   Available space: $AVAILABLE_SPACE"

echo ""

# Check installed dependencies
echo "🔍 Checking Dependencies..."
DEPS_OK=true

# FFmpeg
if command -v ffmpeg &> /dev/null; then
    echo -e "   ${GREEN}✅ FFmpeg installed${NC}"
else
    echo -e "   ${RED}❌ FFmpeg not found - Required for video processing${NC}"
    echo "      Install: sudo apt-get install ffmpeg (Linux) or brew install ffmpeg (macOS)"
    DEPS_OK=false
fi

# Vulkan
if command -v vulkaninfo &> /dev/null; then
    echo -e "   ${GREEN}✅ Vulkan installed${NC}"
elif [ "$(uname)" == "Darwin" ]; then
    echo -e "   ${GREEN}✅ Metal API available${NC}"
else
    echo -e "   ${YELLOW}⚠️  Vulkan not found - GPU effects may be limited${NC}"
fi

echo ""

# Overall assessment
echo "📊 Overall Assessment"
echo "===================="

TOTAL_SCORE=$((CPU_SCORE + RAM_SCORE + GPU_SCORE + STORAGE_SCORE))

if [ $TOTAL_SCORE -ge 10 ]; then
    echo -e "${GREEN}🎉 EXCELLENT - Your system exceeds recommended specs!${NC}"
    echo "   Perfect for professional 4K/8K video editing"
elif [ $TOTAL_SCORE -ge 7 ]; then
    echo -e "${GREEN}✅ GOOD - Your system meets recommended specs${NC}"
    echo "   Great for 1080p/4K video editing"
elif [ $TOTAL_SCORE -ge 4 ]; then
    echo -e "${YELLOW}⚠️  ADEQUATE - Your system meets minimum specs${NC}"
    echo "   Suitable for basic 720p/1080p editing"
else
    echo -e "${RED}❌ BELOW MINIMUM - Upgrade recommended${NC}"
    echo "   May experience performance issues"
fi

echo ""
echo "💡 Recommendations:"

if [ $CPU_SCORE -le 1 ]; then
    echo "   - Consider upgrading to a quad-core or better CPU"
fi

if [ $RAM_SCORE -le 1 ]; then
    echo "   - Upgrade RAM to at least 16GB for better performance"
fi

if [ $GPU_SCORE -eq 0 ]; then
    echo "   - Add a dedicated GPU for hardware acceleration"
fi

if [ $STORAGE_SCORE -le 1 ]; then
    echo "   - Upgrade to SSD for faster video loading and rendering"
fi

if [ "$DEPS_OK" = false ]; then
    echo "   - Install missing dependencies (see above)"
fi

echo ""
echo "🌐 Alternative: Try the web version at https://editor.rust-video-editor.io"
echo "   Web version has lower requirements and runs in any modern browser!"
echo ""

# Save report
REPORT_FILE="hardware-check-$(date +%Y%m%d-%H%M%S).txt"
{
    echo "Rust Video Editor Hardware Check Report"
    echo "======================================"
    echo "Date: $(date)"
    echo ""
    echo "CPU: $CPU_INFO ($CPU_CORES cores)"
    echo "RAM: ${TOTAL_RAM}GB"
    echo "GPU: ${GPU_INFO:-Not detected}"
    echo "Storage: $STORAGE_TYPE"
    echo "Available Space: $AVAILABLE_SPACE"
    echo ""
    echo "Scores:"
    echo "- CPU Score: $CPU_SCORE/3"
    echo "- RAM Score: $RAM_SCORE/3"
    echo "- GPU Score: $GPU_SCORE/3"
    echo "- Storage Score: $STORAGE_SCORE/3"
    echo "- Total Score: $TOTAL_SCORE/12"
} > "$REPORT_FILE"

echo "📄 Report saved to: $REPORT_FILE"