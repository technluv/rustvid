//! Performance benchmarks for buffer operations

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use video_editor_core::{
    buffer::{FramePool, FrameCache},
    frame::Frame,
    traits::PixelFormat,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn bench_frame_pool_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_pool_basic");
    
    let pool_sizes = vec![10, 25, 50, 100];
    
    for pool_size in pool_sizes {
        group.bench_with_input(
            BenchmarkId::new("get_return", pool_size),
            &pool_size,
            |b, &size| {
                let mut pool = FramePool::new(size, 1920, 1080, PixelFormat::RGB24);
                b.iter(|| {
                    if let Some(frame) = pool.get() {
                        pool.return_frame(black_box(frame));
                    }
                })
            },
        );
    }
    
    group.finish();
}

fn bench_frame_pool_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_pool_allocation");
    
    let frame_counts = vec![10, 50, 100];
    
    for count in frame_counts {
        group.bench_with_input(
            BenchmarkId::new("bulk_get", count),
            &count,
            |b, &frame_count| {
                b.iter(|| {
                    let mut pool = FramePool::new(frame_count, 1920, 1080, PixelFormat::RGB24);
                    let mut frames = Vec::new();
                    
                    // Get all frames
                    for _ in 0..frame_count {
                        if let Some(frame) = pool.get() {
                            frames.push(frame);
                        }
                    }
                    
                    // Return all frames
                    for frame in frames {
                        pool.return_frame(frame);
                    }
                    
                    black_box(pool);
                })
            },
        );
    }
    
    group.finish();
}

fn bench_frame_pool_resize(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_pool_resize");
    
    group.bench_function("resize_up", |b| {
        b.iter(|| {
            let mut pool = FramePool::new(25, 1280, 720, PixelFormat::RGB24);
            pool.resize(black_box(50));
            black_box(pool);
        })
    });
    
    group.bench_function("resize_down", |b| {
        b.iter(|| {
            let mut pool = FramePool::new(100, 1280, 720, PixelFormat::RGB24);
            
            // Get some frames to simulate usage
            let frames: Vec<_> = (0..30).filter_map(|_| pool.get()).collect();
            
            pool.resize(black_box(50));
            
            // Return frames
            for frame in frames {
                pool.return_frame(frame);
            }
            
            black_box(pool);
        })
    });
    
    group.finish();
}

fn bench_frame_cache_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_cache_basic");
    
    let cache_sizes = vec![50, 100, 200]; // MB
    
    for cache_size in cache_sizes {
        group.bench_with_input(
            BenchmarkId::new("insert_get", cache_size),
            &cache_size,
            |b, &size| {
                b.iter(|| {
                    let mut cache = FrameCache::new(size);
                    let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::ZERO).unwrap();
                    
                    cache.insert("test", 0, black_box(frame));
                    let _ = black_box(cache.get("test", 0));
                })
            },
        );
    }
    
    group.finish();
}

fn bench_frame_cache_fill(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_cache_fill");
    
    group.bench_function("fill_cache_sequential", |b| {
        b.iter(|| {
            let mut cache = FrameCache::new(100); // 100MB
            
            // Fill cache with frames
            for i in 0..200 {
                let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(i)).unwrap();
                cache.insert("video", i as usize, black_box(frame));
            }
            
            black_box(cache);
        })
    });
    
    group.bench_function("cache_lookup_random", |b| {
        let mut cache = FrameCache::new(100);
        
        // Pre-fill cache
        for i in 0..100 {
            let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(i)).unwrap();
            cache.insert("video", i as usize, frame);
        }
        
        b.iter(|| {
            let frame_idx = black_box(rand::random::<usize>() % 100);
            let _ = black_box(cache.get("video", frame_idx));
        })
    });
    
    group.finish();
}

fn bench_frame_cache_eviction(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_cache_eviction");
    
    group.bench_function("trigger_eviction", |b| {
        b.iter(|| {
            let mut cache = FrameCache::new(10); // Small cache to trigger eviction
            
            // Fill beyond capacity
            for i in 0..50 {
                let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(i)).unwrap();
                cache.insert("video", i as usize, black_box(frame));
            }
            
            black_box(cache);
        })
    });
    
    group.finish();
}

fn bench_concurrent_pool_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_pool_access");
    
    group.bench_function("multi_thread_pool", |b| {
        b.iter(|| {
            let pool = Arc::new(Mutex::new(FramePool::new(100, 1920, 1080, PixelFormat::RGB24)));
            let mut handles = vec![];
            
            // Spawn threads that use the pool
            for _ in 0..4 {
                let pool_clone = Arc::clone(&pool);
                let handle = thread::spawn(move || {
                    for _ in 0..25 {
                        let frame = {
                            let mut pool_guard = pool_clone.lock().unwrap();
                            pool_guard.get()
                        };
                        
                        if let Some(frame) = frame {
                            // Simulate some work
                            thread::sleep(Duration::from_nanos(100));
                            
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
            
            black_box(pool);
        })
    });
    
    group.finish();
}

fn bench_memory_usage_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage_patterns");
    
    group.bench_function("streaming_pattern", |b| {
        b.iter(|| {
            let mut pool = FramePool::new(30, 1920, 1080, PixelFormat::RGB24); // ~1 second buffer
            let mut frames = Vec::new();
            
            // Simulate streaming: get frame, process, return
            for _ in 0..60 { // 2 seconds of frames
                if let Some(frame) = pool.get() {
                    frames.push(frame);
                }
                
                // Return oldest frame after 1 second buffer
                if frames.len() > 30 {
                    pool.return_frame(frames.remove(0));
                }
            }
            
            // Return remaining frames
            for frame in frames {
                pool.return_frame(frame);
            }
            
            black_box(pool);
        })
    });
    
    group.bench_function("batch_processing", |b| {
        b.iter(|| {
            let mut pool = FramePool::new(50, 1920, 1080, PixelFormat::RGB24);
            
            // Get batch of frames
            let frames: Vec<_> = (0..40).filter_map(|_| pool.get()).collect();
            
            // Simulate batch processing
            for _ in &frames {
                // Simulate processing work
                black_box(42);
            }
            
            // Return all frames
            for frame in frames {
                pool.return_frame(frame);
            }
            
            black_box(pool);
        })
    });
    
    group.finish();
}

fn bench_cache_statistics(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_statistics");
    
    group.bench_function("stats_calculation", |b| {
        let mut cache = FrameCache::new(50);
        
        // Fill cache with some data
        for i in 0..30 {
            let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(i)).unwrap();
            cache.insert("video", i as usize, frame);
        }
        
        // Generate some hits and misses
        for i in 0..100 {
            let _ = cache.get("video", i % 50);
        }
        
        b.iter(|| {
            let stats = black_box(cache.stats());
            black_box(stats);
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_frame_pool_basic,
    bench_frame_pool_allocation,
    bench_frame_pool_resize,
    bench_frame_cache_basic,
    bench_frame_cache_fill,
    bench_frame_cache_eviction,
    bench_concurrent_pool_access,
    bench_memory_usage_patterns,
    bench_cache_statistics
);

criterion_main!(benches);