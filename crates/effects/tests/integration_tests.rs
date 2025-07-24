use effects::{
    filters::FilterFactory,
    gpu::GpuContext,
    parameters::{Keyframe, InterpolationType, ParameterValue},
    pipeline::{EffectPipeline, BlendMode},
    traits::*,
    transitions::{DefaultTransitionFactory, fade::FadeTransition},
    DefaultEffectFactory,
};
use std::sync::Arc;

fn create_test_frame(width: u32, height: u32, color: u8) -> Frame {
    Frame {
        width,
        height,
        data: vec![color; (width * height * 4) as usize],
        format: PixelFormat::Rgba8,
        timestamp: 0.0,
    }
}

#[tokio::test]
async fn test_effect_factory() {
    let factory = DefaultEffectFactory::new();
    
    // Test creating effects
    let blur_effect = factory.create("blur").expect("Failed to create blur effect");
    assert_eq!(blur_effect.name(), "Box Blur");
    
    let bc_effect = factory.create("brightness_contrast").expect("Failed to create brightness/contrast effect");
    assert_eq!(bc_effect.name(), "Brightness/Contrast");
    
    // Test listing effects
    let effects = factory.available_effects();
    assert!(effects.contains(&"blur".to_string()));
    assert!(effects.contains(&"brightness_contrast".to_string()));
    
    // Test metadata
    let metadata = factory.effect_metadata("blur");
    assert!(metadata.is_some());
    let metadata = metadata.unwrap();
    assert_eq!(metadata.name, "Blur");
    assert!(metadata.requires_gpu);
}

#[tokio::test]
async fn test_filter_factory() {
    // Test blur filter
    let blur = FilterFactory::create_blur(10.0).expect("Failed to create blur filter");
    assert_eq!(blur.name(), "Box Blur");
    assert_eq!(blur.filter_type(), FilterType::Blur);
    
    // Test brightness/contrast filter
    let bc = FilterFactory::create_brightness_contrast(0.2, 1.5)
        .expect("Failed to create brightness/contrast filter");
    assert_eq!(bc.name(), "Brightness/Contrast");
    assert_eq!(bc.filter_type(), FilterType::ColorCorrection);
    
    // Test color correction filter
    let cc = FilterFactory::create_color_correction(0.1, 1.2, 0.0, 0.2, -0.1)
        .expect("Failed to create color correction filter");
    assert_eq!(cc.name(), "Color Correction");
    assert_eq!(cc.filter_type(), FilterType::ColorCorrection);
}

#[tokio::test]
async fn test_effect_pipeline() {
    let gpu_context = Arc::new(GpuContext::new().await.expect("Failed to create GPU context"));
    let mut pipeline = EffectPipeline::with_gpu_context(gpu_context.clone())
        .await
        .expect("Failed to create pipeline");
    
    let factory = DefaultEffectFactory::new();
    
    // Add effects to pipeline
    let blur_effect = factory.create("blur").unwrap();
    let blur_id = pipeline.add_effect(blur_effect);
    
    let bc_effect = factory.create("brightness_contrast").unwrap();
    let bc_id = pipeline.add_effect(bc_effect);
    
    assert_eq!(pipeline.get_effects().len(), 2);
    
    // Test reordering
    pipeline.reorder_effect(bc_id, 0).expect("Failed to reorder effect");
    assert_eq!(pipeline.get_effects()[0].id, bc_id);
    assert_eq!(pipeline.get_effects()[1].id, blur_id);
    
    // Test enabling/disabling
    pipeline.set_effect_enabled(blur_id, false).expect("Failed to disable effect");
    assert!(!pipeline.get_effects()[1].enabled);
    
    // Test blend mode and opacity
    pipeline.set_effect_blend_mode(bc_id, BlendMode::Multiply).expect("Failed to set blend mode");
    pipeline.set_effect_opacity(bc_id, 0.8).expect("Failed to set opacity");
    
    assert_eq!(pipeline.get_effects()[0].blend_mode, BlendMode::Multiply);
    assert_eq!(pipeline.get_effects()[0].opacity, 0.8);
}

#[tokio::test]
async fn test_frame_processing() {
    let frame = create_test_frame(64, 64, 128);
    
    // Test blur effect
    let mut blur = FilterFactory::create_blur(5.0).unwrap();
    let processed = blur.apply(&frame, 0.0).expect("Failed to apply blur");
    
    assert_eq!(processed.width, frame.width);
    assert_eq!(processed.height, frame.height);
    assert_eq!(processed.format, frame.format);
    
    // Test brightness/contrast effect
    let mut bc = FilterFactory::create_brightness_contrast(0.2, 1.5).unwrap();
    let processed = bc.apply(&frame, 0.0).expect("Failed to apply brightness/contrast");
    
    // The processed frame should be different from the original
    assert_ne!(processed.data, frame.data);
}

#[tokio::test]
async fn test_keyframe_animation() {
    let mut bc = FilterFactory::create_brightness_contrast(0.0, 1.0).unwrap();
    
    // Cast to keyframable trait
    let keyframable = bc.as_mut() as &mut dyn Effect;
    
    // Set static parameters
    keyframable.set_parameter("brightness", ParameterValue::Float(0.0)).unwrap();
    keyframable.set_parameter("contrast", ParameterValue::Float(1.0)).unwrap();
    
    let frame = create_test_frame(32, 32, 128);
    
    // Test processing at different times
    let result1 = keyframable.apply(&frame, 0.0).unwrap();
    let result2 = keyframable.apply(&frame, 1.0).unwrap();
    
    // Results should be the same since no keyframes are set
    assert_eq!(result1.data, result2.data);
}

