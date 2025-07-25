#!/bin/bash

# Deploy to Vercel

set -e

echo "🚀 Deploying to Vercel..."

# Build the web version
echo "📦 Building web application..."
npm run build:web

# Create vercel.json if it doesn't exist
if [ ! -f "vercel.json" ]; then
    cat > vercel.json << 'EOF'
{
  "version": 2,
  "builds": [
    {
      "src": "web/dist/**",
      "use": "@vercel/static"
    }
  ],
  "routes": [
    {
      "src": "/wasm/(.*)",
      "dest": "/web/dist/wasm/$1",
      "headers": {
        "Content-Type": "application/wasm",
        "Cache-Control": "public, max-age=31536000, immutable"
      }
    },
    {
      "src": "/worker/(.*)",
      "dest": "/web/dist/worker/$1",
      "headers": {
        "Service-Worker-Allowed": "/"
      }
    },
    {
      "src": "/(.*)",
      "dest": "/web/dist/$1"
    }
  ],
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        {
          "key": "Cross-Origin-Embedder-Policy",
          "value": "require-corp"
        },
        {
          "key": "Cross-Origin-Opener-Policy",
          "value": "same-origin"
        }
      ]
    }
  ]
}
EOF
fi

# Deploy
if command -v vercel &> /dev/null; then
    vercel --prod
else
    echo "⚠️  Vercel CLI not found. Install with: npm i -g vercel"
    echo "Then run: vercel --prod"
fi