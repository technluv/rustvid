# Frame Buffer System

The frame buffer system provides high-performance frame caching and memory management for video processing.

## Architecture

### Components

1. **FrameBuffer** (`mod.rs`)
   - Main orchestrator for frame buffering
   - Manages ring buffer for sequential access
   - Coordinates with LRU cache for random access
   - Handles async prefetching
   - Collects performance metrics

2. **FrameCache** (`cache.rs`)
   - LRU (Least Recently Used) cache implementation
   - Provides O(1) frame lookups
   - Automatically evicts old frames when capacity is reached
   - Thread-safe with async support

3. **MemoryPool** (`pool.rs`)
   - Pre-allocates memory to reduce allocation overhead
   - Organizes buffers into size buckets
   - Reuses deallocated buffers
   - Provides allocation statistics

## Integration with Decoder

### Channel-based Communication

The buffer system integrates with the decoder through Tokio channels:

```rust
// Decoder sends frames to buffer
let (frame_sender, frame_receiver) = mpsc::channel(50);
let (buffer, buffer_tx) = FrameBuffer::new(config, frame_receiver);

// Decoder sends decoded frames
frame_sender.send(decoded_frame).await?;
```

### Async Prefetching

When a frame is requested but not found, the buffer system can notify the decoder to prioritize decoding:

```rust
// Buffer requests prefetch
if let Some(frame) = buffer.get_frame(frame_num).await {
    // Frame available
} else {
    // Frame not available - prefetch triggered
}
```

### Memory Management

The decoder can use the buffer's memory pool to reduce allocations:

```rust
// Allocate frame data from pool
let frame_data = buffer.allocate_frame_data(size).await;

// Fill with decoded data
decoder.decode_into(&mut frame_data)?;

// Create frame
let frame = Frame {
    frame_number,
    data: Arc::new(frame_data),
    // ...
};

// Send to buffer
frame_sender.send(frame).await?;
```

## Performance Considerations

### Access Patterns

1. **Sequential Access**: Optimized through ring buffer
   - Best for playback scenarios
   - Minimal cache misses
   - Predictable memory usage

2. **Random Access**: Handled by LRU cache
   - Good for scrubbing/seeking
   - May trigger prefetch requests
   - Higher memory usage

3. **Range Access**: Batch operations
   - Efficient for effects processing
   - Triggers prefetch for subsequent frames
   - Can pre-load entire sequences

### Memory Optimization

- Use memory pool for all frame allocations
- Configure pool size based on:
  - Video resolution
  - Frame rate
  - Number of concurrent streams
  - Available system memory

### Metrics Monitoring

Monitor these key metrics:
- Cache hit rate (target: >80% for sequential, >60% for random)
- Pool allocation rate (high = good reuse)
- Prefetch efficiency (low misses = good prediction)

## Configuration Guidelines

### Small Videos (<720p)
```rust
FrameBufferConfig {
    ring_buffer_size: 60,        // 2 seconds at 30fps
    cache_size: 200,             // Extra cache
    prefetch_count: 15,          // Aggressive prefetch
    memory_pool_size: 200 * 1024 * 1024,  // 200MB
    channel_capacity: 100,
}
```

### HD Videos (1080p)
```rust
FrameBufferConfig {
    ring_buffer_size: 30,        // 1 second at 30fps
    cache_size: 100,             // Moderate cache
    prefetch_count: 10,          // Balanced prefetch
    memory_pool_size: 500 * 1024 * 1024,  // 500MB
    channel_capacity: 50,
}
```

### 4K Videos
```rust
FrameBufferConfig {
    ring_buffer_size: 15,        // 0.5 seconds at 30fps
    cache_size: 50,              // Limited cache
    prefetch_count: 5,           // Conservative prefetch
    memory_pool_size: 2 * 1024 * 1024 * 1024,  // 2GB
    channel_capacity: 25,
}
```

## Thread Safety

All components are thread-safe and designed for concurrent access:
- Multiple readers can access frames simultaneously
- Writers are serialized through channels
- Memory pool operations are atomic
- Metrics collection is lock-free where possible