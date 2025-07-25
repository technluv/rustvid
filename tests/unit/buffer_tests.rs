//! Unit tests for buffer management

use video_editor_core::buffer::{FramePool, FrameCache};
use video_editor_core::{Frame, PixelFormat};
use std::time::Duration;
use std::sync::Arc;
use std::thread;

#[test]
fn test_frame_pool_creation() {
    let pool = FramePool::new(10, 1920, 1080, PixelFormat::RGB24);
    
    assert_eq!(pool.capacity(), 10);
    assert_eq!(pool.available(), 10);
    assert_eq!(pool.in_use(), 0);
}

#[test]
fn test_frame_pool_get_return() {
    let mut pool = FramePool::new(5, 640, 480, PixelFormat::RGB24);
    
    // Get frames
    let frame1 = pool.get().unwrap();
    let frame2 = pool.get().unwrap();
    
    assert_eq!(pool.available(), 3);
    assert_eq!(pool.in_use(), 2);
    
    // Return frames
    pool.return_frame(frame1);
    assert_eq!(pool.available(), 4);
    assert_eq!(pool.in_use(), 1);
    
    pool.return_frame(frame2);
    assert_eq!(pool.available(), 5);
    assert_eq!(pool.in_use(), 0);
}

#[test]
fn test_frame_pool_exhaustion() {
    let mut pool = FramePool::new(3, 320, 240, PixelFormat::RGB24);
    
    // Exhaust pool
    let _f1 = pool.get().unwrap();
    let _f2 = pool.get().unwrap();
    let _f3 = pool.get().unwrap();
    
    // Pool should be empty
    assert!(pool.get().is_none());
    assert_eq!(pool.available(), 0);
    assert_eq!(pool.in_use(), 3);
}

#[test]
fn test_frame_pool_concurrent_access() {
    let pool = Arc::new(std::sync::Mutex::new(
        FramePool::new(50, 640, 480, PixelFormat::RGB24)
    ));
    
    let mut handles = vec![];
    
    // Spawn threads that get and return frames
    for i in 0..10 {
        let pool_clone = Arc::clone(&pool);
        
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                let frame = {
                    let mut p = pool_clone.lock().unwrap();
                    p.get()
                };
                
                if let Some(frame) = frame {
                    thread::sleep(Duration::from_millis(i));
                    
                    let mut p = pool_clone.lock().unwrap();
                    p.return_frame(frame);
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // All frames should be returned
    let pool = pool.lock().unwrap();
    assert_eq!(pool.available(), 50);
    assert_eq!(pool.in_use(), 0);
}

#[test]
fn test_frame_cache_basic() {
    let mut cache = FrameCache::new(100); // 100MB cache
    
    // Create test frames
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
fn test_frame_cache_eviction() {
    let mut cache = FrameCache::new(10); // Small 10MB cache
    
    // Each frame is ~1.8MB (640x480x3)
    let frame_size = 640 * 480 * 3;
    let max_frames = (10 * 1024 * 1024) / frame_size;
    
    // Fill cache beyond capacity
    for i in 0..max_frames + 5 {
        let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(i as u64)).unwrap();
        cache.insert("video", i, frame);
    }
    
    // Early frames should be evicted
    assert!(cache.get("video", 0).is_none());
    assert!(cache.get("video", 1).is_none());
    
    // Recent frames should still be present
    assert!(cache.get("video", max_frames + 4).is_some());
}

#[test]
fn test_frame_cache_hit_rate() {
    let mut cache = FrameCache::new(50);
    
    // Insert some frames
    for i in 0..10 {
        let frame = Frame::new(320, 240, PixelFormat::RGB24, Duration::from_millis(i)).unwrap();
        cache.insert("test", i as usize, frame);
    }
    
    // Access pattern to test hit rate
    let mut hits = 0;
    let total_accesses = 20;
    
    for i in 0..total_accesses {
        if cache.get("test", i % 15).is_some() {
            hits += 1;
        }
    }
    
    let hit_rate = hits as f64 / total_accesses as f64;
    assert!(hit_rate > 0.5); // Should have > 50% hit rate
}

#[test]
fn test_frame_cache_clear() {
    let mut cache = FrameCache::new(100);
    
    // Add frames
    for i in 0..5 {
        let frame = Frame::new(320, 240, PixelFormat::RGB24, Duration::from_millis(i)).unwrap();
        cache.insert("video", i as usize, frame);
    }
    
    // Verify frames exist
    assert!(cache.get("video", 0).is_some());
    
    // Clear cache
    cache.clear();
    
    // Verify all frames are gone
    for i in 0..5 {
        assert!(cache.get("video", i).is_none());
    }
}

#[test]
fn test_frame_cache_multiple_videos() {
    let mut cache = FrameCache::new(100);
    
    // Insert frames from different videos
    for video_id in 0..3 {
        for frame_id in 0..5 {
            let frame = Frame::new(320, 240, PixelFormat::RGB24, 
                                 Duration::from_millis(frame_id)).unwrap();
            cache.insert(&format!("video{}", video_id), frame_id as usize, frame);
        }
    }
    
    // Verify all frames are accessible
    for video_id in 0..3 {
        for frame_id in 0..5 {
            assert!(cache.get(&format!("video{}", video_id), frame_id).is_some());
        }
    }
}

#[test]
fn test_frame_pool_statistics() {
    let mut pool = FramePool::new(10, 1920, 1080, PixelFormat::RGB24);
    
    // Get some frames
    let frames: Vec<_> = (0..5).map(|_| pool.get().unwrap()).collect();
    
    let stats = pool.statistics();
    assert_eq!(stats.total_frames, 10);
    assert_eq!(stats.frames_in_use, 5);
    assert_eq!(stats.frames_available, 5);
    assert!(stats.peak_usage >= 5);
    
    // Return frames
    for frame in frames {
        pool.return_frame(frame);
    }
    
    let stats = pool.statistics();
    assert_eq!(stats.frames_in_use, 0);
    assert_eq!(stats.frames_available, 10);
}

#[test]
fn test_frame_pool_resize() {
    let mut pool = FramePool::new(5, 640, 480, PixelFormat::RGB24);
    
    // Resize pool
    pool.resize(10);
    assert_eq!(pool.capacity(), 10);
    assert_eq!(pool.available(), 10);
    
    // Resize down (should work if no frames in use)
    pool.resize(3);
    assert_eq!(pool.capacity(), 3);
    assert_eq!(pool.available(), 3);
}

#[test]
fn test_cache_memory_tracking() {
    let mut cache = FrameCache::new(50);
    
    assert_eq!(cache.current_size(), 0);
    
    // Add frames and track memory
    let frame_size = 640 * 480 * 3;
    for i in 0..5 {
        let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(i)).unwrap();
        cache.insert("video", i as usize, frame);
    }
    
    assert!(cache.current_size() >= frame_size * 5);
    assert!(cache.current_size() <= cache.max_size());
}