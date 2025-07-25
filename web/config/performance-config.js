// Performance optimization configuration for web deployment

export const performanceConfig = {
    // WASM optimization settings
    wasm: {
        // Memory configuration
        memory: {
            initial: 256, // Initial memory pages (16MB)
            maximum: 16384, // Maximum memory pages (1GB)
            shared: true, // Enable SharedArrayBuffer if available
            growth: 'auto' // Auto-grow memory as needed
        },
        
        // Threading configuration
        threading: {
            enabled: true,
            poolSize: navigator.hardwareConcurrency || 4,
            workerPath: '/worker/video-worker.js',
            taskQueue: {
                maxSize: 100,
                priority: 'fifo' // or 'priority'
            }
        },
        
        // SIMD optimization
        simd: {
            enabled: true,
            fallback: true // Fall back to non-SIMD if not supported
        },
        
        // Module caching
        cache: {
            enabled: true,
            storage: 'indexeddb', // or 'cache-api'
            ttl: 86400000 // 24 hours
        }
    },

    // Video processing optimization
    video: {
        // Decoder configuration
        decoder: {
            hardwareAcceleration: 'prefer-hardware',
            optimizeForLatency: true,
            powerEfficient: true
        },
        
        // Encoder configuration
        encoder: {
            hardwareAcceleration: 'prefer-hardware',
            bitrateMode: 'variable',
            latencyMode: 'quality',
            scalabilityMode: 'L1T2'
        },
        
        // Frame processing
        frameProcessing: {
            batchSize: 10,
            parallelism: 4,
            dropThreshold: 3, // Drop frames if behind by N frames
            bufferSize: 30 // Frame buffer size
        },
        
        // Resolution adaptation
        adaptation: {
            enabled: true,
            minResolution: { width: 640, height: 360 },
            maxResolution: { width: 3840, height: 2160 },
            targetFPS: 30,
            autoScale: true
        }
    },

    // Rendering optimization
    rendering: {
        // WebGL/WebGPU configuration
        gpu: {
            preferWebGPU: true,
            antialias: false, // Disable for performance
            preserveDrawingBuffer: false,
            powerPreference: 'high-performance',
            failIfMajorPerformanceCaveat: false
        },
        
        // Canvas configuration
        canvas: {
            willReadFrequently: false,
            desynchronized: true,
            alpha: false
        },
        
        // Frame timing
        timing: {
            targetFPS: 60,
            vsync: true,
            requestIdleCallback: true,
            timeSlice: 16 // ms per frame
        }
    },

    // Network optimization
    network: {
        // Chunk loading
        chunks: {
            preload: ['core', 'effects', 'codecs'],
            lazy: ['filters', 'transitions', 'advanced'],
            parallel: 3 // Max parallel chunk loads
        },
        
        // Media loading
        media: {
            preloadMetadata: true,
            streamingThreshold: 10485760, // 10MB
            bufferTarget: 30, // seconds
            adaptiveBitrate: true
        },
        
        // API requests
        api: {
            timeout: 30000,
            retry: 3,
            cache: true,
            compression: true
        }
    },

    // Memory management
    memory: {
        // Garbage collection hints
        gc: {
            threshold: 0.8, // Trigger at 80% memory usage
            interval: 30000, // Check every 30s
            aggressive: false
        },
        
        // Resource limits
        limits: {
            maxVideoSize: 2147483648, // 2GB
            maxCacheSize: 536870912, // 512MB
            maxUndoSteps: 50,
            maxLayers: 100
        },
        
        // Cleanup policies
        cleanup: {
            unusedTextures: 60000, // Clean after 1 minute
            cachedFrames: 300000, // Clean after 5 minutes
            tempFiles: 600000 // Clean after 10 minutes
        }
    },

    // Performance monitoring
    monitoring: {
        enabled: true,
        
        // Metrics to track
        metrics: {
            fps: true,
            memory: true,
            cpu: true,
            gpu: true,
            network: true
        },
        
        // Reporting
        reporting: {
            interval: 5000, // Report every 5s
            endpoint: '/api/metrics',
            batch: true,
            includeErrors: true
        },
        
        // Performance budgets
        budgets: {
            fps: { min: 24, target: 30, max: 60 },
            memory: { warn: 0.7, critical: 0.9 },
            loadTime: { target: 3000, max: 5000 }
        }
    },

    // Progressive enhancement
    progressive: {
        // Feature detection
        features: {
            checkWebGPU: true,
            checkWebCodecs: true,
            checkOffscreenCanvas: true,
            checkSharedArrayBuffer: true
        },
        
        // Fallback strategies
        fallbacks: {
            webgpu: 'webgl2',
            webgl2: 'webgl',
            webcodecs: 'mse',
            offscreencanvas: 'canvas',
            sharedarraybuffer: 'arraybuffer'
        },
        
        // Quality levels
        quality: {
            auto: true,
            levels: ['low', 'medium', 'high', 'ultra'],
            default: 'medium',
            factors: ['device', 'network', 'battery']
        }
    },

    // Initialize performance optimizations
    init() {
        // Set up performance observer
        if ('PerformanceObserver' in window) {
            const observer = new PerformanceObserver((list) => {
                for (const entry of list.getEntries()) {
                    if (entry.entryType === 'measure') {
                        console.log(`Performance: ${entry.name} - ${entry.duration}ms`);
                    }
                }
            });
            observer.observe({ entryTypes: ['measure'] });
        }

        // Configure memory pressure API if available
        if ('memory' in performance) {
            performance.memory.addEventListener('pressure', (event) => {
                if (event.level === 'critical') {
                    this.memory.gc.aggressive = true;
                    console.warn('Memory pressure critical - enabling aggressive GC');
                }
            });
        }

        // Set up network information monitoring
        if ('connection' in navigator) {
            navigator.connection.addEventListener('change', () => {
                const conn = navigator.connection;
                if (conn.effectiveType === '2g' || conn.saveData) {
                    this.progressive.quality.default = 'low';
                    console.log('Network conditions poor - switching to low quality');
                }
            });
        }

        return this;
    }
};

// Auto-initialize on import
performanceConfig.init();