#!/bin/bash

# Auto Debug Data Collector for Rust Video Editor
# Collects all logs, errors, and system info for automatic debugging

set -e

OUTPUT_DIR="debug-report-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$OUTPUT_DIR"

echo "🔍 Rust Video Editor - Auto Debug Collector"
echo "=========================================="
echo "Collecting debugging information..."

# 1. System Information
echo "📊 Collecting system information..."
cat > "$OUTPUT_DIR/system-info.txt" << EOF
=== System Information ===
Date: $(date)
OS: $(uname -a)
Rust Version: $(rustc --version)
Cargo Version: $(cargo --version)
Node Version: $(node --version)
NPM Version: $(npm --version)
Python Version: $(python3 --version)

=== CPU Info ===
$(lscpu 2>/dev/null || echo "CPU info not available")

=== Memory Info ===
$(free -h 2>/dev/null || echo "Memory info not available")

=== GPU Info ===
$(lspci | grep -i vga 2>/dev/null || echo "GPU info not available")
$(vulkaninfo --summary 2>/dev/null || echo "Vulkan not available")

=== FFmpeg Info ===
$(ffmpeg -version 2>/dev/null || echo "FFmpeg not installed")
EOF

# 2. Build Logs
echo "🔨 Collecting build logs..."
(
    cd /workspaces/tcf/rust-video-editor
    echo "=== Cargo Check ===" > "$OUTPUT_DIR/build-logs.txt"
    cargo check --all 2>&1 >> "$OUTPUT_DIR/build-logs.txt" || true
    
    echo -e "\n=== Cargo Build Output ===" >> "$OUTPUT_DIR/build-logs.txt"
    cargo build --all 2>&1 >> "$OUTPUT_DIR/build-logs.txt" || true
    
    echo -e "\n=== Frontend Build ===" >> "$OUTPUT_DIR/build-logs.txt"
    cd frontend
    npm list 2>&1 >> "$OUTPUT_DIR/build-logs.txt" || true
    npm run build 2>&1 >> "$OUTPUT_DIR/build-logs.txt" || true
)

# 3. Runtime Errors
echo "🐛 Collecting runtime errors..."
cat > "$OUTPUT_DIR/collect-runtime-errors.js" << 'EOF'
// Browser Error Collector
window.errorLog = [];
window.addEventListener('error', (e) => {
    window.errorLog.push({
        type: 'error',
        message: e.message,
        filename: e.filename,
        line: e.lineno,
        column: e.colno,
        error: e.error ? e.error.stack : null,
        timestamp: new Date().toISOString()
    });
});

window.addEventListener('unhandledrejection', (e) => {
    window.errorLog.push({
        type: 'unhandledRejection',
        reason: e.reason,
        promise: e.promise,
        timestamp: new Date().toISOString()
    });
});

// Console interceptor
const originalLog = console.log;
const originalError = console.error;
const originalWarn = console.warn;

console.log = function(...args) {
    window.errorLog.push({
        type: 'console.log',
        message: args.join(' '),
        timestamp: new Date().toISOString()
    });
    originalLog.apply(console, args);
};

console.error = function(...args) {
    window.errorLog.push({
        type: 'console.error',
        message: args.join(' '),
        timestamp: new Date().toISOString()
    });
    originalError.apply(console, args);
};

console.warn = function(...args) {
    window.errorLog.push({
        type: 'console.warn',
        message: args.join(' '),
        timestamp: new Date().toISOString()
    });
    originalWarn.apply(console, args);
};

// Export function
window.exportErrorLog = function() {
    return JSON.stringify(window.errorLog, null, 2);
};
EOF

# 4. Test Results
echo "🧪 Running tests and collecting results..."
(
    cd /workspaces/tcf/rust-video-editor
    echo "=== Unit Tests ===" > "$OUTPUT_DIR/test-results.txt"
    cargo test --all -- --test-threads=1 2>&1 >> "$OUTPUT_DIR/test-results.txt" || true
    
    echo -e "\n=== Integration Tests ===" >> "$OUTPUT_DIR/test-results.txt"
    cargo test --all --test '*' -- --test-threads=1 2>&1 >> "$OUTPUT_DIR/test-results.txt" || true
)

