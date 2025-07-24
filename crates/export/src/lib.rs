//! Rendering and export functionality for Rust Video Editor
//! 
//! This crate handles the final rendering pipeline and export
//! to various video formats.

mod encoder;
mod pipeline;
mod presets;
mod job;

pub use encoder::{VideoEncoder, VideoCodec, HardwareEncoder, EncoderConfig};
pub use pipeline::{RenderPipeline, PipelineConfig, RenderFrame, RenderPipelineHandle};
pub use presets::{ExportPreset, ExportSettingsBuilder};
pub use job::{ExportJob, JobStatus, JobPriority, ExportJobManager, JobStatistics};

use thiserror::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::{info, error};

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Export failed: {0}")]
    ExportFailed(String),
    
    #[error("Invalid export settings")]
    InvalidSettings,
    
    #[error("Codec not supported: {0}")]
    UnsupportedCodec(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ExportError>;

/// Export settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportSettings {
    pub output_path: PathBuf,
    pub format: ExportFormat,
    pub video_codec: String,
    pub audio_codec: String,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub bitrate: u32,
    pub quality: Quality,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ExportFormat {
    Mp4,
    Webm,
    Mov,
    Avi,
    Mkv,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Quality {
    Low,
    Medium,
    High,
    Ultra,
    Custom(u8), // 0-100
}

/// Progress callback for export operations
pub trait ExportProgress: Send + Sync {
    fn on_progress(&mut self, percent: f32, message: &str);
    fn on_complete(&mut self);
    fn on_error(&mut self, error: &ExportError);
}

/// Main export engine
pub struct ExportEngine {
    settings: ExportSettings,
    job_manager: Option<Arc<ExportJobManager>>,
}

impl ExportEngine {
    pub fn new(settings: ExportSettings) -> Self {
        Self { 
            settings,
            job_manager: None,
        }
    }
    
    /// Create a new export engine with job management
    pub fn with_job_manager(settings: ExportSettings, job_manager: Arc<ExportJobManager>) -> Self {
        Self {
            settings,
            job_manager: Some(job_manager),
        }
    }
    
    /// Export a timeline to video
    pub async fn export<P: ExportProgress + Send + 'static>(
        &self,
        timeline: Arc<video_editor_timeline::Timeline>,
        progress: Arc<Mutex<P>>,
    ) -> Result<()> {
        // Validate settings
        if self.settings.width == 0 || self.settings.height == 0 {
            return Err(ExportError::InvalidSettings);
        }
        
        info!("Starting export: {:?}", self.settings.output_path);
        
        // Create encoder config
        let encoder_config = EncoderConfig::from(&self.settings);
        
        // Initialize video encoder
        let mut encoder = VideoEncoder::new(encoder_config.clone(), &self.settings.output_path)?;
        encoder.initialize()?;
        
        // Create effect processor
        let effect_processor = Arc::new(video_editor_effects::EffectProcessor::new());
        
        // Create render pipeline
        let pipeline_config = PipelineConfig {
            resolution: video_editor_core::Resolution::new(self.settings.width, self.settings.height),
            fps: self.settings.fps,
            pixel_format: video_editor_core::PixelFormat::RGBA8,
            buffer_size: 60,
            worker_threads: num_cpus::get(),
        };
        
        let pipeline = RenderPipeline::new(
            pipeline_config,
            timeline,
            effect_processor,
        );
        
        // Start render pipeline
        let pipeline_handle = pipeline.start(progress.clone())?;
        let frame_receiver = pipeline_handle.frame_receiver();
        let total_frames = pipeline_handle.total_frames();
        
        // Encode frames as they come from the pipeline
        let encoding_thread = std::thread::spawn(move || -> Result<()> {
            let mut encoded_frames = 0u64;
            
            while let Ok(frame) = frame_receiver.recv() {
                // Convert frame to format expected by encoder
                // Note: In a real implementation, you'd need to convert from RGBA to YUV420P
                encoder.encode_frame(&frame.data)?;
                
                encoded_frames += 1;
                
                // Update progress (50-100% range for encoding)
                let encoding_progress = 50.0 + (encoded_frames as f32 / total_frames as f32) * 50.0;
                if let Ok(mut p) = progress.lock() {
                    p.on_progress(
                        encoding_progress,
                        &format!("Encoding frame {}/{}", encoded_frames, total_frames)
                    );
                }
            }
            
            // Finalize encoding
            encoder.finalize()?;
            
            info!("Export completed successfully");
            if let Ok(mut p) = progress.lock() {
                p.on_complete();
            }
            
            Ok(())
        });
        
        // Wait for pipeline to complete
        pipeline_handle.wait()?;
        
        // Wait for encoding to complete
        encoding_thread.join()
            .map_err(|_| ExportError::ExportFailed("Encoding thread panicked".to_string()))??;
        
        Ok(())
    }
    
    /// Export with a preset
    pub async fn export_with_preset<P: ExportProgress + Send + 'static>(
        preset: ExportPreset,
        output_path: PathBuf,
        timeline: Arc<video_editor_timeline::Timeline>,
        progress: Arc<Mutex<P>>,
    ) -> Result<()> {
        let settings = preset.to_settings(output_path);
        let engine = ExportEngine::new(settings);
        engine.export(timeline, progress).await
    }
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            output_path: PathBuf::from("output.mp4"),
            format: ExportFormat::Mp4,
            video_codec: "h264".to_string(),
            audio_codec: "aac".to_string(),
            width: 1920,
            height: 1080,
            fps: 30.0,
            bitrate: 8000000, // 8 Mbps
            quality: Quality::High,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_settings_default() {
        let settings = ExportSettings::default();
        assert_eq!(settings.width, 1920);
        assert_eq!(settings.height, 1080);
        assert_eq!(settings.fps, 30.0);
    }
    
    #[test]
    fn test_export_settings_validation() {
        let mut settings = ExportSettings::default();
        settings.width = 0;
        
        let engine = ExportEngine::new(settings);
        
        struct DummyProgress;
        impl ExportProgress for DummyProgress {
            fn on_progress(&mut self, _: f32, _: &str) {}
            fn on_complete(&mut self) {}
            fn on_error(&mut self, _: &ExportError) {}
        }
        
        let timeline = video_editor_timeline::Timeline::new("Test".to_string());
        let mut progress = DummyProgress;
        
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(engine.export(&timeline, &mut progress));
        
        assert!(result.is_err());
    }
}