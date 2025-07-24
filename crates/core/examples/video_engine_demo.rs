//! Demonstration of the video engine with FFmpeg decoder and frame buffer
//! 
//! This example shows how to:
//! - Open a video file with FFmpeg
//! - Decode frames
//! - Use the frame buffer for efficient playback
//! - Monitor performance metrics

use rust_video_core::{
    decoder::ffmpeg::FFmpegDecoder,
    buffer::{FrameBuffer, FrameBufferConfig},
    traits::{VideoDecoder, PixelFormat},
    error::Result,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, interval};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎬 Rust Video Engine Demo");
    println!("========================\n");

    // Configure frame buffer for 1080p video
    let buffer_config = FrameBufferConfig {
        ring_buffer_capacity: 120,    // 4 seconds at 30fps
        cache_capacity: 60,           // 2 seconds of random access
        prefetch_threshold: 30,       // Start prefetching when 30 frames remain
        memory_pool_size: 10 << 20,   // 10MB pool
    };

    // Create frame buffer
    let buffer = Arc::new(FrameBuffer::new(buffer_config));
    
    // Create FFmpeg decoder
    let decoder = FFmpegDecoder::new();
    
    // Example: Decode a video file (would need actual file path)
    if let Ok(mut decoder) = decoder {
        println!("✅ FFmpeg decoder initialized");
        
        // In a real application, you would:
        // 1. Open a video file: decoder.open_file("video.mp4")?;
        // 2. Get video information
        // 3. Start decoding frames
        // 4. Feed frames to the buffer
        // 5. Consume frames from the buffer for playback
        
        // Example playback loop (simplified)
        println!("\n📊 Simulating video playback...");
        let mut interval = interval(Duration::from_millis(33)); // ~30fps
        
        for frame_num in 0..90 {
            interval.tick().await;
            
            // In real implementation:
            // - Decode frame from video
            // - Add to buffer
            // - Retrieve from buffer for display
            
            if frame_num % 30 == 0 {
                let metrics = buffer.get_metrics().await;
                println!(
                    "Frame {}: Cache hit rate: {:.1}%, Prefetch efficiency: {:.1}%",
                    frame_num,
                    metrics.cache_hit_rate * 100.0,
                    metrics.prefetch_efficiency * 100.0
                );
            }
        }
        
        println!("\n✅ Playback simulation complete");
        
        // Final metrics
        let final_metrics = buffer.get_metrics().await;
        println!("\n📊 Final Performance Metrics:");
        println!("  - Total frames processed: {}", final_metrics.total_frames_requested);
        println!("  - Cache hits: {}", final_metrics.cache_hits);
        println!("  - Cache misses: {}", final_metrics.cache_misses);
        println!("  - Cache hit rate: {:.1}%", final_metrics.cache_hit_rate * 100.0);
        println!("  - Prefetch efficiency: {:.1}%", final_metrics.prefetch_efficiency * 100.0);
        println!("  - Memory used: {:.1} MB", final_metrics.memory_used as f64 / 1_048_576.0);
    }
    
    println!("\n🎬 Demo complete!");
    Ok(())
}