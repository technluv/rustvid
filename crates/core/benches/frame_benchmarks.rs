//! Performance benchmarks for frame operations

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use video_editor_core::{
    frame::{Frame, FrameBuilder},
    traits::PixelFormat,
};
use std::time::Duration;

fn bench_frame_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_creation");
    
    let formats = vec![
        ("RGB24", PixelFormat::RGB24),
        ("RGBA", PixelFormat::RGBA),
        ("YUV420P", PixelFormat::YUV420P),
    ];
    
    let sizes = vec![
        ("720p", 1280, 720),
        ("1080p", 1920, 1080),
        ("4K", 3840, 2160),
    ];
    
    for (format_name, format) in formats {
        for (size_name, width, height) in &sizes {
            group.bench_with_input(
                BenchmarkId::new(format_name, size_name),
                &(width, height, format),
                |b, &(w, h, fmt)| {
                    b.iter(|| {
                        black_box(Frame::new(*w, *h, fmt, Duration::from_secs(0)).unwrap())
                    })
                },
            );
        }
    }
    
    group.finish();
}

fn bench_frame_cloning(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_cloning");
    
    let frames = vec![
        ("720p_RGB", Frame::new(1280, 720, PixelFormat::RGB24, Duration::ZERO).unwrap()),
        ("1080p_RGB", Frame::new(1920, 1080, PixelFormat::RGB24, Duration::ZERO).unwrap()),
        ("4K_RGB", Frame::new(3840, 2160, PixelFormat::RGB24, Duration::ZERO).unwrap()),
    ];
    
    for (name, frame) in frames {
        group.bench_function(name, |b| {
            b.iter(|| black_box(frame.clone()))
        });
    }
    
    group.finish();
}

fn bench_frame_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_validation");
    
    let frames = vec![
        ("Valid_720p", Frame::new(1280, 720, PixelFormat::RGB24, Duration::ZERO).unwrap()),
        ("Valid_1080p", Frame::new(1920, 1080, PixelFormat::RGB24, Duration::ZERO).unwrap()),
        ("Valid_4K", Frame::new(3840, 2160, PixelFormat::RGB24, Duration::ZERO).unwrap()),
    ];
    
    for (name, frame) in frames {
        group.bench_function(name, |b| {
            b.iter(|| black_box(frame.validate()))
        });
    }
    
    group.finish();
}

fn bench_frame_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_builder");
    
    group.bench_function("simple_build", |b| {
        b.iter(|| {
            black_box(
                FrameBuilder::new()
                    .width(1920)
                    .height(1080)
                    .format(PixelFormat::RGB24)
                    .timestamp(Duration::from_millis(33))
                    .build()
                    .unwrap()
            )
        })
    });
    
    group.bench_function("complex_build", |b| {
        b.iter(|| {
            black_box(
                FrameBuilder::new()
                    .width(3840)
                    .height(2160)
                    .format(PixelFormat::RGBA)
                    .timestamp(Duration::from_millis(16))
                    .pts(Some(1000))
                    .dts(Some(900))
                    .key_frame(true)
                    .build()
                    .unwrap()
            )
        })
    });
    
    group.finish();
}

fn bench_pixel_format_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("pixel_format");
    
    let formats = vec![
        PixelFormat::RGB24,
        PixelFormat::RGBA,
        PixelFormat::YUV420P,
        PixelFormat::YUV422P,
        PixelFormat::YUV444P,
        PixelFormat::Gray8,
    ];
    
    group.bench_function("bytes_per_pixel", |b| {
        b.iter(|| {
            for format in &formats {
                black_box(format.bytes_per_pixel());
            }
        })
    });
    
    group.finish();
}

// Benchmark frame data manipulation
fn bench_frame_data_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_data_operations");
    
    let mut frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::ZERO).unwrap();
    
    group.bench_function("fill_pattern", |b| {
        b.iter(|| {
            for (i, pixel) in frame.data.iter_mut().enumerate() {
                *pixel = black_box((i % 256) as u8);
            }
        })
    });
    
    let frame_readonly = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::ZERO).unwrap();
    
    group.bench_function("read_all_pixels", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for &pixel in &frame_readonly.data {
                sum += black_box(pixel as u64);
            }
            black_box(sum)
        })
    });
    
    group.bench_function("calculate_average", |b| {
        b.iter(|| {
            let sum: u64 = frame_readonly.data.iter().map(|&p| p as u64).sum();
            black_box(sum as f64 / frame_readonly.data.len() as f64)
        })
    });
    
    group.finish();
}

// Benchmark memory operations
fn bench_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");
    
    group.bench_function("allocate_hd_frame", |b| {
        b.iter(|| {
            let data = black_box(vec![0u8; 1920 * 1080 * 3]);
            drop(data);
        })
    });
    
    group.bench_function("allocate_4k_frame", |b| {
        b.iter(|| {
            let data = black_box(vec![0u8; 3840 * 2160 * 3]);
            drop(data);
        })
    });
    
    // Test different initialization patterns
    group.bench_function("zero_init", |b| {
        b.iter(|| {
            let data = black_box(vec![0u8; 1920 * 1080 * 3]);
            drop(data);
        })
    });
    
    group.bench_function("pattern_init", |b| {
        b.iter(|| {
            let mut data = Vec::with_capacity(1920 * 1080 * 3);
            for i in 0..(1920 * 1080 * 3) {
                data.push((i % 256) as u8);
            }
            black_box(data);
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_frame_creation,
    bench_frame_cloning,
    bench_frame_validation,
    bench_frame_builder,
    bench_pixel_format_operations,
    bench_frame_data_operations,
    bench_memory_operations
);

criterion_main!(benches);