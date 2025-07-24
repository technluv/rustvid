//! Comprehensive tests for buffer management

use super::*;
use crate::{frame::Frame, traits::PixelFormat};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use proptest::prelude::*;
use rstest::*;
use test_case::test_case;

#[cfg(test)]
mod pool_tests {
    use super::*;
    
    #[test]
    fn test_pool_creation() {
        let pool = FramePool::new(10, 1920, 1080, PixelFormat::RGB24);
        assert_eq!(pool.capacity(), 10);
        assert_eq!(pool.available(), 10);
        assert_eq!(pool.in_use(), 0);
    }
    
    #[test]
    fn test_pool_get_and_return() {
        let pool = Arc::new(Mutex::new(FramePool::new(5, 640, 480, PixelFormat::RGB24)));
        
        // Get a frame
        let frame = {
            let mut pool_guard = pool.lock().unwrap();
            pool_guard.get()
        };
        
        assert!(frame.is_some());
        {
            let pool_guard = pool.lock().unwrap();
            assert_eq!(pool_guard.available(), 4);
            assert_eq!(pool_guard.in_use(), 1);
        }
        
        // Return the frame
        {
            let mut pool_guard = pool.lock().unwrap();
            pool_guard.return_frame(frame.unwrap());
            assert_eq!(pool_guard.available(), 5);
            assert_eq!(pool_guard.in_use(), 0);
        }
    }
    
    #[test]
    fn test_pool_exhaustion() {
        let mut pool = FramePool::new(3, 320, 240, PixelFormat::YUV420P);
        
        // Get all frames
        let frames: Vec<_> = (0..3).map(|_| pool.get().unwrap()).collect();
        assert_eq!(pool.available(), 0);
        
        // Try to get one more
        assert!(pool.get().is_none());
        
        // Return one frame
        pool.return_frame(frames[0].clone());
        assert_eq!(pool.available(), 1);
        
        // Now we can get one
        assert!(pool.get().is_some());
    }
    
    #[rstest]
    #[case(PixelFormat::RGB24, 1920*1080*3)]
    #[case(PixelFormat::RGBA, 1920*1080*4)]
    #[case(PixelFormat::YUV420P, 1920*1080*3/2)]
    fn test_pool_frame_sizes(#[case] format: PixelFormat, #[case] expected_size: usize) {
        let pool = FramePool::new(1, 1920, 1080, format);
        let frame = pool.get().unwrap();
        assert_eq!(frame.data.len(), expected_size);
    }
    
    #[test]
    fn test_pool_resize() {
        let mut pool = FramePool::new(5, 640, 480, PixelFormat::RGB24);
        
        // Get some frames
        let _frames: Vec<_> = (0..3).map(|_| pool.get().unwrap()).collect();
        
        // Resize to larger capacity
        pool.resize(10);
        assert_eq!(pool.capacity(), 10);
        assert_eq!(pool.available(), 7); // 10 - 3 in use
        
        // Resize to smaller capacity (should not affect in-use frames)
        pool.resize(2);
        assert_eq!(pool.capacity(), 2);
        assert_eq!(pool.in_use(), 3); // Still 3 in use
    }
}

#[cfg(test)]
mod cache_tests {
    use super::*;
    
    #[test]
    fn test_cache_basic_operations() {
        let mut cache = FrameCache::new(100); // 100MB cache
        
        let frame1 = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
        let frame2 = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(33)).unwrap();
        
        // Insert frames
        cache.insert("video1", 0, frame1.clone());
        cache.insert("video1", 1, frame2.clone());
        
        // Retrieve frames
        assert!(cache.get("video1", 0).is_some());
        assert!(cache.get("video1", 1).is_some());
        assert!(cache.get("video1", 2).is_none());
        assert!(cache.get("video2", 0).is_none());
    }
    
    #[test]
    fn test_cache_eviction() {
        let mut cache = FrameCache::new(5); // 5MB cache - very small
        
        // Create frames that are ~0.9MB each (640x480 RGB)
        let frames: Vec<_> = (0..10)
            .map(|i| Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(i * 33)).unwrap())
            .collect();
        
        // Insert frames - should trigger eviction
        for (i, frame) in frames.iter().enumerate() {
            cache.insert("video", i, frame.clone());
        }
        
        // Cache should contain only the most recent frames
        assert!(cache.current_size_mb() <= 5);
        
        // Recent frames should be present
        assert!(cache.get("video", 9).is_some());
        assert!(cache.get("video", 8).is_some());
        
        // Old frames should have been evicted
        assert!(cache.get("video", 0).is_none());
    }
    
    #[test]
    fn test_cache_clear() {
        let mut cache = FrameCache::new(50);
        
        // Add some frames
        for i in 0..5 {
            let frame = Frame::new(320, 240, PixelFormat::RGB24, Duration::from_millis(i * 33)).unwrap();
            cache.insert("video", i as usize, frame);
        }
        
        assert!(cache.current_size_mb() > 0.0);
        
        // Clear cache
        cache.clear();
        assert_eq!(cache.current_size_mb(), 0.0);
        
        // All frames should be gone
        for i in 0..5 {
            assert!(cache.get("video", i).is_none());
        }
    }
    
    #[test]
    fn test_cache_stats() {
        let mut cache = FrameCache::new(100);
        
        let frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::ZERO).unwrap();
        cache.insert("video", 0, frame.clone());
        
        let stats = cache.stats();
        assert_eq!(stats.total_frames, 1);
        assert!(stats.cache_size_mb > 0.0);
        assert_eq!(stats.hit_rate, 0.0); // No hits yet
        
        // Access the frame
        cache.get("video", 0);
        cache.get("video", 0);
        cache.get("video", 1); // Miss
        
        let stats = cache.stats();
        assert!(stats.hit_rate > 0.0);
    }
}

