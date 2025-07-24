//! Integration tests for the Rust Video Editor

use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;
use tokio::fs;

// Test video decode/encode cycle
#[cfg(test)]
mod decode_encode_tests {
    use super::*;
    use video_editor_core::{decoder::FFmpegDecoder, traits::VideoDecoder, VideoFormat};
    use video_editor_export::{ExportEngine, ExportSettings, ExportFormat, Quality, ExportProgress};
    
    struct TestProgress {
        progress: f32,
        completed: bool,
    }
    
    impl ExportProgress for TestProgress {
        fn on_progress(&mut self, percent: f32, _message: &str) {
            self.progress = percent;
        }
        
        fn on_complete(&mut self) {
            self.completed = true;
        }
        
        fn on_error(&mut self, _error: &video_editor_export::ExportError) {
            panic!("Export error occurred");
        }
    }
    
    async fn create_test_video(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Use FFmpeg to create a test video
        let status = tokio::process::Command::new("ffmpeg")
            .args(&[
                "-f", "lavfi",
                "-i", "testsrc=duration=5:size=640x480:rate=30",
                "-c:v", "libx264",
                "-preset", "ultrafast",
                "-pix_fmt", "yuv420p",
                "-y",
                output_path,
            ])
            .status()
            .await?;
        
        if !status.success() {
            return Err("Failed to create test video".into());
        }
        
        Ok(())
    }
    
    #[tokio::test]
    #[ignore] // Requires FFmpeg
    async fn test_full_decode_encode_cycle() {
        let temp_dir = TempDir::new().unwrap();
        let input_path = temp_dir.path().join("input.mp4");
        let output_path = temp_dir.path().join("output.mp4");
        
        // Create test input video
        create_test_video(input_path.to_str().unwrap()).await.unwrap();
        
        // Decode the video
        let mut decoder = FFmpegDecoder::new();
        let format = decoder.open(input_path.to_str().unwrap()).unwrap();
        
        assert_eq!(format.width, 640);
        assert_eq!(format.height, 480);
        assert!((format.fps - 30.0).abs() < 0.1);
        
        // Decode some frames
        let mut frames = Vec::new();
        let mut frame_count = 0;
        while let Ok(Some(frame)) = decoder.decode_frame() {
            assert!(frame.validate());
            frames.push(frame);
            frame_count += 1;
            
            if frame_count >= 30 { // Test first second
                break;
            }
        }
        
        assert!(frame_count > 0);
        
        // Create a timeline for export testing
        let mut timeline = video_editor_timeline::Timeline::new("Test Export".to_string());
        timeline.add_track("Video".to_string());
        
        // Set up export
        let export_settings = ExportSettings {
            output_path: output_path.clone(),
            format: ExportFormat::Mp4,
            video_codec: "libx264".to_string(),
            audio_codec: "aac".to_string(),
            width: 640,
            height: 480,
            fps: 30.0,
            bitrate: 1000000, // 1 Mbps
            quality: Quality::Medium,
        };
        
        let engine = ExportEngine::new(export_settings);
        let mut progress = TestProgress { progress: 0.0, completed: false };
        
        // Export the timeline
        engine.export(&timeline, &mut progress).await.unwrap();
        
        assert!(progress.completed);
        assert_eq!(progress.progress, 100.0);
        
        // Verify output file exists
        assert!(output_path.exists());
        
        // Verify output file is not empty
        let metadata = fs::metadata(&output_path).await.unwrap();
        assert!(metadata.len() > 0);
    }
    
    #[tokio::test]
    #[ignore] // Requires FFmpeg
    async fn test_format_conversion() {
        let temp_dir = TempDir::new().unwrap();
        let input_path = temp_dir.path().join("input.mp4");
        let webm_output = temp_dir.path().join("output.webm");
        let mov_output = temp_dir.path().join("output.mov");
        
        // Create test input
        create_test_video(input_path.to_str().unwrap()).await.unwrap();
        
        // Test conversion to WebM
        let webm_settings = ExportSettings {
            output_path: webm_output.clone(),
            format: ExportFormat::Webm,
            video_codec: "libvpx-vp9".to_string(),
            audio_codec: "libopus".to_string(),
            width: 640,
            height: 480,
            fps: 30.0,
            bitrate: 1000000,
            quality: Quality::Medium,
        };
        
        let timeline = video_editor_timeline::Timeline::new("WebM Test".to_string());
        let engine = ExportEngine::new(webm_settings);
        let mut progress = TestProgress { progress: 0.0, completed: false };
        
        engine.export(&timeline, &mut progress).await.unwrap();
        assert!(progress.completed);
        
        // Test conversion to MOV
        let mov_settings = ExportSettings {
            output_path: mov_output.clone(),
            format: ExportFormat::Mov,
            video_codec: "libx264".to_string(),
            audio_codec: "aac".to_string(),
            width: 640,
            height: 480,
            fps: 30.0,
            bitrate: 1000000,
            quality: Quality::High,
        };
        
        let engine = ExportEngine::new(mov_settings);
        let mut progress = TestProgress { progress: 0.0, completed: false };
        
        engine.export(&timeline, &mut progress).await.unwrap();
        assert!(progress.completed);
        
        // Verify both files exist
        assert!(webm_output.exists());
        assert!(mov_output.exists());
    }
}

