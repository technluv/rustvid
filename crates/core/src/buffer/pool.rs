//! Memory pool for efficient frame data allocation

use crate::{frame::Frame, traits::PixelFormat};
use std::sync::Arc;
use std::collections::VecDeque;

/// Frame pool for reusing frame objects
pub struct FramePool {
    capacity: usize,
    width: u32,
    height: u32,
    format: PixelFormat,
    available: VecDeque<Frame>,
    in_use_count: usize,
}

impl FramePool {
    /// Create a new frame pool
    pub fn new(capacity: usize, width: u32, height: u32, format: PixelFormat) -> Self {
        let mut available = VecDeque::with_capacity(capacity);
        
        // Pre-allocate frames
        for _ in 0..capacity {
            if let Ok(frame) = Frame::new(width, height, format, std::time::Duration::ZERO) {
                available.push_back(frame);
            }
        }
        
        Self {
            capacity,
            width,
            height,
            format,
            available,
            in_use_count: 0,
        }
    }
    
    /// Get a frame from the pool
    pub fn get(&mut self) -> Option<Frame> {
        if let Some(mut frame) = self.available.pop_front() {
            // Reset frame timestamp and metadata
            frame.timestamp = std::time::Duration::ZERO;
            frame.metadata = Default::default();
            self.in_use_count += 1;
            Some(frame)
        } else {
            None
        }
    }
    
    /// Return a frame to the pool
    pub fn return_frame(&mut self, frame: Frame) {
        // Only accept frames with matching dimensions and format
        if frame.width == self.width && frame.height == self.height && frame.format == self.format {
            self.available.push_back(frame);
            self.in_use_count = self.in_use_count.saturating_sub(1);
        }
    }
    
    /// Get the pool capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get the number of available frames
    pub fn available(&self) -> usize {
        self.available.len()
    }
    
    /// Get the number of frames in use
    pub fn in_use(&self) -> usize {
        self.in_use_count
    }
    
    /// Resize the pool capacity
    pub fn resize(&mut self, new_capacity: usize) {
        if new_capacity > self.capacity {
            // Add more frames
            let additional = new_capacity - self.capacity;
            for _ in 0..additional {
                if let Ok(frame) = Frame::new(self.width, self.height, self.format, std::time::Duration::ZERO) {
                    self.available.push_back(frame);
                }
            }
        } else if new_capacity < self.capacity {
            // Remove excess frames
            let to_remove = self.capacity - new_capacity;
            for _ in 0..to_remove {
                if self.available.len() > 0 {
                    self.available.pop_back();
                } else {
                    break;
                }
            }
        }
        self.capacity = new_capacity;
    }
}

/// Memory pool for efficient raw buffer allocation
pub struct MemoryPool {
    total_size_mb: usize,
    allocated_size: usize,
    available_buffers: Vec<Vec<u8>>,
    stats: PoolStats,
}

#[derive(Debug, Default)]
struct PoolStats {
    allocations: u64,
    deallocations: u64,
    pool_hits: u64,
    pool_misses: u64,
}

impl MemoryPool {
    pub fn new(total_size_mb: usize) -> Self {
        Self {
            total_size_mb,
            allocated_size: 0,
            available_buffers: Vec::new(),
            stats: PoolStats::default(),
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> Option<Vec<u8>> {
        self.stats.allocations += 1;
        
        // Try to find a suitable buffer
        if let Some(pos) = self.available_buffers.iter().position(|b| b.capacity() >= size) {
            let mut buffer = self.available_buffers.swap_remove(pos);
            buffer.resize(size, 0);
            self.stats.pool_hits += 1;
            return Some(buffer);
        }
        
        // Allocate new buffer if within limits
        let size_mb = size / (1024 * 1024);
        if self.allocated_size + size_mb <= self.total_size_mb {
            self.allocated_size += size_mb;
            self.stats.pool_misses += 1;
            Some(vec![0u8; size])
        } else {
            None
        }
    }
    
    pub fn deallocate(&mut self, buffer: Vec<u8>) {
        self.stats.deallocations += 1;
        if self.available_buffers.len() < 100 { // Keep a reasonable cache
            self.available_buffers.push(buffer);
        }
    }
    
    pub fn stats(&self) -> (u64, u64, f64) {
        let hit_rate = if self.stats.allocations > 0 {
            (self.stats.pool_hits as f64 / self.stats.allocations as f64) * 100.0
        } else {
            0.0
        };
        (self.stats.allocations, self.stats.deallocations, hit_rate)
    }
}

#[cfg(test)]
mod tests;