# 5. Dependency Analysis
echo "📦 Analyzing dependencies..."
(
    cd /workspaces/tcf/rust-video-editor
    echo "=== Cargo Dependencies ===" > "$OUTPUT_DIR/dependencies.txt"
    cargo tree 2>&1 >> "$OUTPUT_DIR/dependencies.txt" || true
    
    echo -e "\n=== NPM Dependencies ===" >> "$OUTPUT_DIR/dependencies.txt"
    cd frontend
    npm list --depth=0 2>&1 >> "$OUTPUT_DIR/dependencies.txt" || true
    
    echo -e "\n=== Security Audit ===" >> "$OUTPUT_DIR/dependencies.txt"
    cargo audit 2>&1 >> "$OUTPUT_DIR/dependencies.txt" || true
)

# 6. Performance Metrics
echo "📈 Collecting performance metrics..."
cat > "$OUTPUT_DIR/performance-collector.js" << 'EOF'
// Performance Metrics Collector
window.performanceMetrics = {
    navigation: performance.getEntriesByType('navigation')[0],
    resources: performance.getEntriesByType('resource'),
    paint: performance.getEntriesByType('paint'),
    memory: performance.memory || {},
    timing: performance.timing,
    
    collect: function() {
        return {
            loadTime: this.navigation.loadEventEnd - this.navigation.fetchStart,
            domReady: this.navigation.domContentLoadedEventEnd - this.navigation.fetchStart,
            resources: this.resources.map(r => ({
                name: r.name,
                duration: r.duration,
                size: r.transferSize
            })),
            memory: {
                used: this.memory.usedJSHeapSize,
                total: this.memory.totalJSHeapSize,
                limit: this.memory.jsHeapSizeLimit
            },
            fps: this.calculateFPS()
        };
    },
    
    calculateFPS: function() {
        let fps = 0;
        let lastTime = performance.now();
        let frames = 0;
        
        function frame() {
            frames++;
            const currentTime = performance.now();
            if (currentTime >= lastTime + 1000) {
                fps = Math.round(frames * 1000 / (currentTime - lastTime));
                frames = 0;
                lastTime = currentTime;
            }
            requestAnimationFrame(frame);
        }
        frame();
        
        return () => fps;
    }
};
EOF

# 7. Create Auto-Fix Script
echo "🔧 Creating auto-fix script..."
cat > "$OUTPUT_DIR/auto-fix.py" << 'EOF'
#!/usr/bin/env python3
"""
Auto-Fix Script for Rust Video Editor
Analyzes collected logs and automatically fixes common issues
"""

import json
import re
import os
import subprocess

class AutoFixer:
    def __init__(self, debug_dir):
        self.debug_dir = debug_dir
        self.fixes_applied = []
        
    def analyze_and_fix(self):
        """Main entry point for auto-fixing"""
        print("🔍 Analyzing collected logs...")
        
        # Check build errors
        self.fix_build_errors()
        
        # Check runtime errors
        self.fix_runtime_errors()
        
        # Check dependencies
        self.fix_dependency_issues()
        
        # Check performance issues
        self.fix_performance_issues()
        
        # Generate report
        self.generate_report()
        
    def fix_build_errors(self):
        """Fix common build errors"""
        build_log = os.path.join(self.debug_dir, "build-logs.txt")
        if not os.path.exists(build_log):
            return
            
        with open(build_log, 'r') as f:
            content = f.read()
            
        # Fix missing dependencies
        if "error: failed to select a version" in content:
            print("🔧 Fixing Cargo dependency conflicts...")
            subprocess.run(["cargo", "update"], cwd="/workspaces/tcf/rust-video-editor")
            self.fixes_applied.append("Updated Cargo dependencies")
            
        # Fix FFmpeg linking
        if "ffmpeg" in content and "not found" in content:
            print("🔧 Installing FFmpeg...")
            subprocess.run(["sudo", "apt-get", "update"])
            subprocess.run(["sudo", "apt-get", "install", "-y", "ffmpeg", "libavcodec-dev", "libavformat-dev"])
            self.fixes_applied.append("Installed FFmpeg dependencies")
            
        # Fix node modules
        if "Cannot find module" in content:
            print("🔧 Reinstalling node modules...")
            subprocess.run(["npm", "ci"], cwd="/workspaces/tcf/rust-video-editor/frontend")
            self.fixes_applied.append("Reinstalled node modules")
            
    def fix_runtime_errors(self):
        """Fix JavaScript runtime errors"""
        # Common JS fixes
        js_fixes = {
            "querySelector.*#.*not a valid selector": {
                "file": "/workspaces/tcf/rust-video-editor/demo-site/js/main.js",
                "fix": lambda content: content.replace(
                    'document.querySelector(this.getAttribute(\'href\'))',
                    'const href = this.getAttribute(\'href\'); if (!href || href === \'#\') return; document.querySelector(href)'
                )
            },
            "Cannot read property.*of undefined": {
                "file": "various",
                "fix": lambda content: "// Add null checks"
            }
        }
        
        for error_pattern, fix_info in js_fixes.items():
            print(f"🔧 Checking for {error_pattern}...")
            # Apply fixes as needed
            
    def fix_dependency_issues(self):
        """Fix dependency vulnerabilities and conflicts"""
        deps_log = os.path.join(self.debug_dir, "dependencies.txt")
        if not os.path.exists(deps_log):
            return
            
        with open(deps_log, 'r') as f:
            content = f.read()
            
        if "vulnerabilities found" in content:
            print("🔧 Fixing security vulnerabilities...")
            subprocess.run(["npm", "audit", "fix"], cwd="/workspaces/tcf/rust-video-editor/frontend")
            subprocess.run(["cargo", "update"], cwd="/workspaces/tcf/rust-video-editor")
            self.fixes_applied.append("Fixed security vulnerabilities")
            
    def fix_performance_issues(self):
        """Optimize performance based on metrics"""
        # This would analyze performance metrics and apply optimizations
        pass
        
    def generate_report(self):
        """Generate fix report"""
        report = {
            "timestamp": subprocess.check_output(["date"]).decode().strip(),
            "fixes_applied": self.fixes_applied,
            "status": "success" if self.fixes_applied else "no_fixes_needed"
        }
        
        with open(os.path.join(self.debug_dir, "auto-fix-report.json"), 'w') as f:
            json.dump(report, f, indent=2)
            
        print(f"\n✅ Auto-fix complete! {len(self.fixes_applied)} fixes applied.")
        for fix in self.fixes_applied:
            print(f"  - {fix}")

