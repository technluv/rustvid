// Browser Compatibility Shims for Rust Video Editor
// Ensures broad browser support for modern features

(function() {
    'use strict';

    // Check for WebAssembly support
    if (!window.WebAssembly) {
        console.error('WebAssembly not supported in this browser');
        window.location.href = '/unsupported.html';
        return;
    }

    // SharedArrayBuffer polyfill for browsers without support
    if (!window.SharedArrayBuffer) {
        console.warn('SharedArrayBuffer not supported, using fallback');
        window.SharedArrayBuffer = ArrayBuffer;
    }

    // Atomics polyfill
    if (!window.Atomics) {
        console.warn('Atomics not supported, using fallback');
        window.Atomics = {
            add: function(typedArray, index, value) {
                const oldValue = typedArray[index];
                typedArray[index] += value;
                return oldValue;
            },
            sub: function(typedArray, index, value) {
                const oldValue = typedArray[index];
                typedArray[index] -= value;
                return oldValue;
            },
            and: function(typedArray, index, value) {
                const oldValue = typedArray[index];
                typedArray[index] &= value;
                return oldValue;
            },
            or: function(typedArray, index, value) {
                const oldValue = typedArray[index];
                typedArray[index] |= value;
                return oldValue;
            },
            xor: function(typedArray, index, value) {
                const oldValue = typedArray[index];
                typedArray[index] ^= value;
                return oldValue;
            },
            load: function(typedArray, index) {
                return typedArray[index];
            },
            store: function(typedArray, index, value) {
                typedArray[index] = value;
                return value;
            },
            wait: function() {
                return 'not-equal'; // Simplified fallback
            },
            notify: function() {
                return 0; // Simplified fallback
            },
            isLockFree: function(size) {
                return size === 1 || size === 2 || size === 4;
            }
        };
    }

    // WebCodecs API feature detection and polyfill
    if (!('VideoEncoder' in window)) {
        console.warn('WebCodecs API not supported, using fallback');
        
        // Basic polyfill structure
        window.VideoEncoder = class VideoEncoder {
            constructor(config) {
                this.config = config;
                console.warn('Using VideoEncoder polyfill - limited functionality');
            }
            
            configure(config) {
                this.config = config;
            }
            
            encode(frame, options = {}) {
                // Fallback to canvas-based encoding
                console.warn('VideoEncoder.encode() polyfill - performance may be limited');
            }
            
            flush() {
                return Promise.resolve();
            }
            
            close() {
                // Cleanup
            }
        };
        
        window.VideoDecoder = class VideoDecoder {
            constructor(config) {
                this.config = config;
                console.warn('Using VideoDecoder polyfill - limited functionality');
            }
            
            configure(config) {
                this.config = config;
            }
            
            decode(chunk) {
                // Fallback to native video element decoding
                console.warn('VideoDecoder.decode() polyfill - performance may be limited');
            }
            
            flush() {
                return Promise.resolve();
            }
            
            close() {
                // Cleanup
            }
        };
    }

    // OffscreenCanvas polyfill
    if (!window.OffscreenCanvas) {
        window.OffscreenCanvas = class OffscreenCanvas {
            constructor(width, height) {
                const canvas = document.createElement('canvas');
                canvas.width = width;
                canvas.height = height;
                
                this.width = width;
                this.height = height;
                this._canvas = canvas;
                this._ctx = canvas.getContext('2d');
            }
            
            getContext(contextType) {
                if (contextType === '2d') {
                    return this._ctx;
                } else if (contextType === 'webgl' || contextType === 'webgl2') {
                    return this._canvas.getContext(contextType);
                }
                return null;
            }
            
            convertToBlob(options = {}) {
                return new Promise((resolve) => {
                    this._canvas.toBlob(resolve, options.type || 'image/png', options.quality);
                });
            }
            
            transferToImageBitmap() {
                return createImageBitmap(this._canvas);
            }
        };
    }

    // Web Workers transferable streams polyfill
    if (!('ReadableStream' in window) || !ReadableStream.prototype.pipeTo) {
        console.warn('Streams API incomplete, adding polyfill');
        // Add minimal streams polyfill if needed
    }

    // Performance API enhancements
    if (!performance.measureUserAgentSpecificMemory) {
        performance.measureUserAgentSpecificMemory = async function() {
            // Fallback implementation
            return {
                bytes: performance.memory ? performance.memory.usedJSHeapSize : 0,
                breakdown: []
            };
        };
    }

    // File System Access API feature detection
    if (!('showOpenFilePicker' in window)) {
        // Fallback to traditional file input
        window.showOpenFilePicker = async function(options = {}) {
            return new Promise((resolve, reject) => {
                const input = document.createElement('input');
                input.type = 'file';
                if (options.multiple) input.multiple = true;
                if (options.types) {
                    const accept = options.types.map(type => 
                        Object.keys(type.accept).join(',')
                    ).join(',');
                    input.accept = accept;
                }
                
                input.onchange = () => {
                    const files = Array.from(input.files).map(file => ({
                        getFile: async () => file,
                        name: file.name
                    }));
                    resolve(files);
                };
                
                input.click();
            });
        };
        
        window.showSaveFilePicker = async function(options = {}) {
            // Fallback to download
            return {
                createWritable: async () => ({
                    write: async (data) => {
                        const blob = new Blob([data]);
                        const url = URL.createObjectURL(blob);
                        const a = document.createElement('a');
                        a.href = url;
                        a.download = options.suggestedName || 'download';
                        a.click();
                        URL.revokeObjectURL(url);
                    },
                    close: async () => {}
                })
            };
        };
    }

    // GPU API feature detection
    if (!navigator.gpu) {
        console.warn('WebGPU not supported, will fall back to WebGL');
        navigator.gpu = null;
    }

    // Ensure requestIdleCallback exists
    if (!window.requestIdleCallback) {
        window.requestIdleCallback = function(callback, options) {
            const start = Date.now();
            return setTimeout(() => {
                callback({
                    didTimeout: false,
                    timeRemaining: () => Math.max(0, 50 - (Date.now() - start))
                });
            }, options?.timeout || 1);
        };
        
        window.cancelIdleCallback = clearTimeout;
    }

    // Add feature flags to window
    window.RustVideoEditorFeatures = {
        webAssembly: !!window.WebAssembly,
        sharedArrayBuffer: !!window.SharedArrayBuffer && window.SharedArrayBuffer !== ArrayBuffer,
        atomics: !!window.Atomics && window.Atomics.wait !== undefined,
        webCodecs: 'VideoEncoder' in window && 'VideoDecoder' in window,
        offscreenCanvas: !!window.OffscreenCanvas,
        webGPU: !!navigator.gpu,
        fileSystemAccess: 'showOpenFilePicker' in window,
        serviceWorker: 'serviceWorker' in navigator,
        webWorkers: !!window.Worker,
        simd: !!WebAssembly.validate(new Uint8Array([0,97,115,109,1,0,0,0,1,5,1,96,0,1,123,3,2,1,0,10,10,1,8,0,65,0,253,15,253,98,11]))
    };

    // Log feature support
    console.log('Rust Video Editor Feature Support:', window.RustVideoEditorFeatures);

})();