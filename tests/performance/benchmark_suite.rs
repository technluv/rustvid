//! Performance benchmark suite

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use video_editor_core::{Frame, PixelFormat, buffer::{FramePool, FrameCache}};
use video_editor_effects::{BrightnessEffect, BlurEffect, Effect, ParameterValue};
use std::time::Duration;

fn benchmark_frame_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_creation");
    
    for (width, height) in &[(640, 480), (1280, 720), (1920, 1080), (3840, 2160)] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}x{}", width, height)),
            &(*width, *height),
            |b, &(w, h)| {
                b.iter(|| {
                    Frame::new(w, h, PixelFormat::RGB24, Duration::from_millis(0))
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_frame_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_conversion");
    
    let frame_1080p = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    
    group.bench_function("RGB24_to_YUV420P_1080p", |b| {
        b.iter(|| {
            black_box(frame_1080p.convert_to(PixelFormat::YUV420P))
        });
    });
    
    group.bench_function("RGB24_to_RGBA32_1080p", |b| {
        b.iter(|| {
            black_box(frame_1080p.convert_to(PixelFormat::RGBA32))
        });
    });
    
    let frame_4k = Frame::new(3840, 2160, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    
    group.bench_function("RGB24_to_YUV420P_4K", |b| {
        b.iter(|| {
            black_box(frame_4k.convert_to(PixelFormat::YUV420P))
        });
    });
    
    group.finish();
}

fn benchmark_buffer_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_pool");
    
    group.bench_function("pool_get_return_1080p", |b| {
        let mut pool = FramePool::new(100, 1920, 1080, PixelFormat::RGB24);
        
        b.iter(|| {
            let frame = pool.get().unwrap();
            black_box(&frame);
            pool.return_frame(frame);
        });
    });
    
    group.bench_function("pool_concurrent_access", |b| {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let pool = Arc::new(Mutex::new(FramePool::new(200, 1920, 1080, PixelFormat::RGB24)));
        
        b.iter(|| {
            let mut handles = vec![];
            
            for _ in 0..4 {
                let pool_clone = Arc::clone(&pool);
                let handle = thread::spawn(move || {
                    for _ in 0..10 {
                        let frame = {
                            let mut p = pool_clone.lock().unwrap();
                            p.get()
                        };
                        
                        if let Some(frame) = frame {
                            black_box(&frame);
                            let mut p = pool_clone.lock().unwrap();
                            p.return_frame(frame);
                        }
                    }
                });
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
    
    group.finish();
}

fn benchmark_cache_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_operations");
    
    group.bench_function("cache_insert_1080p", |b| {
        let mut cache = FrameCache::new(1000); // 1GB cache
        let frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
        let mut counter = 0;
        
        b.iter(|| {
            cache.insert("video", counter, frame.clone());
            counter += 1;
        });
    });
    
    group.bench_function("cache_lookup_hit", |b| {
        let mut cache = FrameCache::new(1000);
        
        // Pre-fill cache
        for i in 0..100 {
            let frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(i)).unwrap();
            cache.insert("video", i as usize, frame);
        }
        
        b.iter(|| {
            black_box(cache.get("video", 50));
        });
    });
    
    group.bench_function("cache_lookup_miss", |b| {
        let cache = FrameCache::new(1000);
        
        b.iter(|| {
            black_box(cache.get("nonexistent", 0));
        });
    });
    
    group.finish();
}

fn benchmark_effects(c: &mut Criterion) {
    let mut group = c.benchmark_group("effects");
    
    // Create test frames
    let frame_720p = Frame::new(1280, 720, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    let frame_1080p = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    let frame_4k = Frame::new(3840, 2160, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    
    // Brightness effect benchmarks
    group.bench_function("brightness_720p", |b| {
        let mut effect = BrightnessEffect::new();
        effect.set_parameter("brightness", ParameterValue::Float(1.5)).unwrap();
        let mut frame = frame_720p.clone();
        
        b.iter(|| {
            effect.process(&mut frame as &mut dyn std::any::Any).unwrap();
        });
    });
    
    group.bench_function("brightness_1080p", |b| {
        let mut effect = BrightnessEffect::new();
        effect.set_parameter("brightness", ParameterValue::Float(1.5)).unwrap();
        let mut frame = frame_1080p.clone();
        
        b.iter(|| {
            effect.process(&mut frame as &mut dyn std::any::Any).unwrap();
        });
    });
    
    group.bench_function("brightness_4k", |b| {
        let mut effect = BrightnessEffect::new();
        effect.set_parameter("brightness", ParameterValue::Float(1.5)).unwrap();
        let mut frame = frame_4k.clone();
        
        b.iter(|| {
            effect.process(&mut frame as &mut dyn std::any::Any).unwrap();
        });
    });
    
    // Blur effect benchmarks (more computationally intensive)
    group.bench_function("blur_720p", |b| {
        let mut effect = BlurEffect::new();
        effect.set_parameter("radius", ParameterValue::Float(5.0)).unwrap();
        let mut frame = frame_720p.clone();
        
        b.iter(|| {
            effect.process(&mut frame as &mut dyn std::any::Any).unwrap();
        });
    });
    
    group.bench_function("blur_1080p", |b| {
        let mut effect = BlurEffect::new();
        effect.set_parameter("radius", ParameterValue::Float(5.0)).unwrap();
        let mut frame = frame_1080p.clone();
        
        b.iter(|| {
            effect.process(&mut frame as &mut dyn std::any::Any).unwrap();
        });
    });
    
    group.finish();
}

fn benchmark_pipeline(c: &mut Criterion) {
    use video_editor_effects::{EffectPipeline, ContrastEffect};
    
    let mut group = c.benchmark_group("pipeline");
    
    group.bench_function("multi_effect_pipeline_1080p", |b| {
        let mut pipeline = EffectPipeline::new();
        
        // Add multiple effects
        let mut brightness = BrightnessEffect::new();
        brightness.set_parameter("brightness", ParameterValue::Float(1.2)).unwrap();
        pipeline.add_effect(Box::new(brightness));
        
        let mut contrast = ContrastEffect::new();
        contrast.set_parameter("contrast", ParameterValue::Float(1.3)).unwrap();
        pipeline.add_effect(Box::new(contrast));
        
        let mut blur = BlurEffect::new();
        blur.set_parameter("radius", ParameterValue::Float(2.0)).unwrap();
        pipeline.add_effect(Box::new(blur));
        
        let mut frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
        
        b.iter(|| {
            pipeline.process(&mut frame).unwrap();
        });
    });
    
    group.finish();
}

fn benchmark_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");
    
    group.bench_function("frame_clone_1080p", |b| {
        let frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
        
        b.iter(|| {
            black_box(frame.clone());
        });
    });
    
    group.bench_function("frame_clone_4k", |b| {
        let frame = Frame::new(3840, 2160, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
        
        b.iter(|| {
            black_box(frame.clone());
        });
    });
    
    group.bench_function("bulk_allocation", |b| {
        b.iter(|| {
            let frames: Vec<_> = (0..10)
                .map(|i| Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(i)).unwrap())
                .collect();
            black_box(frames);
        });
    });
    
    group.finish();
}

fn benchmark_timeline_operations(c: &mut Criterion) {
    use video_editor_timeline::{Timeline, Clip};
    use uuid::Uuid;
    
    let mut group = c.benchmark_group("timeline");
    
    group.bench_function("timeline_add_clips", |b| {
        b.iter(|| {
            let mut timeline = Timeline::new("Benchmark Timeline".to_string());
            let track = timeline.add_track("Video Track".to_string());
            let track_id = track.id;
            
            // Add 100 clips
            if let Some(track) = timeline.tracks.iter_mut().find(|t| t.id == track_id) {
                for i in 0..100 {
                    track.clips.push(Clip {
                        id: Uuid::new_v4(),
                        start_time: Duration::from_secs(i * 10),
                        duration: Duration::from_secs(10),
                        source_path: format!("video_{}.mp4", i),
                        in_point: Duration::from_secs(0),
                        out_point: Duration::from_secs(10),
                    });
                }
            }
            
            black_box(timeline);
        });
    });
    
    group.bench_function("timeline_seek", |b| {
        let mut timeline = Timeline::new("Benchmark Timeline".to_string());
        let track = timeline.add_track("Video Track".to_string());
        let track_id = track.id;
        
        // Add clips
        if let Some(track) = timeline.tracks.iter_mut().find(|t| t.id == track_id) {
            for i in 0..1000 {
                track.clips.push(Clip {
                    id: Uuid::new_v4(),
                    start_time: Duration::from_secs(i * 10),
                    duration: Duration::from_secs(10),
                    source_path: format!("video_{}.mp4", i),
                    in_point: Duration::from_secs(0),
                    out_point: Duration::from_secs(10),
                });
            }
        }
        
        b.iter(|| {
            // Seek to middle of timeline
            let position = Duration::from_secs(5000);
            timeline.seek(position);
            black_box(timeline.current_position());
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_frame_creation,
    benchmark_frame_conversion,
    benchmark_buffer_pool,
    benchmark_cache_operations,
    benchmark_effects,
    benchmark_pipeline,
    benchmark_memory_operations,
    benchmark_timeline_operations
);

criterion_main!(benches);