#[cfg(test)]
mod concurrent_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    #[test]
    fn test_pool_thread_safety() {
        let pool = Arc::new(Mutex::new(FramePool::new(20, 640, 480, PixelFormat::RGB24)));
        let mut handles = vec![];
        
        // Spawn threads that get and return frames
        for _ in 0..10 {
            let pool_clone = Arc::clone(&pool);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let frame = {
                        let mut pool_guard = pool_clone.lock().unwrap();
                        pool_guard.get()
                    };
                    
                    if let Some(frame) = frame {
                        // Simulate some work
                        thread::sleep(Duration::from_micros(10));
                        
                        let mut pool_guard = pool_clone.lock().unwrap();
                        pool_guard.return_frame(frame);
                    }
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Pool should be back to initial state
        let pool_guard = pool.lock().unwrap();
        assert_eq!(pool_guard.available(), 20);
        assert_eq!(pool_guard.in_use(), 0);
    }
    
    #[test]
    fn test_cache_concurrent_access() {
        let cache = Arc::new(Mutex::new(FrameCache::new(200)));
        let mut handles = vec![];
        let successful_ops = Arc::new(AtomicUsize::new(0));
        
        // Writer threads
        for thread_id in 0..5 {
            let cache_clone = Arc::clone(&cache);
            let ops_clone = Arc::clone(&successful_ops);
            let handle = thread::spawn(move || {
                for i in 0..20 {
                    let frame = Frame::new(320, 240, PixelFormat::RGB24, 
                                         Duration::from_millis((thread_id * 20 + i) as u64)).unwrap();
                    let mut cache_guard = cache_clone.lock().unwrap();
                    cache_guard.insert(&format!("video{}", thread_id), i, frame);
                    ops_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }
        
        // Reader threads
        for thread_id in 0..5 {
            let cache_clone = Arc::clone(&cache);
            let handle = thread::spawn(move || {
                for i in 0..20 {
                    let cache_guard = cache_clone.lock().unwrap();
                    let _frame = cache_guard.get(&format!("video{}", thread_id), i);
                    // Don't assert on presence - frame might not be inserted yet
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(successful_ops.load(Ordering::Relaxed), 100);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_pool_allocation_performance() {
        let mut pool = FramePool::new(100, 1920, 1080, PixelFormat::RGB24);
        
        let start = Instant::now();
        let frames: Vec<_> = (0..100).map(|_| pool.get().unwrap()).collect();
        let allocation_time = start.elapsed();
        
        println!("Allocated 100 HD frames in {:?}", allocation_time);
        assert!(allocation_time < Duration::from_millis(100), "Pool allocation too slow");
        
        let start = Instant::now();
        for frame in frames {
            pool.return_frame(frame);
        }
        let return_time = start.elapsed();
        
        println!("Returned 100 HD frames in {:?}", return_time);
        assert!(return_time < Duration::from_millis(10), "Frame return too slow");
    }
    
    #[test]
    fn test_cache_lookup_performance() {
        let mut cache = FrameCache::new(500);
        
        // Fill cache with frames
        for i in 0..1000 {
            let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(i)).unwrap();
            cache.insert("video", i as usize, frame);
        }
        
        // Measure lookup performance
        let start = Instant::now();
        let mut hits = 0;
        for i in 0..10000 {
            if cache.get("video", i % 1000).is_some() {
                hits += 1;
            }
        }
        let lookup_time = start.elapsed();
        
        println!("Performed 10,000 cache lookups in {:?}", lookup_time);
        println!("Hit rate: {:.2}%", (hits as f64 / 10000.0) * 100.0);
        
        assert!(lookup_time < Duration::from_millis(100), "Cache lookup too slow");
    }
}

// Property-based tests
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_pool_consistency(
            capacity in 1usize..100,
            width in 16u32..1920,
            height in 16u32..1080,
            gets in 0usize..200,
        ) {
            let mut pool = FramePool::new(capacity, width, height, PixelFormat::RGB24);
            let mut frames = Vec::new();
            
            // Get as many frames as requested (up to capacity)
            for _ in 0..gets.min(capacity) {
                if let Some(frame) = pool.get() {
                    frames.push(frame);
                }
            }
            
            assert_eq!(pool.available() + pool.in_use(), capacity);
            assert_eq!(pool.in_use(), frames.len());
            
            // Return all frames
            for frame in frames {
                pool.return_frame(frame);
            }
            
            assert_eq!(pool.available(), capacity);
            assert_eq!(pool.in_use(), 0);
        }
        
        #[test]
        fn test_cache_size_limits(
            max_size_mb in 1usize..1000,
            frame_count in 0usize..100,
        ) {
            let mut cache = FrameCache::new(max_size_mb);
            
            // Insert frames
            for i in 0..frame_count {
                let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(i as u64)).unwrap();
                cache.insert("video", i, frame);
            }
            
            // Cache size should never exceed limit
            assert!(cache.current_size_mb() <= max_size_mb as f32 * 1.1); // Allow 10% overhead
        }
    }
}