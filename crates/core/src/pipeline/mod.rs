//! Video processing pipeline for coordinating decoder, buffer, and processing stages
//! 
//! The pipeline provides a high-level interface for video processing operations,
//! managing the flow of frames through the system.

use crate::{
    decoder::ffmpeg::FFmpegDecoder,
    buffer::{FrameBuffer, FrameBufferConfig},
    traits::{VideoDecoder, VideoProcessor, PixelFormat},
    frame::Frame,
    error::{VideoError, Result},
};
use std::sync::Arc;
use std::path::Path;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;

/// Configuration for the video processing pipeline
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Frame buffer configuration
    pub buffer_config: FrameBufferConfig,
    /// Number of decoder threads
    pub decoder_threads: usize,
    /// Target pixel format for decoded frames
    pub target_pixel_format: PixelFormat,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            buffer_config: FrameBufferConfig::default(),
            decoder_threads: 2,
            target_pixel_format: PixelFormat::RGB24,
        }
    }
}

/// Video processing pipeline state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineState {
    /// Pipeline is idle
    Idle,
    /// Pipeline is decoding
    Decoding,
    /// Pipeline is paused
    Paused,
    /// Pipeline has encountered an error
    Error,
}

/// Main video processing pipeline
pub struct VideoPipeline {
    /// Frame buffer for decoded frames
    buffer: Arc<FrameBuffer>,
    /// Current pipeline state
    state: Arc<RwLock<PipelineState>>,
    /// Pipeline configuration
    config: PipelineConfig,
    /// Decoder task handle
    decoder_handle: Option<JoinHandle<()>>,
    /// Control channel for the decoder
    control_tx: Option<mpsc::Sender<PipelineControl>>,
}

/// Control messages for the pipeline
#[derive(Debug)]
enum PipelineControl {
    /// Start decoding
    Start,
    /// Pause decoding
    Pause,
    /// Resume decoding
    Resume,
    /// Stop decoding
    Stop,
    /// Seek to timestamp
    Seek(std::time::Duration),
}

impl VideoPipeline {
    /// Create a new video processing pipeline
    pub fn new(config: PipelineConfig) -> Self {
        let buffer = Arc::new(FrameBuffer::new(config.buffer_config.clone()));
        
        Self {
            buffer,
            state: Arc::new(RwLock::new(PipelineState::Idle)),
            config,
            decoder_handle: None,
            control_tx: None,
        }
    }
    
    /// Open a video file and prepare for processing
    pub async fn open_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        // Update state
        *self.state.write().await = PipelineState::Idle;
        
        // Create control channel
        let (control_tx, mut control_rx) = mpsc::channel(10);
        self.control_tx = Some(control_tx);
        
        // Clone necessary items for the decoder task
        let buffer = self.buffer.clone();
        let state = self.state.clone();
        let target_format = self.config.target_pixel_format;
        let path = path.as_ref().to_path_buf();
        
        // Spawn decoder task
        let decoder_handle = tokio::spawn(async move {
            let mut decoder = match FFmpegDecoder::new() {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Failed to create decoder: {}", e);
                    *state.write().await = PipelineState::Error;
                    return;
                }
            };
            
            // Open the file
            if let Err(e) = decoder.open_file(&path) {
                eprintln!("Failed to open file: {}", e);
                *state.write().await = PipelineState::Error;
                return;
            }
            
            // Get decoder channel
            let decoder_tx = buffer.get_decoder_channel();
            
            // Decoding loop
            let mut is_paused = false;
            loop {
                // Check for control messages
                if let Ok(msg) = control_rx.try_recv() {
                    match msg {
                        PipelineControl::Start | PipelineControl::Resume => {
                            is_paused = false;
                            *state.write().await = PipelineState::Decoding;
                        }
                        PipelineControl::Pause => {
                            is_paused = true;
                            *state.write().await = PipelineState::Paused;
                        }
                        PipelineControl::Stop => {
                            break;
                        }
                        PipelineControl::Seek(timestamp) => {
                            if let Err(e) = decoder.seek(timestamp) {
                                eprintln!("Seek failed: {}", e);
                            }
                        }
                    }
                }
                
                // Decode frame if not paused
                if !is_paused {
                    match decoder.decode_frame() {
                        Ok(Some(mut frame)) => {
                            // Convert to target pixel format if needed
                            if frame.pixel_format() != target_format {
                                if let Ok(converted) = decoder.convert_pixel_format(&frame, target_format) {
                                    frame = converted;
                                }
                            }
                            
                            // Send frame to buffer
                            if decoder_tx.send(frame).await.is_err() {
                                break; // Buffer closed
                            }
                        }
                        Ok(None) => {
                            // End of stream
                            break;
                        }
                        Err(e) => {
                            eprintln!("Decode error: {}", e);
                            // Continue trying to decode
                        }
                    }
                } else {
                    // Sleep briefly when paused
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
            }
            
            *state.write().await = PipelineState::Idle;
        });
        
        self.decoder_handle = Some(decoder_handle);
        Ok(())
    }
    
    /// Start or resume video processing
    pub async fn play(&mut self) -> Result<()> {
        if let Some(tx) = &self.control_tx {
            tx.send(PipelineControl::Start).await
                .map_err(|_| VideoError::ResourceError("Control channel closed".into()))?;
        }
        Ok(())
    }
    
    /// Pause video processing
    pub async fn pause(&mut self) -> Result<()> {
        if let Some(tx) = &self.control_tx {
            tx.send(PipelineControl::Pause).await
                .map_err(|_| VideoError::ResourceError("Control channel closed".into()))?;
        }
        Ok(())
    }
    
    /// Stop video processing
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(tx) = &self.control_tx {
            tx.send(PipelineControl::Stop).await
                .map_err(|_| VideoError::ResourceError("Control channel closed".into()))?;
        }
        
        // Wait for decoder to finish
        if let Some(handle) = self.decoder_handle.take() {
            let _ = handle.await;
        }
        
        Ok(())
    }
    
    /// Seek to a specific timestamp
    pub async fn seek(&mut self, timestamp: std::time::Duration) -> Result<()> {
        if let Some(tx) = &self.control_tx {
            tx.send(PipelineControl::Seek(timestamp)).await
                .map_err(|_| VideoError::ResourceError("Control channel closed".into()))?;
        }
        Ok(())
    }
    
    /// Get the next frame from the buffer
    pub async fn get_frame(&self) -> Option<Arc<Frame>> {
        self.buffer.get_frame().await
    }
    
    /// Get current pipeline state
    pub async fn get_state(&self) -> PipelineState {
        *self.state.read().await
    }
    
    /// Get buffer metrics
    pub async fn get_metrics(&self) -> crate::buffer::BufferMetrics {
        self.buffer.get_metrics().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_pipeline_creation() {
        let config = PipelineConfig::default();
        let pipeline = VideoPipeline::new(config);
        
        assert_eq!(pipeline.get_state().await, PipelineState::Idle);
    }
}