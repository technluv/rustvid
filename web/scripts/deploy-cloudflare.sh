#!/bin/bash

# Deploy to Cloudflare Pages

set -e

echo "🚀 Deploying to Cloudflare Pages..."

# Build the web version
echo "📦 Building web application..."
npm run build:web

# Create wrangler.toml if it doesn't exist
if [ ! -f "wrangler.toml" ]; then
    cat > wrangler.toml << 'EOF'
name = "rust-video-editor"
compatibility_date = "2024-01-01"

[site]
bucket = "./web/dist"

[[r2_buckets]]
binding = "VIDEO_STORAGE"
bucket_name = "video-editor-storage"

[build]
command = "npm run build:web"

[env.production]
vars = { ENVIRONMENT = "production" }

# Headers for WASM support
[[headers]]
  for = "/*"
  [headers.values]
    "Cross-Origin-Embedder-Policy" = "require-corp"
    "Cross-Origin-Opener-Policy" = "same-origin"

[[headers]]
  for = "/wasm/*"
  [headers.values]
    "Content-Type" = "application/wasm"
    "Cache-Control" = "public, max-age=31536000, immutable"
EOF
fi

# Create _headers file for Cloudflare Pages
cat > web/dist/_headers << 'EOF'
/*
  Cross-Origin-Embedder-Policy: require-corp
  Cross-Origin-Opener-Policy: same-origin

/wasm/*
  Content-Type: application/wasm
  Cache-Control: public, max-age=31536000, immutable

/worker/*
  Service-Worker-Allowed: /
EOF

# Deploy
if command -v wrangler &> /dev/null; then
    wrangler pages publish web/dist --project-name=rust-video-editor
else
    echo "⚠️  Wrangler CLI not found. Install with: npm i -g wrangler"
    echo "Then run: wrangler pages publish web/dist --project-name=rust-video-editor"
fi