use criterion::{black_box, criterion_group, criterion_main, Criterion};
use effects::{
    filters::FilterFactory,
    gpu::GpuContext,
    pipeline::EffectPipeline,
    traits::*,
    DefaultEffectFactory,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn create_test_frame(width: u32, height: u32) -> Frame {
    Frame {
        width,
        height,
        data: vec![128; (width * height * 4) as usize],
        format: PixelFormat::Rgba8,
        timestamp: 0.0,
    }
}

fn benchmark_blur_effect(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let frame = create_test_frame(1920, 1080);
    
    c.bench_function("blur_effect_cpu", |b| {
        b.to_async(&rt).iter(|| async {
            let mut blur = FilterFactory::create_blur(5.0).unwrap();
            let result = blur.apply(black_box(&frame), 0.0).unwrap();
            black_box(result);
        });
    });
}

fn benchmark_brightness_contrast(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let frame = create_test_frame(1920, 1080);
    
    c.bench_function("brightness_contrast_cpu", |b| {
        b.to_async(&rt).iter(|| async {
            let mut effect = FilterFactory::create_brightness_contrast(0.2, 1.5).unwrap();
            let result = effect.apply(black_box(&frame), 0.0).unwrap();
            black_box(result);
        });
    });
}

fn benchmark_color_correction(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let frame = create_test_frame(1920, 1080);
    
    c.bench_function("color_correction_cpu", |b| {
        b.to_async(&rt).iter(|| async {
            let mut effect = FilterFactory::create_color_correction(0.1, 1.2, 0.0, 0.2, -0.1).unwrap();
            let result = effect.apply(black_box(&frame), 0.0).unwrap();
            black_box(result);
        });
    });
}

fn benchmark_pipeline_processing(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let frame = create_test_frame(1920, 1080);
    
    c.bench_function("pipeline_multiple_effects", |b| {
        b.to_async(&rt).iter(|| async {
            let gpu_context = Arc::new(GpuContext::new().await.unwrap());
            let mut pipeline = EffectPipeline::with_gpu_context(gpu_context).await.unwrap();
            let factory = DefaultEffectFactory::new();
            
            // Add multiple effects
            let blur_effect = factory.create("blur").unwrap();
            pipeline.add_effect(blur_effect);
            
            let bc_effect = factory.create("brightness_contrast").unwrap();
            pipeline.add_effect(bc_effect);
            
            let result = pipeline.process_frame(black_box(&frame), 0.0).await.unwrap();
            black_box(result);
        });
    });
}

fn benchmark_batch_processing(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let frames = vec![
        create_test_frame(1920, 1080),
        create_test_frame(1920, 1080),
        create_test_frame(1920, 1080),
        create_test_frame(1920, 1080),
    ];
    
    c.bench_function("batch_processing_4_frames", |b| {
        b.to_async(&rt).iter(|| async {
            let gpu_context = Arc::new(GpuContext::new().await.unwrap());
            let mut pipeline = EffectPipeline::with_gpu_context(gpu_context).await.unwrap();
            let factory = DefaultEffectFactory::new();
            
            let blur_effect = factory.create("blur").unwrap();
            pipeline.add_effect(blur_effect);
            
            let results = pipeline
                .process_frames_batch(black_box(&frames), 0.0, 1.0 / 30.0)
                .await
                .unwrap();
            black_box(results);
        });
    });
}

fn benchmark_transitions(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let frame1 = create_test_frame(1920, 1080);
    let frame2 = create_test_frame(1920, 1080);
    
    c.bench_function("fade_transition", |b| {
        b.to_async(&rt).iter(|| async {
            use effects::transitions::fade::FadeTransition;
            let mut transition = FadeTransition::new();
            let result = transition
                .apply_transition(black_box(&frame1), black_box(&frame2), 0.5, 0.0)
                .unwrap();
            black_box(result);
        });
    });
    
    c.bench_function("dissolve_transition", |b| {
        b.to_async(&rt).iter(|| async {
            use effects::transitions::dissolve::DissolveTransition;
            let mut transition = DissolveTransition::new();
            let result = transition
                .apply_transition(black_box(&frame1), black_box(&frame2), 0.5, 0.0)
                .unwrap();
            black_box(result);
        });
    });
}

fn benchmark_different_resolutions(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let resolutions = [
        (640, 480),   // SD
        (1280, 720),  // HD
        (1920, 1080), // Full HD
        (3840, 2160), // 4K
    ];
    
    for (width, height) in resolutions.iter() {
        let frame = create_test_frame(*width, *height);
        let bench_name = format!("blur_{}x{}", width, height);
        
        c.bench_function(&bench_name, |b| {
            b.to_async(&rt).iter(|| async {
                let mut blur = FilterFactory::create_blur(5.0).unwrap();
                let result = blur.apply(black_box(&frame), 0.0).unwrap();
                black_box(result);
            });
        });
    }
}

criterion_group!(
    benches,
    benchmark_blur_effect,
    benchmark_brightness_contrast,
    benchmark_color_correction,
    benchmark_pipeline_processing,
    benchmark_batch_processing,
    benchmark_transitions,
    benchmark_different_resolutions,
);
criterion_main!(benches);