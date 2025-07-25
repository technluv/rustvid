#!/bin/bash

# Test-Fix Loop Script
# Runs tests, identifies failures, and fixes them iteratively

set -e

echo "🔄 Starting Test-Fix Loop"
echo "========================"

ITERATION=0
MAX_ITERATIONS=10
LOG_DIR="test-fix-logs"
mkdir -p "$LOG_DIR"

while [ $ITERATION -lt $MAX_ITERATIONS ]; do
    ITERATION=$((ITERATION + 1))
    echo ""
    echo "📊 Iteration $ITERATION"
    echo "----------------"
    
    # Run cargo check first
    echo "🔍 Running cargo check..."
    if cargo check --all 2>&1 | tee "$LOG_DIR/check-$ITERATION.log"; then
        echo "✅ Compilation successful"
        
        # Run tests
        echo "🧪 Running tests..."
        if cargo test --all --no-fail-fast 2>&1 | tee "$LOG_DIR/test-$ITERATION.log"; then
            echo "🎉 All tests passed!"
            echo "✅ Test-Fix loop completed successfully after $ITERATION iterations"
            exit 0
        else
            echo "❌ Tests failed, analyzing..."
            
            # Extract test failures
            grep -E "(test .* FAILED|error:|failures:)" "$LOG_DIR/test-$ITERATION.log" > "$LOG_DIR/failures-$ITERATION.log" || true
            
            # Count failures
            FAILURE_COUNT=$(grep -c "FAILED" "$LOG_DIR/test-$ITERATION.log" || echo "0")
            echo "📊 Found $FAILURE_COUNT test failures"
            
            # Show summary
            echo "Failed tests:"
            grep "test .* FAILED" "$LOG_DIR/test-$ITERATION.log" || echo "No specific test failures found"
        fi
    else
        echo "❌ Compilation failed, checking errors..."
        
        # Extract compilation errors
        grep -E "(error\[E[0-9]+\]:|error:)" "$LOG_DIR/check-$ITERATION.log" > "$LOG_DIR/compile-errors-$ITERATION.log" || true
        
        echo "Compilation errors:"
        head -20 "$LOG_DIR/compile-errors-$ITERATION.log"
    fi
    
    # Add delay to prevent overwhelming the system
    sleep 2
done

echo "⚠️ Maximum iterations reached. Manual intervention required."
exit 1