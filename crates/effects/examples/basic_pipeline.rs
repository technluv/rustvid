use effects::{
    filters::FilterFactory,
    gpu::GpuContext,
    parameters::{Keyframe, InterpolationType, ParameterValue},
    pipeline::EffectPipeline,
    traits::*,
    transitions::{fade::FadeTransition, DefaultTransitionFactory},
    DefaultEffectFactory,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Effects System Demo");
    
    // Create a sample frame
    let frame = Frame {
        width: 1920,
        height: 1080,
        data: vec![128; 1920 * 1080 * 4], // Gray frame
        format: PixelFormat::Rgba8,
        timestamp: 0.0,
    };
    
    // Initialize GPU context
    let gpu_context = Arc::new(GpuContext::new().await?);
    println!("GPU Context initialized: {:?}", gpu_context.adapter_info);
    
    // Create effect pipeline
    let mut pipeline = EffectPipeline::with_gpu_context(gpu_context.clone()).await?;
    
    // Create effects using factory
    let factory = DefaultEffectFactory::new();
    
    // Add blur effect
    let blur_effect = factory.create("blur")?;
    let blur_id = pipeline.add_effect(blur_effect);
    
    // Add brightness/contrast effect
    let mut bc_effect = factory.create("brightness_contrast")?;
    
    // Add keyframes for brightness animation
    let keyframe1 = Keyframe {
        time: 0.0,
        value: ParameterValue::Float(0.0),
        interpolation: InterpolationType::Linear,
    };
    let keyframe2 = Keyframe {
        time: 2.0,
        value: ParameterValue::Float(0.5),
        interpolation: InterpolationType::Linear,
    };
    
    if let Ok(keyframable) = bc_effect.as_any_mut().downcast_mut::<dyn effects::traits::Keyframable>() {
        keyframable.add_keyframe(0.0, "brightness", ParameterValue::Float(0.0))?;
        keyframable.add_keyframe(2.0, "brightness", ParameterValue::Float(0.5))?;
        println!("Added keyframes for brightness animation");
    }
    
    let bc_id = pipeline.add_effect(bc_effect);
    
    // Process frame through pipeline
    println!("Processing frame through effect pipeline...");
    let processed_frame = pipeline.process_frame(&frame, 1.0).await?;
    println!("Frame processed successfully!");
    
    // Create transition
    let transition_factory = DefaultTransitionFactory::new();
    let mut fade_transition = transition_factory.create("fade")?;
    fade_transition.set_duration(1.5);
    
    // Create second frame for transition
    let frame2 = Frame {
        width: 1920,
        height: 1080,
        data: vec![255; 1920 * 1080 * 4], // White frame
        format: PixelFormat::Rgba8,
        timestamp: 1.0,
    };
    
    // Apply transition
    println!("Applying fade transition...");
    let transition_result = fade_transition.apply_transition(&frame, &frame2, 0.5, 0.5)?;
    println!("Transition applied successfully!");
    
    // Demonstrate batch processing
    let frames = vec![frame.clone(), frame2.clone(), processed_frame];
    println!("Processing batch of {} frames...", frames.len());
    
    let batch_results = pipeline
        .process_frames_batch(&frames, 0.0, 1.0 / 30.0)
        .await?;
    println!("Batch processing completed: {} frames processed", batch_results.len());
    
    // List available effects
    println!("\nAvailable effects:");
    for effect_name in factory.available_effects() {
        if let Some(metadata) = factory.effect_metadata(&effect_name) {
            println!("- {}: {}", metadata.name, metadata.description);
            println!("  Category: {}, GPU Required: {}", metadata.category, metadata.requires_gpu);
            for param in &metadata.parameters {
                println!("    Parameter: {} ({})", param.display_name, param.description);
            }
        }
    }
    
    // List available transitions
    println!("\nAvailable transitions:");
    for transition_name in transition_factory.available_transitions() {
        println!("- {}", transition_name);
    }
    
    // Demonstrate filter factory
    println!("\nCreating effects with filter factory:");
    
    let blur_filter = FilterFactory::create_blur(10.0)?;
    println!("Created blur filter with radius 10.0");
    
    let color_correction = FilterFactory::create_color_correction(0.1, 1.2, 0.0, 0.2, -0.1)?;
    println!("Created color correction filter with custom parameters");
    
    let levels = FilterFactory::create_levels(0.0, 1.0, 1.2, 0.0, 1.0)?;
    println!("Created levels adjustment filter");
    
    println!("\nDemo completed successfully!");
    
    Ok(())
}

// Helper trait for downcasting
trait AsAnyMut {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> AsAnyMut for T {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}