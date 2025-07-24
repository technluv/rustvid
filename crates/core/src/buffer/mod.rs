//! Frame buffer system for efficient video processing
//! 
//! This module provides a high-performance frame buffering system with:
//! - Ring buffer for sequential access
//! - LRU cache for random access
//! - Async prefetching
//! - Memory pool to reduce allocations

use std::sync::Arc;
use std::collections::VecDeque;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task::JoinHandle;

pub mod cache;
pub mod pool;

use cache::FrameCache;
use pool::MemoryPool;

/// Represents a single video frame
#[derive(Clone, Debug)]
pub struct Frame {
    /// Frame number in the video sequence
    pub frame_number: u64,
    /// Presentation timestamp in microseconds
    pub pts: i64,
    /// Frame data (raw pixels)
    pub data: Arc<Vec<u8>>,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Pixel format (e.g., RGB, YUV420)
    pub format: PixelFormat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    RGB24,
    RGBA32,
    YUV420P,
    YUV422P,
    YUV444P,
}

impl PixelFormat {
    /// Get bytes per pixel for this format
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            PixelFormat::RGB24 => 3,
            PixelFormat::RGBA32 => 4,
            PixelFormat::YUV420P => 3, // 1.5 bytes average (Y + U/4 + V/4)
            PixelFormat::YUV422P => 4, // 2 bytes average (Y + U/2 + V/2)
            PixelFormat::YUV444P => 6, // 3 bytes average (Y + U + V)
        }
    }
}

/// Configuration for the frame buffer
#[derive(Clone, Debug)]
pub struct FrameBufferConfig {
    /// Maximum number of frames in the ring buffer
    pub ring_buffer_size: usize,
    /// Maximum number of frames in the LRU cache
    pub cache_size: usize,
    /// Number of frames to prefetch ahead
    pub prefetch_count: usize,
    /// Size of the memory pool in bytes
    pub memory_pool_size: usize,
    /// Channel capacity for decoder communication
    pub channel_capacity: usize,
}

impl Default for FrameBufferConfig {
    fn default() -> Self {
        Self {
            ring_buffer_size: 30,      // ~1 second at 30fps
            cache_size: 100,           // Additional cached frames
            prefetch_count: 10,        // Prefetch 10 frames ahead
            memory_pool_size: 500 * 1024 * 1024, // 500MB pool
            channel_capacity: 50,      // Channel buffer size
        }
    }
}

/// Performance metrics for the frame buffer
#[derive(Clone, Debug, Default)]
pub struct FrameBufferMetrics {
    /// Total frames processed
    pub frames_processed: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
    /// Ring buffer hits
    pub ring_buffer_hits: u64,
    /// Ring buffer misses
    pub ring_buffer_misses: u64,
    /// Prefetch requests
    pub prefetch_requests: u64,
    /// Memory allocations from pool
    pub pool_allocations: u64,
    /// Memory allocations bypassing pool
    pub direct_allocations: u64,
}

/// Main frame buffer implementation
pub struct FrameBuffer {
    /// Configuration
    config: FrameBufferConfig,
    /// Ring buffer for sequential access
    ring_buffer: Arc<Mutex<VecDeque<Frame>>>,
    /// LRU cache for random access
    cache: Arc<FrameCache>,
    /// Memory pool for frame data
    memory_pool: Arc<MemoryPool>,
    /// Channel for receiving decoded frames
    frame_receiver: Arc<Mutex<mpsc::Receiver<Frame>>>,
    /// Channel for sending prefetch requests
    prefetch_sender: mpsc::Sender<u64>,
    /// Performance metrics
    metrics: Arc<RwLock<FrameBufferMetrics>>,
    /// Background tasks
    tasks: Vec<JoinHandle<()>>,
}

impl FrameBuffer {
    /// Create a new frame buffer
    pub fn new(
        config: FrameBufferConfig,
        frame_receiver: mpsc::Receiver<Frame>,
    ) -> (Self, mpsc::Sender<Frame>) {
        let (frame_sender, internal_receiver) = mpsc::channel(config.channel_capacity);
        let (prefetch_sender, prefetch_receiver) = mpsc::channel(config.prefetch_count * 2);
        
        let ring_buffer = Arc::new(Mutex::new(VecDeque::with_capacity(config.ring_buffer_size)));
        let cache = Arc::new(FrameCache::new(config.cache_size));
        let memory_pool = Arc::new(MemoryPool::new(config.memory_pool_size));
        let metrics = Arc::new(RwLock::new(FrameBufferMetrics::default()));
        
        let mut buffer = Self {
            config: config.clone(),
            ring_buffer: ring_buffer.clone(),
            cache: cache.clone(),
            memory_pool: memory_pool.clone(),
            frame_receiver: Arc::new(Mutex::new(frame_receiver)),
            prefetch_sender: prefetch_sender.clone(),
            metrics: metrics.clone(),
            tasks: Vec::new(),
        };
        
        // Start background tasks
        buffer.start_frame_processor(internal_receiver);
        buffer.start_prefetch_handler(prefetch_receiver);
        
        (buffer, frame_sender)
    }
    
