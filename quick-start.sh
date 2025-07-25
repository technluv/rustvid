#!/bin/bash

echo "🎬 Rust Video Editor - Quick Start"
echo "=================================="
echo ""
echo "Choose an option:"
echo "1) View Demo Website (Instant)"
echo "2) Run Desktop App (Requires build)"
echo "3) Deploy to Web (Free hosting)"
echo "4) Build for Distribution"
echo ""
read -p "Enter choice (1-4): " choice

case $choice in
    1)
        echo "🌐 Starting demo website..."
        cd demo-site
        echo "Demo running at: http://localhost:8080"
        echo "Press Ctrl+C to stop"
        python3 serve.py
        ;;
    2)
        echo "🖥️ Building and running desktop app..."
        echo "Installing dependencies..."
        cd frontend && npm install && cd ..
        echo "Starting Tauri app..."
        cargo tauri dev
        ;;
    3)
        echo "🚀 Deploying to web..."
        echo "Choose deployment platform:"
        echo "1) Vercel (Recommended)"
        echo "2) Netlify"
        echo "3) GitHub Pages"
        read -p "Enter choice (1-3): " deploy_choice
        
        case $deploy_choice in
            1)
                echo "Deploying to Vercel..."
                cd web
                npx vercel --yes
                ;;
            2)
                echo "Deploying to Netlify..."
                cd web
                npx netlify-cli deploy --prod
                ;;
            3)
                echo "📝 To deploy to GitHub Pages:"
                echo "1. Push code to GitHub"
                echo "2. Go to Settings > Pages"
                echo "3. Select 'Deploy from branch'"
                echo "4. Choose 'gh-pages' branch"
                echo "5. Your site will be at: https://[username].github.io/rust-video-editor"
                ;;
        esac
        ;;
    4)
        echo "📦 Building for distribution..."
        echo "Building for current platform..."
        cargo tauri build
        echo ""
        echo "✅ Build complete! Find your installer in:"
        echo "   target/release/bundle/"
        ;;
esac