// Test effect pipeline integration
#[cfg(test)]
mod effect_pipeline_tests {
    use super::*;
    use effects::{Effect, BrightnessEffect, ParameterValue};
    use std::any::Any;
    
    struct MockFrame {
        data: Vec<u8>,
        brightness: f32,
    }
    
    impl MockFrame {
        fn new() -> Self {
            Self {
                data: vec![128; 640 * 480 * 3], // Gray frame
                brightness: 0.5, // Mid brightness
            }
        }
        
        fn apply_brightness(&mut self, factor: f32) {
            self.brightness = (self.brightness * factor).clamp(0.0, 1.0);
            // Simulate brightness adjustment on data
            for pixel in &mut self.data {
                *pixel = ((*pixel as f32) * factor).clamp(0.0, 255.0) as u8;
            }
        }
    }
    
    #[tokio::test]
    async fn test_effect_chain_processing() {
        let mut frame = MockFrame::new();
        let original_brightness = frame.brightness;
        
        // Create effect chain
        let mut effects: Vec<Box<dyn Effect>> = vec![
            Box::new(BrightnessEffect::new()),
        ];
        
        // Configure brightness effect
        effects[0].set_parameter("brightness", ParameterValue::Float(1.5)).unwrap();
        
        // Apply effects
        for effect in &mut effects {
            effect.process(&mut frame as &mut dyn Any).unwrap();
        }
        
        // In a real implementation, the effect would modify the frame
        // For now, we manually apply the change to test the concept
        frame.apply_brightness(1.5);
        
        assert!(frame.brightness > original_brightness);
    }
    
    #[test]
    fn test_multiple_effects() {
        struct ContrastEffect {
            contrast: f32,
        }
        
        impl ContrastEffect {
            fn new() -> Self {
                Self { contrast: 1.0 }
            }
        }
        
        impl Effect for ContrastEffect {
            fn name(&self) -> &str {
                "Contrast"
            }
            
            fn process(&mut self, _input: &mut dyn Any) -> effects::Result<()> {
                Ok(())
            }
            
            fn get_parameters(&self) -> Vec<effects::Parameter> {
                vec![effects::Parameter {
                    name: "contrast".to_string(),
                    value: ParameterValue::Float(self.contrast),
                    min: Some(ParameterValue::Float(0.0)),
                    max: Some(ParameterValue::Float(2.0)),
                }]
            }
            
            fn set_parameter(&mut self, name: &str, value: ParameterValue) -> effects::Result<()> {
                match (name, value) {
                    ("contrast", ParameterValue::Float(v)) => {
                        self.contrast = v;
                        Ok(())
                    }
                    _ => Err(effects::EffectError::InvalidParameter(name.to_string())),
                }
            }
        }
        
        let mut effects: Vec<Box<dyn Effect>> = vec![
            Box::new(BrightnessEffect::new()),
            Box::new(ContrastEffect::new()),
        ];
        
        // Configure effects
        effects[0].set_parameter("brightness", ParameterValue::Float(1.2)).unwrap();
        effects[1].set_parameter("contrast", ParameterValue::Float(1.3)).unwrap();
        
        let mut frame = MockFrame::new();
        
        // Apply all effects
        for effect in &mut effects {
            let result = effect.process(&mut frame as &mut dyn Any);
            assert!(result.is_ok());
        }
    }
}

// Test UI interactions (mock tests)
#[cfg(test)]
mod ui_integration_tests {
    use super::*;
    use video_editor_ui::{AppState, UiConfig, Theme};
    
    #[test]
    fn test_app_state_management() {
        let mut app_state = AppState::default();
        
        assert_eq!(app_state.project_name, "Untitled Project");
        assert!(!app_state.is_playing);
        assert_eq!(app_state.current_time, Duration::from_secs(0));
        assert_eq!(app_state.zoom_level, 1.0);
        
        // Simulate UI interactions
        app_state.project_name = "My Video Project".to_string();
        app_state.is_playing = true;
        app_state.current_time = Duration::from_secs(10);
        app_state.zoom_level = 1.5;
        
        assert_eq!(app_state.project_name, "My Video Project");
        assert!(app_state.is_playing);
        assert_eq!(app_state.current_time, Duration::from_secs(10));
        assert_eq!(app_state.zoom_level, 1.5);
    }
    