    /// Get a frame by number
    pub async fn get_frame(&self, frame_number: u64) -> Option<Frame> {
        let mut metrics = self.metrics.write().await;
        metrics.frames_processed += 1;
        
        // Check ring buffer first (for sequential access)
        {
            let ring = self.ring_buffer.lock().await;
            if let Some(frame) = ring.iter().find(|f| f.frame_number == frame_number) {
                metrics.ring_buffer_hits += 1;
                return Some(frame.clone());
            }
            metrics.ring_buffer_misses += 1;
        }
        
        // Check cache (for random access)
        if let Some(frame) = self.cache.get(frame_number).await {
            metrics.cache_hits += 1;
            return Some(frame);
        }
        metrics.cache_misses += 1;
        
        // Request prefetch if not found
        metrics.prefetch_requests += 1;
        let _ = self.prefetch_sender.send(frame_number).await;
        
        None
    }
    
    /// Get multiple frames in a range
    pub async fn get_frame_range(&self, start: u64, end: u64) -> Vec<Frame> {
        let mut frames = Vec::new();
        
        for frame_num in start..=end {
            if let Some(frame) = self.get_frame(frame_num).await {
                frames.push(frame);
            }
        }
        
        // Trigger prefetch for the next batch
        let _ = self.prefetch_sender.send(end + 1).await;
        
        frames
    }
    
    /// Start the frame processor task
    fn start_frame_processor(&mut self, mut receiver: mpsc::Receiver<Frame>) {
        let ring_buffer = self.ring_buffer.clone();
        let cache = self.cache.clone();
        let config = self.config.clone();
        
        let task = tokio::spawn(async move {
            while let Some(frame) = receiver.recv().await {
                // Add to ring buffer
                let mut ring = ring_buffer.lock().await;
                if ring.len() >= config.ring_buffer_size {
                    // Remove oldest frame
                    if let Some(old_frame) = ring.pop_front() {
                        // Move to cache
                        cache.insert(old_frame.frame_number, old_frame).await;
                    }
                }
                ring.push_back(frame.clone());
                
                // Also add to cache for faster random access
                cache.insert(frame.frame_number, frame).await;
            }
        });
        
        self.tasks.push(task);
    }
    
    /// Start the prefetch handler task
    fn start_prefetch_handler(&mut self, mut receiver: mpsc::Receiver<u64>) {
        let prefetch_count = self.config.prefetch_count;
        let prefetch_sender = self.prefetch_sender.clone();
        
        let task = tokio::spawn(async move {
            while let Some(frame_number) = receiver.recv().await {
                // Request prefetch for subsequent frames
                for i in 1..=prefetch_count as u64 {
                    let _ = prefetch_sender.send(frame_number + i).await;
                }
            }
        });
        
        self.tasks.push(task);
    }
    
    /// Get current metrics
    pub async fn get_metrics(&self) -> FrameBufferMetrics {
        self.metrics.read().await.clone()
    }
    
    /// Reset metrics
    pub async fn reset_metrics(&self) {
        *self.metrics.write().await = FrameBufferMetrics::default();
    }
    
    /// Allocate memory from the pool
    pub async fn allocate_frame_data(&self, size: usize) -> Vec<u8> {
        let mut metrics = self.metrics.write().await;
        
        if let Some(buffer) = self.memory_pool.allocate(size).await {
            metrics.pool_allocations += 1;
            buffer
        } else {
            metrics.direct_allocations += 1;
            vec![0u8; size]
        }
    }
    
    /// Return memory to the pool
    pub async fn deallocate_frame_data(&self, data: Vec<u8>) {
        self.memory_pool.deallocate(data).await;
    }
}

/// Shutdown the frame buffer gracefully
impl Drop for FrameBuffer {
    fn drop(&mut self) {
        // Cancel all background tasks
        for task in &self.tasks {
            task.abort();
        }
    }
}

#[cfg(test)]
mod tests;