#[tokio::test]
async fn test_transitions() {
    let frame1 = create_test_frame(64, 64, 0);   // Black
    let frame2 = create_test_frame(64, 64, 255); // White
    
    // Test fade transition
    let mut fade = FadeTransition::new();
    fade.set_duration(2.0);
    assert_eq!(fade.duration(), 2.0);
    assert_eq!(fade.transition_type(), TransitionType::Fade);
    
    // Test transition at different progress values
    let result_start = fade.apply_transition(&frame1, &frame2, 0.0, 0.0).unwrap();
    let result_mid = fade.apply_transition(&frame1, &frame2, 0.5, 1.0).unwrap();
    let result_end = fade.apply_transition(&frame1, &frame2, 1.0, 2.0).unwrap();
    
    // At start, should be mostly like frame1 (black)
    assert!(result_start.data[0] < 50);
    
    // At middle, should be gray
    assert!(result_mid.data[0] > 100 && result_mid.data[0] < 200);
    
    // At end, should be mostly like frame2 (white)
    assert!(result_end.data[0] > 200);
}

#[tokio::test]
async fn test_transition_factory() {
    let factory = DefaultTransitionFactory::new();
    
    // Test creating transitions
    let fade = factory.create("fade").expect("Failed to create fade transition");
    assert_eq!(fade.name(), "Fade");
    assert_eq!(fade.transition_type(), TransitionType::Fade);
    
    let dissolve = factory.create("dissolve").expect("Failed to create dissolve transition");
    assert_eq!(dissolve.name(), "Dissolve");
    assert_eq!(dissolve.transition_type(), TransitionType::Dissolve);
    
    // Test listing transitions
    let transitions = factory.available_transitions();
    assert!(transitions.contains(&"fade".to_string()));
    assert!(transitions.contains(&"dissolve".to_string()));
    assert!(transitions.contains(&"wipe_left".to_string()));
}

#[tokio::test]
async fn test_batch_processing() {
    let gpu_context = Arc::new(GpuContext::new().await.expect("Failed to create GPU context"));
    let mut pipeline = EffectPipeline::with_gpu_context(gpu_context.clone())
        .await
        .expect("Failed to create pipeline");
    
    let factory = DefaultEffectFactory::new();
    let blur_effect = factory.create("blur").unwrap();
    pipeline.add_effect(blur_effect);
    
    // Create multiple frames
    let frames = vec![
        create_test_frame(32, 32, 64),
        create_test_frame(32, 32, 128),
        create_test_frame(32, 32, 192),
    ];
    
    // Process batch
    let results = pipeline
        .process_frames_batch(&frames, 0.0, 1.0 / 30.0)
        .await
        .expect("Failed to process batch");
    
    assert_eq!(results.len(), frames.len());
    
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.width, frames[i].width);
        assert_eq!(result.height, frames[i].height);
        assert_eq!(result.format, frames[i].format);
    }
}

#[tokio::test]
async fn test_pipeline_integration() {
    let gpu_context = Arc::new(GpuContext::new().await.expect("Failed to create GPU context"));
    let mut pipeline = EffectPipeline::with_gpu_context(gpu_context.clone())
        .await
        .expect("Failed to create pipeline");
    
    let factory = DefaultEffectFactory::new();
    
    // Create a complex pipeline
    let blur_effect = factory.create("blur").unwrap();
    let blur_id = pipeline.add_effect(blur_effect);
    
    let bc_effect = factory.create("brightness_contrast").unwrap();
    let bc_id = pipeline.add_effect(bc_effect);
    
    let cc_effect = factory.create("color_correction").unwrap();
    let cc_id = pipeline.add_effect(cc_effect);
    
    // Configure effects
    pipeline.set_effect_opacity(bc_id, 0.8).unwrap();
    pipeline.set_effect_blend_mode(cc_id, BlendMode::Overlay).unwrap();
    
    // Process frame
    let frame = create_test_frame(128, 128, 128);
    let result = pipeline.process_frame(&frame, 0.0).await.expect("Failed to process frame");
    
    assert_eq!(result.width, frame.width);
    assert_eq!(result.height, frame.height);
    assert_eq!(result.format, frame.format);
    
    // The result should be different from the original
    assert_ne!(result.data, frame.data);
}

#[tokio::test]
async fn test_parameter_validation() {
    let mut blur = FilterFactory::create_blur(5.0).unwrap();
    
    // Test valid parameter
    assert!(blur.set_parameter("radius", ParameterValue::Float(10.0)).is_ok());
    
    // Test invalid parameter value
    assert!(blur.set_parameter("radius", ParameterValue::Float(-5.0)).is_err());
    assert!(blur.set_parameter("radius", ParameterValue::Float(100.0)).is_err());
    
    // Test unknown parameter
    assert!(blur.set_parameter("unknown", ParameterValue::Float(1.0)).is_err());
}

#[tokio::test]
async fn test_effect_cloning() {
    let mut original = FilterFactory::create_brightness_contrast(0.2, 1.5).unwrap();
    original.set_parameter("brightness", ParameterValue::Float(0.3)).unwrap();
    
    let cloned = original.clone_effect();
    
    assert_eq!(original.name(), cloned.name());
    assert_eq!(original.id() != cloned.id(), true); // IDs should be different
    
    // Parameters should be copied
    let orig_params = original.parameters();
    let cloned_params = cloned.parameters();
    
    assert_eq!(orig_params.len(), cloned_params.len());
}