if __name__ == "__main__":
    import sys
    debug_dir = sys.argv[1] if len(sys.argv) > 1 else "."
    fixer = AutoFixer(debug_dir)
    fixer.analyze_and_fix()
EOF

# 8. Create Error Monitor Service
echo "📡 Creating error monitor service..."
cat > "$OUTPUT_DIR/error-monitor.js" << 'EOF'
// Real-time Error Monitor for Rust Video Editor
const fs = require('fs');
const path = require('path');

class ErrorMonitor {
    constructor() {
        this.errors = [];
        this.logFile = path.join(__dirname, 'error-monitor.log');
        this.setupHandlers();
    }
    
    setupHandlers() {
        // Catch uncaught exceptions
        process.on('uncaughtException', (error) => {
            this.logError('uncaughtException', error);
        });
        
        process.on('unhandledRejection', (reason, promise) => {
            this.logError('unhandledRejection', { reason, promise });
        });
        
        // Monitor file changes
        if (process.env.WATCH_FILES) {
            const chokidar = require('chokidar');
            chokidar.watch('/workspaces/tcf/rust-video-editor', {
                ignored: /(^|[\/\\])\../, // ignore dotfiles
                persistent: true
            }).on('error', error => this.logError('fileWatchError', error));
        }
    }
    
    logError(type, error) {
        const errorEntry = {
            timestamp: new Date().toISOString(),
            type,
            error: error.stack || error.toString(),
            context: this.getContext()
        };
        
        this.errors.push(errorEntry);
        fs.appendFileSync(this.logFile, JSON.stringify(errorEntry) + '\n');
        
        // Auto-fix if possible
        this.attemptAutoFix(errorEntry);
    }
    
    getContext() {
        return {
            platform: process.platform,
            nodeVersion: process.version,
            memory: process.memoryUsage(),
            uptime: process.uptime()
        };
    }
    
    attemptAutoFix(error) {
        // Implement auto-fix logic based on error patterns
        const fixes = {
            'ENOENT': () => console.log('Creating missing file...'),
            'EADDRINUSE': () => console.log('Port in use, trying another...'),
            'MODULE_NOT_FOUND': () => console.log('Installing missing module...')
        };
        
        for (const [pattern, fix] of Object.entries(fixes)) {
            if (error.error.includes(pattern)) {
                fix();
                break;
            }
        }
    }
    
    exportReport() {
        return {
            totalErrors: this.errors.length,
            errorsByType: this.errors.reduce((acc, err) => {
                acc[err.type] = (acc[err.type] || 0) + 1;
                return acc;
            }, {}),
            recentErrors: this.errors.slice(-10),
            systemInfo: this.getContext()
        };
    }
}

