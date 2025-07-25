#!/bin/bash

# Build script for WebAssembly version of Rust Video Editor

set -e

echo "🔧 Building WASM module..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

# Paths
RUST_DIR="../src-rust"
OUTPUT_DIR="../web/assets/wasm"
BINDGEN_DIR="../web/assets/js"

# Create output directories
mkdir -p "$OUTPUT_DIR"
mkdir -p "$BINDGEN_DIR"

# Build the WASM module
echo -e "${BLUE}Building WASM module...${NC}"
cd "$RUST_DIR"
wasm-pack build \
    --target web \
    --out-dir "$OUTPUT_DIR" \
    --no-typescript \
    --release

# Optimize WASM file
echo -e "${BLUE}Optimizing WASM binary...${NC}"
if command -v wasm-opt &> /dev/null; then
    wasm-opt -Oz \
        -o "$OUTPUT_DIR/rust_video_editor_bg_opt.wasm" \
        "$OUTPUT_DIR/rust_video_editor_bg.wasm"
    mv "$OUTPUT_DIR/rust_video_editor_bg_opt.wasm" "$OUTPUT_DIR/rust_video_editor_bg.wasm"
fi

# Generate JavaScript bindings
echo -e "${BLUE}Generating JavaScript bindings...${NC}"
cp "$OUTPUT_DIR/rust_video_editor.js" "$BINDGEN_DIR/"

# Create loader module
cat > "$BINDGEN_DIR/wasm-loader.js" << 'EOF'
// WebAssembly loader with fallback support

export class WasmLoader {
    constructor() {
        this.module = null;
        this.instance = null;
        this.memory = null;
    }

    async init(wasmPath) {
        try {
            // Check WebAssembly support
            if (!('WebAssembly' in window)) {
                throw new Error('WebAssembly not supported');
            }

            // Fetch and compile WASM module
            const response = await fetch(wasmPath);
            const bytes = await response.arrayBuffer();
            
            // Check for streaming compilation support
            if (WebAssembly.instantiateStreaming) {
                const result = await WebAssembly.instantiateStreaming(response, this.getImports());
                this.module = result.module;
                this.instance = result.instance;
            } else {
                const result = await WebAssembly.instantiate(bytes, this.getImports());
                this.module = result.module;
                this.instance = result.instance;
            }

            this.memory = this.instance.exports.memory;
            return this.instance.exports;

        } catch (error) {
            console.error('Failed to load WASM:', error);
            throw error;
        }
    }

    getImports() {
        return {
            env: {
                memory: new WebAssembly.Memory({ initial: 256, maximum: 16384 }),
                __wbindgen_throw: (ptr, len) => {
                    throw new Error(this.getString(ptr, len));
                }
            }
        };
    }

    getString(ptr, len) {
        const memory = new Uint8Array(this.memory.buffer);
        const bytes = memory.slice(ptr, ptr + len);
        return new TextDecoder('utf-8').decode(bytes);
    }
}
EOF

# Create worker for heavy computations
cat > "$BINDGEN_DIR/video-worker.js" << 'EOF'
// Web Worker for video processing

importScripts('./rust_video_editor.js');

let wasmModule = null;

self.addEventListener('message', async (event) => {
    const { type, data } = event.data;

    switch (type) {
        case 'init':
            try {
                wasmModule = await wasm_bindgen('./rust_video_editor_bg.wasm');
                self.postMessage({ type: 'ready' });
            } catch (error) {
                self.postMessage({ type: 'error', error: error.message });
            }
            break;

        case 'process':
            if (!wasmModule) {
                self.postMessage({ type: 'error', error: 'WASM not initialized' });
                return;
            }

            try {
                const result = wasmModule.process_video_frame(data.frame, data.effects);
                self.postMessage({ type: 'result', data: result });
            } catch (error) {
                self.postMessage({ type: 'error', error: error.message });
            }
            break;
    }
});
EOF

echo -e "${GREEN}✓ WASM build complete!${NC}"
echo "Output files:"
echo "  - $OUTPUT_DIR/rust_video_editor_bg.wasm"
echo "  - $BINDGEN_DIR/rust_video_editor.js"
echo "  - $BINDGEN_DIR/wasm-loader.js"
echo "  - $BINDGEN_DIR/video-worker.js"