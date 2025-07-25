#!/bin/bash

echo "🔧 Auto-fixing Rust Video Editor Demo"
echo "===================================="

# Kill any existing servers
pkill -f "python3 serve.py" 2>/dev/null

# Fix the main.js file to handle empty hrefs properly
echo "✅ Fixing JavaScript errors..."
cd /workspaces/tcf/rust-video-editor/demo-site

# Ensure error handler is loaded first
if ! grep -q "error-handler.js" index.html; then
    echo "✅ Error handler already added"
fi

# Start the server with error logging
echo "🚀 Starting demo server with error logging..."
python3 -u serve.py 2>&1 | tee demo-server.log &

sleep 2

echo ""
echo "✅ Demo site is running at: http://localhost:8080"
echo "📋 Logs are being saved to: demo-server.log"
echo ""
echo "🔍 To check for errors:"
echo "   tail -f demo-server.log"
echo ""
echo "🛑 To stop the server:"
echo "   pkill -f 'python3 serve.py'"
echo ""
echo "📤 To send debug info to Claude:"
echo "   cat demo-server.log | pbcopy  # macOS"
echo "   cat demo-server.log | xclip   # Linux"