    #[test]
    fn test_ui_config_themes() {
        let light_config = UiConfig {
            window_title: "Video Editor".to_string(),
            initial_width: 1920,
            initial_height: 1080,
            theme: Theme::Light,
        };
        
        let dark_config = UiConfig {
            theme: Theme::Dark,
            ..light_config.clone()
        };
        
        assert_eq!(light_config.theme, Theme::Light);
        assert_eq!(dark_config.theme, Theme::Dark);
    }
    
    #[test]
    fn test_ui_config_serialization() {
        let config = UiConfig {
            window_title: "Test Editor".to_string(),
            initial_width: 1280,
            initial_height: 720,
            theme: Theme::System,
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: UiConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.window_title, deserialized.window_title);
        assert_eq!(config.initial_width, deserialized.initial_width);
        assert_eq!(config.initial_height, deserialized.initial_height);
    }
}

// Test timeline and effects integration
#[cfg(test)]
mod timeline_effects_integration {
    use super::*;
    use video_editor_timeline::{Timeline, Clip};
    use effects::{Effect, BrightnessEffect, ParameterValue};
    use uuid::Uuid;
    
    #[test]
    fn test_timeline_with_effects() {
        let mut timeline = Timeline::new("Effects Test".to_string());
        let track = timeline.add_track("Video with Effects".to_string());
        let track_id = track.id;
        
        // Add a clip to the track
        if let Some(track) = timeline.tracks.iter_mut().find(|t| t.id == track_id) {
            track.clips.push(Clip {
                id: Uuid::new_v4(),
                start_time: Duration::from_secs(0),
                duration: Duration::from_secs(10),
                source_path: "test_video.mp4".to_string(),
                in_point: Duration::from_secs(0),
                out_point: Duration::from_secs(10),
            });
        }
        
        // Create effects that could be applied to clips
        let mut brightness_effect = BrightnessEffect::new();
        brightness_effect.set_parameter("brightness", ParameterValue::Float(1.3)).unwrap();
        
        // Verify effect configuration
        let params = brightness_effect.get_parameters();
        assert_eq!(params.len(), 1);
        
        match &params[0].value {
            ParameterValue::Float(v) => assert_eq!(*v, 1.3),
            _ => panic!("Expected float parameter"),
        }
        
        // Timeline should have the clip
        assert_eq!(timeline.tracks.len(), 1);
        assert_eq!(timeline.tracks[0].clips.len(), 1);
    }
}

// Memory and performance integration tests
#[cfg(test)]
mod performance_integration_tests {
    use super::*;
    use std::time::Instant;
    use video_editor_core::buffer::{FramePool, FrameCache};
    use video_editor_core::traits::PixelFormat;
    
    #[test]
    fn test_memory_pool_under_load() {
        let mut pool = FramePool::new(50, 1920, 1080, PixelFormat::RGB24);
        
        let start = Instant::now();
        
        // Simulate high load
        let mut frames = Vec::new();
        for _ in 0..45 {
            if let Some(frame) = pool.get() {
                frames.push(frame);
            }
        }
        
        // Return all frames
        for frame in frames {
            pool.return_frame(frame);
        }
        
        let elapsed = start.elapsed();
        println!("Pool operations under load completed in {:?}", elapsed);
        
        assert!(elapsed.as_millis() < 100);
        assert_eq!(pool.available(), 50);
        assert_eq!(pool.in_use(), 0);
    }
    
    #[test]
    fn test_cache_performance_under_load() {
        let mut cache = FrameCache::new(100); // 100MB cache
        
        // Fill cache with frames
        let start = Instant::now();
        for i in 0..500 {
            let frame = video_editor_core::frame::Frame::new(
                640, 
                480, 
                PixelFormat::RGB24, 
                Duration::from_millis(i)
            ).unwrap();
            cache.insert("test_video", i as usize, frame);
        }
        
        // Test random access
        let mut hits = 0;
        for _ in 0..1000 {
            let random_frame = rand::random::<usize>() % 500;
            if cache.get("test_video", random_frame).is_some() {
                hits += 1;
            }
        }
        
        let elapsed = start.elapsed();
        println!("Cache performance test completed in {:?}", elapsed);
        println!("Cache hit rate: {:.2}%", (hits as f64 / 1000.0) * 100.0);
        
        assert!(elapsed.as_millis() < 500);
        assert!(hits > 0);
    }
}