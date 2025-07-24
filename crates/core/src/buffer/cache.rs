//! Frame cache implementation for efficient frame storage

use crate::frame::Frame;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

/// Key for cache entries
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct CacheKey {
    source: String,
    frame_index: usize,
}

/// Cache entry with metadata
struct CacheEntry {
    frame: Frame,
    size_bytes: usize,
    last_accessed: Instant,
}

/// Frame cache with LRU eviction
pub struct FrameCache {
    max_size_mb: usize,
    current_size_bytes: usize,
    entries: HashMap<CacheKey, CacheEntry>,
    access_order: VecDeque<CacheKey>,
    hits: u64,
    misses: u64,
}

impl FrameCache {
    /// Create a new frame cache
    pub fn new(max_size_mb: usize) -> Self {
        Self {
            max_size_mb,
            current_size_bytes: 0,
            entries: HashMap::new(),
            access_order: VecDeque::new(),
            hits: 0,
            misses: 0,
        }
    }
    
    /// Insert a frame into the cache
    pub fn insert(&mut self, source: &str, frame_index: usize, frame: Frame) {
        let key = CacheKey {
            source: source.to_string(),
            frame_index,
        };
        
        let size_bytes = frame.data.len();
        
        // Remove if already exists
        if let Some(old_entry) = self.entries.remove(&key) {
            self.current_size_bytes -= old_entry.size_bytes;
            self.access_order.retain(|k| k != &key);
        }
        
        // Evict entries if needed
        while self.current_size_bytes + size_bytes > self.max_size_mb * 1024 * 1024 {
            if let Some(evict_key) = self.access_order.pop_front() {
                if let Some(entry) = self.entries.remove(&evict_key) {
                    self.current_size_bytes -= entry.size_bytes;
                }
            } else {
                break;
            }
        }
        
        // Insert new entry
        self.entries.insert(key.clone(), CacheEntry {
            frame,
            size_bytes,
            last_accessed: Instant::now(),
        });
        self.access_order.push_back(key);
        self.current_size_bytes += size_bytes;
    }
    
    /// Get a frame from the cache
    pub fn get(&mut self, source: &str, frame_index: usize) -> Option<&Frame> {
        let key = CacheKey {
            source: source.to_string(),
            frame_index,
        };
        
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.last_accessed = Instant::now();
            self.hits += 1;
            
            // Move to end of access order
            self.access_order.retain(|k| k != &key);
            self.access_order.push_back(key);
            
            Some(&entry.frame)
        } else {
            self.misses += 1;
            None
        }
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.access_order.clear();
        self.current_size_bytes = 0;
        self.hits = 0;
        self.misses = 0;
    }
    
    /// Get current cache size in MB
    pub fn current_size_mb(&self) -> f32 {
        self.current_size_bytes as f32 / (1024.0 * 1024.0)
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let hit_rate = if self.hits + self.misses > 0 {
            (self.hits as f64 / (self.hits + self.misses) as f64) * 100.0
        } else {
            0.0
        };
        
        CacheStats {
            total_frames: self.entries.len(),
            cache_size_mb: self.current_size_mb(),
            hit_rate,
            hits: self.hits,
            misses: self.misses,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_frames: usize,
    pub cache_size_mb: f32,
    pub hit_rate: f64,
    pub hits: u64,
    pub misses: u64,
}

#[cfg(test)]
mod tests;