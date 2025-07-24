//! Example demonstrating video export functionality

use video_editor_export::*;
use video_editor_timeline::Timeline;
use video_editor_effects::EffectProcessor;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

/// Simple progress tracker
struct ConsoleProgress {
    last_percent: u32,
}

impl ExportProgress for ConsoleProgress {
    fn on_progress(&mut self, percent: f32, message: &str) {
        let current_percent = percent as u32;
        if current_percent != self.last_percent || current_percent % 10 == 0 {
            println!("[{:3}%] {}", current_percent, message);
            self.last_percent = current_percent;
        }
    }
    
    fn on_complete(&mut self) {
        println!("✓ Export completed successfully!");
    }
    
    fn on_error(&mut self, error: &ExportError) {
        eprintln!("✗ Export failed: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    println!("Video Export Demo");
    println!("=================\n");
    
    // Create a sample timeline
    let timeline = Arc::new(Timeline::new("Demo Project".to_string()));
    
    // List available presets
    println!("Available Export Presets:");
    for (category, presets) in ExportPreset::by_category() {
        println!("\n{}:", category);
        for preset in presets {
            println!("  - {} : {}", preset.display_name(), preset.description());
        }
    }
    
    // Example 1: Export with preset
    println!("\n\n1. Exporting with YouTube 1080p preset...");
    let progress = Arc::new(Mutex::new(ConsoleProgress { last_percent: 0 }));
    
    match ExportEngine::export_with_preset(
        ExportPreset::YouTube1080p,
        PathBuf::from("output_youtube.mp4"),
        timeline.clone(),
        progress.clone(),
    ).await {
        Ok(_) => println!("Export completed: output_youtube.mp4"),
        Err(e) => eprintln!("Export failed: {}", e),
    }
    
    // Example 2: Custom export settings
    println!("\n2. Exporting with custom settings...");
    let custom_settings = ExportSettingsBuilder::new()
        .output_path(PathBuf::from("output_custom.mp4"))
        .resolution(1280, 720)
        .fps(60.0)
        .bitrate(5_000_000)
        .quality(Quality::High)
        .video_codec("h265")
        .build();
    
    let engine = ExportEngine::new(custom_settings);
    let progress = Arc::new(Mutex::new(ConsoleProgress { last_percent: 0 }));
    
    match engine.export(timeline.clone(), progress).await {
        Ok(_) => println!("Export completed: output_custom.mp4"),
        Err(e) => eprintln!("Export failed: {}", e),
    }
    
    // Example 3: Export job management
    println!("\n3. Export job management demo...");
    let job_manager = Arc::new(ExportJobManager::new(2)); // Max 2 concurrent jobs
    
    // Create multiple export jobs
    let job_ids: Vec<_> = vec![
        ("Mobile Export", ExportPreset::MobileHigh, JobPriority::Normal),
        ("Web Export", ExportPreset::WebMP4, JobPriority::Low),
        ("Professional Export", ExportPreset::ProRes422, JobPriority::High),
    ].into_iter().map(|(name, preset, priority)| {
        let settings = preset.to_settings(PathBuf::from(format!("{}.{}", name.to_lowercase().replace(' ', "_"), preset.file_extension())));
        job_manager.create_job(name.to_string(), settings, priority)
    }).collect();
    
    // Display job queue
    println!("\nJob Queue:");
    for job in job_manager.get_all_jobs() {
        println!("  - {} [{}] Priority: {:?}", job.name, job.id, job.priority);
    }
    
    // Get statistics
    let stats = job_manager.get_statistics();
    println!("\nJob Statistics:");
    println!("  Total jobs: {}", stats.total_jobs);
    println!("  Queued: {}", stats.queued_jobs);
    println!("  Active: {}", stats.active_jobs);
    println!("  Completed: {}", stats.completed_jobs);
    
    // Cancel the low priority job
    if let Some(job_id) = job_ids.get(1) {
        println!("\nCancelling job: {}", job_id);
        job_manager.cancel_job(*job_id)?;
    }
    
    println!("\nDemo completed!");
    
    Ok(())
}