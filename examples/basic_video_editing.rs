//! Basic video editing example demonstrating core functionality
//! 
//! This example shows how to:
//! - Load a video file
//! - Apply basic effects
//! - Create a simple timeline
//! - Export the result

use rust_video_core::{VideoDecoder, VideoEncoder, VideoFormat, Frame, Result};
use video_editor_timeline::{Timeline, Clip, Track};
use video_editor_effects::{Effect, BrightnessEffect, ContrastEffect};
use video_editor_export::{ExportEngine, ExportSettings, ExportFormat, Quality};
use std::time::Duration;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Rust Video Editor - Basic Editing Example");
    println!("=========================================\n");
    
    // Create a new timeline
    let mut timeline = create_sample_timeline()?;
    
    // Apply effects to clips
    apply_effects_to_timeline(&mut timeline)?;
    
    // Export the timeline
    export_timeline(&timeline).await?;
    
    println!("\nVideo editing completed successfully!");
    Ok(())
}

/// Creates a sample timeline with multiple clips
fn create_sample_timeline() -> Result<Timeline> {
    println!("Creating timeline...");
    
    let mut timeline = Timeline::new("My Video Project".to_string());
    
    // Add video track
    let video_track = timeline.add_track("Video Track 1".to_string());
    let video_track_id = video_track.id;
    
    // Add audio track
    let audio_track = timeline.add_track("Audio Track 1".to_string());
    let audio_track_id = audio_track.id;
    
    // Add clips to video track
    let clip1 = Clip {
        id: uuid::Uuid::new_v4(),
        start_time: Duration::from_secs(0),
        duration: Duration::from_secs(5),
        source_path: "assets/clip1.mp4".to_string(),
        in_point: Duration::from_secs(0),
        out_point: Duration::from_secs(5),
    };
    
    let clip2 = Clip {
        id: uuid::Uuid::new_v4(),
        start_time: Duration::from_secs(5),
        duration: Duration::from_secs(7),
        source_path: "assets/clip2.mp4".to_string(),
        in_point: Duration::from_secs(2), // Start 2 seconds into the source
        out_point: Duration::from_secs(9),
    };
    
    // Add clips to timeline
    if let Some(track) = timeline.tracks.iter_mut().find(|t| t.id == video_track_id) {
        track.clips.push(clip1);
        track.clips.push(clip2);
    }
    
    // Update timeline duration
    timeline.duration = Duration::from_secs(12);
    
    println!("Timeline created with {} tracks and {} total clips", 
             timeline.tracks.len(), 
             timeline.tracks.iter().map(|t| t.clips.len()).sum::<usize>());
    
    Ok(timeline)
}

/// Applies various effects to clips in the timeline
fn apply_effects_to_timeline(timeline: &mut Timeline) -> Result<()> {
    println!("\nApplying effects to clips...");
    
    // In a real implementation, effects would be stored with clips
    // For this example, we'll demonstrate the effect API
    
    let mut brightness_effect = BrightnessEffect::new();
    brightness_effect.set_parameter("brightness", 
        video_editor_effects::ParameterValue::Float(1.2))?;
    
    let mut contrast_effect = video_editor_effects::BrightnessEffect::new();
    contrast_effect.set_parameter("brightness", 
        video_editor_effects::ParameterValue::Float(1.1))?;
    
    println!("Applied brightness adjustment: +20%");
    println!("Applied contrast adjustment: +10%");
    
    Ok(())
}

