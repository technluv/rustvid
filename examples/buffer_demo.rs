//! Example demonstrating the frame buffer system integration

use tokio::sync::mpsc;
use video_editor_core::buffer::{Frame, FrameBuffer, FrameBufferConfig, PixelFormat};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Frame Buffer System Demo");
    println!("========================\n");
    
    // Create buffer configuration
    let config = FrameBufferConfig {
        ring_buffer_size: 30,
        cache_size: 100,
        prefetch_count: 10,
        memory_pool_size: 200 * 1024 * 1024, // 200MB
        channel_capacity: 50,
    };
    
    // Create channels for frame communication
    let (decoder_tx, decoder_rx) = mpsc::channel(50);
    
    // Create frame buffer
    let (buffer, buffer_tx) = FrameBuffer::new(config, decoder_rx);
    
    // Simulate decoder task
    let decoder_task = tokio::spawn(async move {
        println!("Starting simulated decoder...");
        
        for i in 0..100 {
            // Simulate frame decoding
            let frame = Frame {
                frame_number: i,
                pts: i as i64 * 33333, // ~30fps
                data: Arc::new(vec![0u8; 1920 * 1080 * 3]), // Full HD RGB
                width: 1920,
                height: 1080,
                format: PixelFormat::RGB24,
            };
            
            if decoder_tx.send(frame).await.is_err() {
                break;
            }
            
            // Simulate decoding time
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        println!("Decoder finished");
    });
    
    // Buffer processor task
    let buffer_processor = tokio::spawn(async move {
        // Forward frames from decoder to buffer
        while let Ok(frame) = buffer_tx.try_recv() {
            // Frame is automatically processed by the buffer
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    });
    
    // Wait for some frames to be buffered
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Demonstrate sequential access
    println!("\nSequential Access Test:");
    println!("-----------------------");
    let start = Instant::now();
    
    for i in 0..30 {
        if let Some(frame) = buffer.get_frame(i).await {
            println!("  Frame {}: {}x{}, PTS: {} µs", 
                frame.frame_number, frame.width, frame.height, frame.pts);
        }
    }
    
    let sequential_time = start.elapsed();
    println!("Sequential access time: {:?}", sequential_time);
    
    // Demonstrate random access
    println!("\nRandom Access Test:");
    println!("-------------------");
    let start = Instant::now();
    let random_frames = vec![5, 25, 10, 50, 15, 35, 20, 45];
    
    for &frame_num in &random_frames {
        if let Some(frame) = buffer.get_frame(frame_num).await {
            println!("  Frame {}: cached", frame.frame_number);
        } else {
            println!("  Frame {}: not yet available", frame_num);
        }
    }
    
    let random_time = start.elapsed();
    println!("Random access time: {:?}", random_time);
    
    // Demonstrate range access
    println!("\nRange Access Test:");
    println!("------------------");
    let start = Instant::now();
    
    let frames = buffer.get_frame_range(10, 20).await;
    println!("Retrieved {} frames in range 10-20", frames.len());
    
    let range_time = start.elapsed();
    println!("Range access time: {:?}", range_time);
    
    // Show metrics
    println!("\nBuffer Metrics:");
    println!("---------------");
    let metrics = buffer.get_metrics().await;
    println!("  Frames processed: {}", metrics.frames_processed);
    println!("  Cache hits: {} ({:.1}%)", 
        metrics.cache_hits, 
        (metrics.cache_hits as f64 / metrics.frames_processed as f64) * 100.0);
    println!("  Cache misses: {}", metrics.cache_misses);
    println!("  Ring buffer hits: {} ({:.1}%)", 
        metrics.ring_buffer_hits,
        (metrics.ring_buffer_hits as f64 / metrics.frames_processed as f64) * 100.0);
    println!("  Ring buffer misses: {}", metrics.ring_buffer_misses);
    println!("  Prefetch requests: {}", metrics.prefetch_requests);
    println!("  Pool allocations: {}", metrics.pool_allocations);
    println!("  Direct allocations: {}", metrics.direct_allocations);
    
    // Memory pool demonstration
    println!("\nMemory Pool Test:");
    println!("-----------------");
    
    // Allocate and deallocate buffers
    let mut buffers = Vec::new();
    for i in 0..5 {
        let size = (i + 1) * 1024 * 1024; // 1MB, 2MB, 3MB, etc.
        let buf = buffer.allocate_frame_data(size).await;
        println!("  Allocated {} MB buffer", (i + 1));
        buffers.push(buf);
    }
    
    // Return buffers to pool
    for (i, buf) in buffers.into_iter().enumerate() {
        buffer.deallocate_frame_data(buf).await;
        println!("  Deallocated {} MB buffer", (i + 1));
    }
    
    // Allocate again to show pool reuse
    let reused = buffer.allocate_frame_data(2 * 1024 * 1024).await;
    println!("  Reallocated 2 MB buffer (from pool)");
    buffer.deallocate_frame_data(reused).await;
    
    // Clean up
    decoder_task.abort();
    buffer_processor.abort();
    
    println!("\nDemo completed!");
    
    Ok(())
}