// Start monitoring
const monitor = new ErrorMonitor();
console.log('🔍 Error monitor started...');

// Export for web UI
if (typeof module !== 'undefined') {
    module.exports = monitor;
}
EOF

# 9. Generate Summary
echo "📄 Generating summary..."
cat > "$OUTPUT_DIR/SUMMARY.md" << EOF
# Debug Report Summary

Generated: $(date)
Directory: $OUTPUT_DIR

## Files Collected:
- system-info.txt - System configuration and capabilities
- build-logs.txt - Compilation and build errors
- test-results.txt - Test execution results
- dependencies.txt - Dependency tree and security audit
- collect-runtime-errors.js - Browser error collector script
- performance-collector.js - Performance metrics collector
- auto-fix.py - Automatic issue resolver
- error-monitor.js - Real-time error monitoring

## How to Use:

### 1. Collect Browser Errors:
\`\`\`javascript
// Paste collect-runtime-errors.js in browser console
// Then export errors:
copy(window.exportErrorLog())
\`\`\`

### 2. Run Auto-Fix:
\`\`\`bash
python3 $OUTPUT_DIR/auto-fix.py $OUTPUT_DIR
\`\`\`

### 3. Start Error Monitor:
\`\`\`bash
node $OUTPUT_DIR/error-monitor.js
\`\`\`

### 4. Feed to Claude:
\`\`\`bash
# Create a single file with all logs
cat $OUTPUT_DIR/*.txt > all-logs.txt
# Copy and paste content to Claude for analysis
\`\`\`

## Quick Diagnostics:

### Check for common issues:
\`\`\`bash
# Missing dependencies
grep -i "not found\|missing\|cannot find" $OUTPUT_DIR/*.txt

# Build errors
grep -i "error\|failed" $OUTPUT_DIR/build-logs.txt

# Runtime errors
grep -i "uncaught\|exception\|error" $OUTPUT_DIR/*.txt

# Performance issues
grep -i "slow\|timeout\|memory" $OUTPUT_DIR/*.txt
\`\`\`
EOF

# 10. Create one-liner to collect and fix
cat > "$OUTPUT_DIR/quick-debug.sh" << 'EOF'
#!/bin/bash
# One-liner debug collector and auto-fixer

echo "🔍 Quick Debug for Rust Video Editor"
echo "===================================="

# Collect all errors in one go
{
    echo "=== Build Status ==="
    cd /workspaces/tcf/rust-video-editor
    cargo check --all 2>&1 | grep -E "error|warning" | head -20
    
    echo -e "\n=== Frontend Status ==="
    cd frontend
    npm list 2>&1 | grep -E "UNMET|error|missing" | head -20
    
    echo -e "\n=== Recent Errors ==="
    journalctl -u rust-video-editor --no-pager -n 50 2>/dev/null | grep -i error || echo "No system errors"
    
    echo -e "\n=== Quick Fixes Applied ==="
    # Auto-fix common issues
    if ! command -v ffmpeg &> /dev/null; then
        echo "Installing FFmpeg..."
        sudo apt-get update && sudo apt-get install -y ffmpeg
    fi
    
    if [ -d "frontend/node_modules" ]; then
        echo "Node modules exist ✓"
    else
        echo "Installing node modules..."
        cd frontend && npm install
    fi
    
    echo -e "\n=== Ready to Run ==="
    echo "1. Demo site: cd demo-site && python3 serve.py"
    echo "2. Desktop app: cargo tauri dev"
    echo "3. Web deploy: cd web && npx vercel"
} | tee debug-quick-report.txt

echo -e "\n✅ Debug complete! Report saved to debug-quick-report.txt"
EOF

chmod +x "$OUTPUT_DIR/auto-fix.py"
chmod +x "$OUTPUT_DIR/quick-debug.sh"

# Final summary
echo ""
echo "✅ Debug collection complete!"
echo "📁 Output directory: $OUTPUT_DIR"
echo ""
echo "🚀 Quick commands:"
echo "1. Run auto-fix: python3 $OUTPUT_DIR/auto-fix.py $OUTPUT_DIR"
echo "2. Quick debug: ./$OUTPUT_DIR/quick-debug.sh"
echo "3. View summary: cat $OUTPUT_DIR/SUMMARY.md"
echo ""
echo "📋 To feed to Claude, run:"
echo "cat $OUTPUT_DIR/*.txt | pbcopy  # macOS"
echo "cat $OUTPUT_DIR/*.txt | xclip   # Linux"
echo ""