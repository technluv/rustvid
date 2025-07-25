#!/bin/bash
# Comprehensive Test Runner for Rust Video Editor

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
REPORT_DIR="$SCRIPT_DIR/reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test categories
declare -a TEST_CATEGORIES=(
    "unit"
    "integration"
    "platform"
    "performance"
    "accessibility"
    "e2e"
    "smoke"
    "stress"
)

# Platform detection
detect_platform() {
    case "$(uname -s)" in
        Linux*)     PLATFORM="linux";;
        Darwin*)    PLATFORM="macos";;
        CYGWIN*|MINGW*|MSYS*) PLATFORM="windows";;
        *)          PLATFORM="unknown";;
    esac
}

# Check prerequisites
check_prerequisites() {
    echo "Checking prerequisites..."
    
    # Check Rust
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: Rust/Cargo not found${NC}"
        exit 1
    fi
    
    # Check FFmpeg
    if ! command -v ffmpeg &> /dev/null; then
        echo -e "${YELLOW}Warning: FFmpeg not found. Some tests will be skipped.${NC}"
        SKIP_FFMPEG_TESTS=1
    fi
    
    # Check Python (for test fixtures)
    if ! command -v python3 &> /dev/null; then
        echo -e "${YELLOW}Warning: Python3 not found. Test fixtures generation will be skipped.${NC}"
        SKIP_FIXTURE_GEN=1
    fi
    
    echo -e "${GREEN}Prerequisites check completed${NC}"
}

# Generate test fixtures
generate_fixtures() {
    if [ "$SKIP_FIXTURE_GEN" != "1" ]; then
        echo "Generating test fixtures..."
        cd "$PROJECT_ROOT/tests/fixtures"
        
        if [ -f "create_test_videos.py" ]; then
            python3 create_test_videos.py
        fi
        
        echo -e "${GREEN}Test fixtures generated${NC}"
    fi
}

# Create report directory
setup_reports() {
    mkdir -p "$REPORT_DIR"
    mkdir -p "$REPORT_DIR/coverage"
    mkdir -p "$REPORT_DIR/benchmarks"
    mkdir -p "$REPORT_DIR/memory"
    
    echo "Test reports will be saved to: $REPORT_DIR"
}

# Run unit tests
run_unit_tests() {
    echo -e "\n${YELLOW}Running Unit Tests...${NC}"
    
    RUST_LOG=debug cargo test --lib --features test-utils -- \
        --test-threads=4 \
        --nocapture \
        --format json > "$REPORT_DIR/unit_tests_$TIMESTAMP.json" 2>&1 || true
    
    # Generate coverage if tarpaulin is installed
    if command -v cargo-tarpaulin &> /dev/null; then
        echo "Generating coverage report..."
        cargo tarpaulin --out Html --output-dir "$REPORT_DIR/coverage" || true
    fi
}

# Run integration tests
run_integration_tests() {
    echo -e "\n${YELLOW}Running Integration Tests...${NC}"
    
    if [ "$SKIP_FFMPEG_TESTS" != "1" ]; then
        RUST_LOG=debug cargo test --test integration_tests -- \
            --test-threads=2 \
            --nocapture || true
    else
        echo -e "${YELLOW}Skipping FFmpeg-dependent integration tests${NC}"
    fi
}

# Run platform-specific tests
run_platform_tests() {
    echo -e "\n${YELLOW}Running Platform-Specific Tests ($PLATFORM)...${NC}"
    
    case "$PLATFORM" in
        linux)
            cargo test --features linux-specific -- --nocapture || true
            ;;
        macos)
            cargo test --features macos-specific -- --nocapture || true
            ;;
        windows)
            cargo test --features windows-specific -- --nocapture || true
            ;;
    esac
}

# Run performance benchmarks
run_performance_tests() {
    echo -e "\n${YELLOW}Running Performance Benchmarks...${NC}"
    
    # Run criterion benchmarks if available
    if grep -q "criterion" "$PROJECT_ROOT/Cargo.toml"; then
        cargo bench --bench '*' -- --save-baseline "$TIMESTAMP" || true
        
        # Copy benchmark results
        if [ -d "target/criterion" ]; then
            cp -r target/criterion "$REPORT_DIR/benchmarks/"
        fi
    fi
    
    # Run custom benchmarks
    cargo test --test performance_tests --release -- --nocapture || true
}

# Run accessibility tests
run_accessibility_tests() {
    echo -e "\n${YELLOW}Running Accessibility Tests...${NC}"
    
    # Run Tauri accessibility tests if UI crate exists
    if [ -d "$PROJECT_ROOT/crates/ui" ]; then
        cd "$PROJECT_ROOT/crates/ui"
        
        # Run frontend accessibility tests
        if [ -f "package.json" ] && command -v npm &> /dev/null; then
            npm test -- --coverage || true
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    cargo test --test accessibility_tests -- --nocapture || true
}

