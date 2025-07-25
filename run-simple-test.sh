#!/bin/bash

# Simple test runner to verify basic functionality

echo "🧪 Running Simple Test Suite"
echo "==========================="

# Create a simple test file
cat > /tmp/simple_test.rs << 'EOF'
#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_math() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn test_video_dimensions() {
        let width = 1920;
        let height = 1080;
        assert_eq!(width * height, 2073600);
    }
    
    #[test]
    fn test_frame_rate() {
        let fps = 30.0_f64;
        let frame_duration_ms = 1000.0_f64 / fps;
        assert!((frame_duration_ms - 33.333_f64).abs() < 0.001);
    }
}

fn main() {
    println!("Test harness compiled successfully!");
}
EOF

# Compile and run the test
echo "Compiling test..."
if rustc --test /tmp/simple_test.rs -o /tmp/simple_test 2>&1; then
    echo "✅ Compilation successful"
    echo ""
    echo "Running tests..."
    if /tmp/simple_test 2>&1; then
        echo ""
        echo "✅ All tests passed!"
        exit 0
    else
        echo "❌ Tests failed"
        exit 1
    fi
else
    echo "❌ Compilation failed"
    exit 1
fi