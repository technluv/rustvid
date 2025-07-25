// Service Worker for Rust Video Editor Web
// Provides offline support and caching

const CACHE_NAME = 'rust-video-editor-v1';
const RUNTIME_CACHE = 'runtime-cache-v1';

// Assets to cache on install
const STATIC_ASSETS = [
    '/',
    '/index.html',
    '/manifest.json',
    '/css/app.css',
    '/js/app.js',
    '/js/wasm-loader.js',
    '/js/video-worker.js',
    '/wasm/rust_video_editor_bg.wasm',
    '/wasm/rust_video_editor.js',
    '/icons/icon-192.png',
    '/icons/icon-512.png'
];

// Install event - cache static assets
self.addEventListener('install', (event) => {
    console.log('[SW] Installing service worker...');
    
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then(cache => {
                console.log('[SW] Caching static assets');
                return cache.addAll(STATIC_ASSETS);
            })
            .then(() => self.skipWaiting())
    );
});

// Activate event - clean up old caches
self.addEventListener('activate', (event) => {
    console.log('[SW] Activating service worker...');
    
    event.waitUntil(
        caches.keys()
            .then(cacheNames => {
                return Promise.all(
                    cacheNames
                        .filter(cacheName => {
                            return cacheName !== CACHE_NAME && 
                                   cacheName !== RUNTIME_CACHE;
                        })
                        .map(cacheName => {
                            console.log('[SW] Deleting old cache:', cacheName);
                            return caches.delete(cacheName);
                        })
                );
            })
            .then(() => self.clients.claim())
    );
});

// Fetch event - serve from cache with network fallback
self.addEventListener('fetch', (event) => {
    const { request } = event;
    const url = new URL(request.url);

    // Skip non-GET requests
    if (request.method !== 'GET') {
        return;
    }

    // Skip cross-origin requests
    if (url.origin !== location.origin) {
        return;
    }

    // Handle API requests differently
    if (url.pathname.startsWith('/api/')) {
        event.respondWith(networkFirst(request));
        return;
    }

    // Handle WASM files with cache-first strategy
    if (url.pathname.endsWith('.wasm')) {
        event.respondWith(cacheFirst(request));
        return;
    }

    // Handle video files with streaming support
    if (isVideoRequest(request)) {
        event.respondWith(handleVideoRequest(request));
        return;
    }

    // Default strategy: cache-first for static assets
    event.respondWith(cacheFirst(request));
});

// Cache-first strategy
async function cacheFirst(request) {
    const cache = await caches.open(CACHE_NAME);
    const cached = await cache.match(request);
    
    if (cached) {
        console.log('[SW] Serving from cache:', request.url);
        return cached;
    }
    
    try {
        const response = await fetch(request);
        
        if (response.ok) {
            cache.put(request, response.clone());
        }
        
        return response;
    } catch (error) {
        console.error('[SW] Fetch failed:', error);
        
        // Return offline page if available
        const offlineResponse = await cache.match('/offline.html');
        if (offlineResponse) {
            return offlineResponse;
        }
        
        throw error;
    }
}

// Network-first strategy for dynamic content
async function networkFirst(request) {
    const cache = await caches.open(RUNTIME_CACHE);
    
    try {
        const response = await fetch(request);
        
        if (response.ok) {
            cache.put(request, response.clone());
        }
        
        return response;
    } catch (error) {
        console.log('[SW] Network failed, trying cache:', request.url);
        
        const cached = await cache.match(request);
        if (cached) {
            return cached;
        }
        
        throw error;
    }
}

// Handle video requests with range support
async function handleVideoRequest(request) {
    const cache = await caches.open(RUNTIME_CACHE);
    
    // Check if we have a cached response
    const cachedResponse = await cache.match(request, { ignoreVary: true });
    
    if (!cachedResponse) {
        // Fetch from network
        return fetch(request);
    }
    
    // Handle range requests for video streaming
    const rangeHeader = request.headers.get('range');
    if (!rangeHeader) {
        return cachedResponse;
    }
    
    const rangeMatch = rangeHeader.match(/bytes=(\d+)-(\d*)/);
    if (!rangeMatch) {
        return cachedResponse;
    }
    
    const start = parseInt(rangeMatch[1], 10);
    const end = rangeMatch[2] ? parseInt(rangeMatch[2], 10) : undefined;
    
    const blob = await cachedResponse.blob();
    const slicedBlob = end ? blob.slice(start, end + 1) : blob.slice(start);
    
    const slicedResponse = new Response(slicedBlob, {
        status: 206,
        statusText: 'Partial Content',
        headers: {
            'Content-Range': `bytes ${start}-${end || blob.size - 1}/${blob.size}`,
            'Accept-Ranges': 'bytes',
            'Content-Length': slicedBlob.size,
            'Content-Type': cachedResponse.headers.get('Content-Type')
        }
    });
    
    return slicedResponse;
}

// Check if request is for video content
function isVideoRequest(request) {
    const url = new URL(request.url);
    const videoExtensions = ['.mp4', '.webm', '.ogg', '.mov', '.avi'];
    
    return videoExtensions.some(ext => url.pathname.endsWith(ext)) ||
           request.headers.get('accept')?.includes('video/');
}

// Message handler for cache management
self.addEventListener('message', (event) => {
    const { type, payload } = event.data;
    
    switch (type) {
        case 'SKIP_WAITING':
            self.skipWaiting();
            break;
            
        case 'CLEAR_CACHE':
            event.waitUntil(
                caches.keys()
                    .then(names => Promise.all(
                        names.map(name => caches.delete(name))
                    ))
                    .then(() => {
                        event.ports[0].postMessage({ 
                            type: 'CACHE_CLEARED' 
                        });
                    })
            );
            break;
            
        case 'CACHE_VIDEO':
            event.waitUntil(
                cacheVideo(payload.url)
                    .then(() => {
                        event.ports[0].postMessage({ 
                            type: 'VIDEO_CACHED',
                            url: payload.url
                        });
                    })
            );
            break;
    }
});

// Cache video for offline use
async function cacheVideo(url) {
    const cache = await caches.open(RUNTIME_CACHE);
    const response = await fetch(url);
    
    if (response.ok) {
        await cache.put(url, response);
    }
    
    return response;
}