#!/bin/bash

# Deploy to Netlify

set -e

echo "🚀 Deploying to Netlify..."

# Build the web version
echo "📦 Building web application..."
npm run build:web

# Create netlify.toml if it doesn't exist
if [ ! -f "netlify.toml" ]; then
    cat > netlify.toml << 'EOF'
[build]
  command = "npm run build:web"
  publish = "web/dist"

[[headers]]
  for = "/*"
  [headers.values]
    Cross-Origin-Embedder-Policy = "require-corp"
    Cross-Origin-Opener-Policy = "same-origin"

[[headers]]
  for = "/wasm/*"
  [headers.values]
    Content-Type = "application/wasm"
    Cache-Control = "public, max-age=31536000, immutable"

[[headers]]
  for = "/worker/*"
  [headers.values]
    Service-Worker-Allowed = "/"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200

# Enable SharedArrayBuffer support
[[plugins]]
  package = "@netlify/plugin-csp-nonce"

  [plugins.inputs]
    policies = [
      {
        directive = "script-src",
        values = ["'self'", "'unsafe-inline'", "'unsafe-eval'", "blob:"]
      },
      {
        directive = "worker-src",
        values = ["'self'", "blob:"]
      }
    ]
EOF
fi

# Deploy
if command -v netlify &> /dev/null; then
    netlify deploy --prod --dir=web/dist
else
    echo "⚠️  Netlify CLI not found. Install with: npm i -g netlify-cli"
    echo "Then run: netlify deploy --prod --dir=web/dist"
fi