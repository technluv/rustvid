# 🚀 Quick Run & Deployment Guide

## 🖥️ Run Locally (Development)

### Prerequisites Check ✅
- Rust: ✅ (1.88.0)
- Node.js: ✅ (22.17.0)
- npm: ✅ (9.8.1)

### 1. Install Dependencies

```bash
cd /workspaces/tcf/rust-video-editor

# Install Rust dependencies
cargo fetch

# Install frontend dependencies
cd frontend
npm install
cd ..

# Install Tauri CLI
cargo install tauri-cli
```

### 2. Run in Development Mode

```bash
# Run the desktop app
cargo tauri dev

# Or run individual components:
# Backend only
cargo run --bin rust-video-editor

# Frontend only (in frontend/ directory)
npm run dev
```

### 3. Build for Production

```bash
# Build optimized version
cargo tauri build

# Output will be in:
# - Linux: target/release/bundle/
# - Windows: target/release/bundle/msi/
# - macOS: target/release/bundle/dmg/
```

## 🌐 Web Version (WASM)

### 1. Build Web Assembly

```bash
cd web
./scripts/build-wasm.sh

# Or manually:
wasm-pack build --target web --out-dir web/pkg
```

### 2. Run Web Version Locally

```bash
cd web
python3 -m http.server 8080
# Visit http://localhost:8080
```

## 💰 Cheapest Deployment Options

### 1. **GitHub Pages (FREE)** - Best for Demo
```bash
# Static hosting for web version
# 1. Enable GitHub Pages in repo settings
# 2. Deploy to gh-pages branch:
git checkout -b gh-pages
cp -r web/* .
git add .
git commit -m "Deploy web version"
git push origin gh-pages
# Access at: https://[username].github.io/rust-video-editor
```

### 2. **Vercel (FREE)** - Best Overall
```bash
# Install Vercel CLI
npm i -g vercel

# Deploy web version
cd web
vercel

# Features:
# - Custom domain
# - Automatic HTTPS
# - Global CDN
# - Serverless functions
# - 100GB bandwidth/month free
```

### 3. **Netlify (FREE)** - Great Alternative
```bash
# Deploy via CLI
npm i -g netlify-cli
cd web
netlify deploy --prod

# Or drag & drop at netlify.com
# Features:
# - 100GB bandwidth/month
# - Custom domain
# - Instant rollbacks
# - Form handling
```

### 4. **Cloudflare Pages (FREE)** - Best Performance
```bash
# Via Wrangler CLI
npm i -g wrangler
cd web
wrangler pages publish . --project-name=rust-video-editor

# Features:
# - Unlimited bandwidth
# - Global CDN
# - Workers for video processing
# - R2 storage for videos
```

### 5. **Railway (FREE tier)** - For Full App
```bash
# Deploy backend + frontend
railway login
railway init
railway up

# Features:
# - $5 free credit/month
# - PostgreSQL included
# - Custom domains
# - Good for API backend
```

## 🐳 Docker Deployment (Self-Host)

### 1. Build Docker Image

```bash
# Create this Dockerfile first
cat > Dockerfile << 'EOF'
FROM rust:1.88 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y \
    ffmpeg \
    libvulkan1 \
    mesa-vulkan-drivers
COPY --from=builder /app/target/release/rust-video-editor /usr/local/bin/
COPY --from=builder /app/web /var/www/html
EXPOSE 8080
CMD ["rust-video-editor", "--web-server"]
EOF

# Build
docker build -t rust-video-editor .

# Run
docker run -p 8080:8080 rust-video-editor
```

### 2. Deploy to Cloud Run (Cheap)

```bash
# Google Cloud Run (pay per use)
gcloud run deploy rust-video-editor \
    --source . \
    --platform managed \
    --allow-unauthenticated

# Costs: ~$0.00002400 per request
# Free tier: 2 million requests/month
```

## 📱 Progressive Web App (PWA)

The web version works as a PWA:
1. Visit the web version
2. Click "Install" in browser
3. Works offline with cached videos

## 🎯 Recommended Deployment Strategy

### For Personal Use:
1. **Desktop**: Download from GitHub Releases
2. **Web**: Deploy to Vercel (free, fast)

### For Business:
1. **Desktop**: Distribute via package managers
2. **Web**: Cloudflare Pages + R2 storage
3. **API**: Railway or Google Cloud Run

### For Open Source Community:
1. **Binaries**: GitHub Releases
2. **Web Demo**: GitHub Pages
3. **Package Managers**: Homebrew, Snap, etc.

## 💡 Quick Start Commands

```bash
# Fastest way to try it:
cd /workspaces/tcf/rust-video-editor

# 1. Run demo website locally
cd demo-site
python3 serve.py
# Visit http://localhost:8080

# 2. Or run the actual editor
cargo tauri dev

# 3. Or deploy web version instantly
cd web
npx vercel
```

## 🔧 Troubleshooting

### Missing FFmpeg
```bash
# Linux
sudo apt-get install ffmpeg

# macOS
brew install ffmpeg

# Windows
choco install ffmpeg
```

### Missing Vulkan (for GPU effects)
```bash
# Linux
sudo apt-get install libvulkan1 mesa-vulkan-drivers

# macOS (comes with MoltenVK)
# Windows (install GPU drivers)
```

### Build Errors
```bash
# Clear cache and rebuild
cargo clean
rm -rf frontend/node_modules
npm install
cargo build
```

## 📊 Cost Comparison

| Platform | Free Tier | Paid | Best For |
|----------|-----------|------|----------|
| GitHub Pages | Unlimited | N/A | Static demo |
| Vercel | 100GB/mo | $20/mo | Production web |
| Netlify | 100GB/mo | $19/mo | Team features |
| Cloudflare | Unlimited* | $20/mo | High traffic |
| Railway | $5 credit | $20/mo | Full stack |
| AWS S3 | 5GB | $0.023/GB | Video storage |
| Self-host | Hardware cost | Hardware | Full control |

*Cloudflare has unlimited bandwidth but limited build minutes

## 🚀 One-Line Deploy

```bash
# Quickest deployment (requires Node.js)
npx vercel web --prod --yes
```

This will deploy the web version to Vercel instantly with a unique URL!