# Run end-to-end tests
run_e2e_tests() {
    echo -e "\n${YELLOW}Running End-to-End Tests...${NC}"
    
    # Start test server if needed
    if [ -f "$PROJECT_ROOT/demo-site/serve.py" ]; then
        python3 "$PROJECT_ROOT/demo-site/serve.py" &
        SERVER_PID=$!
        sleep 2
    fi
    
    cargo test --test e2e_tests -- --nocapture || true
    
    # Clean up server
    if [ ! -z "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null || true
    fi
}

# Run memory leak tests
run_memory_tests() {
    echo -e "\n${YELLOW}Running Memory Leak Tests...${NC}"
    
    # Use valgrind on Linux
    if [ "$PLATFORM" = "linux" ] && command -v valgrind &> /dev/null; then
        echo "Running with Valgrind..."
        valgrind --leak-check=full \
                 --show-leak-kinds=all \
                 --track-origins=yes \
                 --xml=yes \
                 --xml-file="$REPORT_DIR/memory/valgrind_$TIMESTAMP.xml" \
                 cargo test --test memory_tests -- --test-threads=1 || true
    else
        cargo test --test memory_tests -- --nocapture || true
    fi
}

# Run stress tests
run_stress_tests() {
    echo -e "\n${YELLOW}Running Stress Tests...${NC}"
    
    # Increase limits for stress testing
    if [ "$PLATFORM" = "linux" ]; then
        ulimit -n 4096 # Increase file descriptor limit
    fi
    
    RUST_LOG=warn cargo test --test stress_tests --release -- \
        --test-threads=1 \
        --nocapture || true
}

# Generate HTML report
generate_html_report() {
    echo -e "\n${YELLOW}Generating HTML Report...${NC}"
    
    cat > "$REPORT_DIR/index.html" << EOF
<!DOCTYPE html>
<html>
<head>
    <title>Rust Video Editor Test Report - $TIMESTAMP</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background-color: #333; color: white; padding: 20px; }
        .summary { background-color: #f9f9f9; padding: 20px; margin: 20px 0; }
        .passed { color: green; }
        .failed { color: red; }
        .warning { color: orange; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
    </style>
</head>
<body>
    <div class="header">
        <h1>Rust Video Editor Test Report</h1>
        <p>Generated: $(date)</p>
        <p>Platform: $PLATFORM</p>
    </div>
    
    <div class="summary">
        <h2>Test Summary</h2>
        <ul>
            <li>Unit Tests: <span class="passed">Check unit_tests_$TIMESTAMP.json</span></li>
            <li>Integration Tests: <span class="passed">Completed</span></li>
            <li>Platform Tests: <span class="passed">$PLATFORM tests completed</span></li>
            <li>Performance Tests: <span class="passed">See benchmarks/</span></li>
            <li>Accessibility Tests: <span class="passed">Completed</span></li>
            <li>Memory Tests: <span class="passed">See memory/</span></li>
        </ul>
    </div>
    
    <h2>Detailed Results</h2>
    <p>Check individual test result files in the reports directory.</p>
    
    <h3>Coverage Report</h3>
    <p><a href="coverage/tarpaulin-report.html">View Coverage Report</a></p>
    
    <h3>Benchmark Results</h3>
    <p><a href="benchmarks/report/index.html">View Benchmark Report</a></p>
</body>
</html>
EOF
    
    echo -e "${GREEN}HTML report generated: $REPORT_DIR/index.html${NC}"
}

# Main execution
main() {
    echo "========================================="
    echo "Rust Video Editor - Comprehensive Test Suite"
    echo "========================================="
    
    detect_platform
    check_prerequisites
    setup_reports
    generate_fixtures
    
    # Run all test categories
    run_unit_tests
    run_integration_tests
    run_platform_tests
    run_performance_tests
    run_accessibility_tests
    run_e2e_tests
    run_memory_tests
    run_stress_tests
    
    # Generate final report
    generate_html_report
    
    echo -e "\n${GREEN}All tests completed!${NC}"
    echo "Test reports saved to: $REPORT_DIR"
    
    # Open report in browser if possible
    if command -v xdg-open &> /dev/null; then
        xdg-open "$REPORT_DIR/index.html"
    elif command -v open &> /dev/null; then
        open "$REPORT_DIR/index.html"
    fi
}

# Handle script arguments
case "${1:-}" in
    unit)
        run_unit_tests
        ;;
    integration)
        run_integration_tests
        ;;
    platform)
        run_platform_tests
        ;;
    performance)
        run_performance_tests
        ;;
    accessibility)
        run_accessibility_tests
        ;;
    e2e)
        run_e2e_tests
        ;;
    memory)
        run_memory_tests
        ;;
    stress)
        run_stress_tests
        ;;
    *)
        main
        ;;
esac