/// Exports the timeline to a video file
async fn export_timeline(timeline: &Timeline) -> Result<()> {
    println!("\nExporting timeline...");
    
    // Configure export settings
    let export_settings = ExportSettings {
        output_path: Path::new("output/edited_video.mp4").to_path_buf(),
        format: ExportFormat::Mp4,
        video_codec: "h264".to_string(),
        audio_codec: "aac".to_string(),
        width: 1920,
        height: 1080,
        fps: 30.0,
        bitrate: 8_000_000, // 8 Mbps
        quality: Quality::High,
    };
    
    // Create export engine
    let export_engine = ExportEngine::new(export_settings);
    
    // Create progress handler
    struct ConsoleProgress {
        last_percent: f32,
    }
    
    impl video_editor_export::ExportProgress for ConsoleProgress {
        fn on_progress(&mut self, percent: f32, message: &str) {
            if percent - self.last_percent >= 5.0 {
                println!("Export progress: {:.0}% - {}", percent, message);
                self.last_percent = percent;
            }
        }
        
        fn on_complete(&mut self) {
            println!("Export completed!");
        }
        
        fn on_error(&mut self, error: &video_editor_export::ExportError) {
            eprintln!("Export error: {}", error);
        }
    }
    
    let mut progress = ConsoleProgress { last_percent: 0.0 };
    
    // Perform export
    export_engine.export(timeline, &mut progress).await?;
    
    Ok(())
}

/// Example of processing individual frames
async fn process_frames_example() -> Result<()> {
    use rust_video_core::{decoder::FFmpegDecoder, traits::VideoDecoder};
    
    // Create decoder
    let mut decoder = FFmpegDecoder::new();
    
    // Open video file
    let format = decoder.open("input.mp4")?;
    println!("Video format: {}x{} @ {} fps", format.width, format.height, format.fps);
    
    // Process frames
    let mut frame_count = 0;
    while let Some(mut frame) = decoder.decode_frame()? {
        // Apply some processing to the frame
        apply_custom_effect(&mut frame)?;
        
        frame_count += 1;
        if frame_count % 30 == 0 {
            println!("Processed {} frames", frame_count);
        }
    }
    
    decoder.close()?;
    Ok(())
}

/// Example of a custom frame processing effect
fn apply_custom_effect(frame: &mut Frame) -> Result<()> {
    let width = frame.metadata().width as usize;
    let height = frame.metadata().height as usize;
    let pixels = frame.pixels_mut();
    
    // Apply a simple vignette effect
    for y in 0..height {
        for x in 0..width {
            // Calculate distance from center
            let cx = width as f32 / 2.0;
            let cy = height as f32 / 2.0;
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let distance = (dx * dx + dy * dy).sqrt();
            let max_distance = (cx * cx + cy * cy).sqrt();
            
            // Calculate vignette factor
            let vignette = 1.0 - (distance / max_distance).min(1.0);
            let factor = 0.5 + 0.5 * vignette;
            
            // Apply to pixel
            let idx = (y * width + x) * 3;
            if idx + 3 <= pixels.len() {
                pixels[idx] = (pixels[idx] as f32 * factor) as u8;
                pixels[idx + 1] = (pixels[idx + 1] as f32 * factor) as u8;
                pixels[idx + 2] = (pixels[idx + 2] as f32 * factor) as u8;
            }
        }
    }
    
    Ok(())
}

/// Example of working with timeline transitions
fn add_transitions_example(timeline: &mut Timeline) -> Result<()> {
    use video_editor_timeline::{Transition, TransitionType};
    
    // Find adjacent clips
    for track in &mut timeline.tracks {
        for i in 0..track.clips.len().saturating_sub(1) {
            let clip1_end = track.clips[i].start_time + track.clips[i].duration;
            let clip2_start = track.clips[i + 1].start_time;
            
            // If clips are adjacent, add a transition
            if clip1_end == clip2_start {
                let transition = Transition {
                    id: uuid::Uuid::new_v4(),
                    transition_type: TransitionType::Dissolve,
                    duration: Duration::from_millis(500),
                    clip1_id: track.clips[i].id,
                    clip2_id: track.clips[i + 1].id,
                };
                
                // In a real implementation, transitions would be stored
                println!("Added dissolve transition between clips");
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timeline_creation() {
        let timeline = create_sample_timeline().unwrap();
        assert_eq!(timeline.tracks.len(), 2);
        assert_eq!(timeline.name, "My Video Project");
    }
}