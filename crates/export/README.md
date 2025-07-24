# Video Editor Export Module

This module provides comprehensive video export functionality with FFmpeg integration, including hardware encoding support, preset management, and job queuing.

## Features

### Video Encoding (VideoEncoder)
- **Supported Codecs**: H.264, H.265/HEVC, VP9, AV1
- **Hardware Encoding**: Automatic detection and support for:
  - NVIDIA NVENC
  - Intel QuickSync
  - AMD AMF
  - Apple VideoToolbox (macOS)
- **Quality Settings**: Low, Medium, High, Ultra, or Custom (0-100)
- **Configurable Parameters**: Bitrate, framerate, resolution, keyframe interval, B-frames

### Render Pipeline
- **Multi-threaded Processing**: Parallel frame rendering and effect application
- **Pipeline Stages**:
  1. Timeline rendering: Extract frames from timeline
  2. Effect processing: Apply effects using worker threads
  3. Frame encoding: Encode processed frames
- **Buffered Processing**: Configurable buffer size for smooth operation
- **Progress Tracking**: Real-time progress updates

### Export Presets
Pre-configured settings for popular platforms:

#### Video Platforms
- **YouTube 1080p**: H.264, 1920x1080, 30fps, 8Mbps
- **YouTube 4K**: H.265, 3840x2160, 30fps, 40Mbps
- **YouTube Shorts**: H.264, 1080x1920 (9:16), 30fps, 10Mbps
- **Vimeo**: H.264, 1920x1080, 30fps, 10Mbps (high quality)

#### Social Media
- **Twitter/X**: H.264, 1280x720, 30fps, 5Mbps
- **Instagram**: H.264, 1080x1080 (square), 30fps, 8Mbps
- **TikTok**: H.264, 1080x1920 (9:16), 30fps, 8Mbps

#### Professional Formats
- **ProRes 422**: Apple ProRes 422, 1920x1080, 30fps, ~147Mbps
- **ProRes 4444**: Apple ProRes 4444 with alpha, 1920x1080, 30fps, ~330Mbps
- **DNxHD**: Avid DNxHD 145, 1920x1080, 30fps, 145Mbps

#### Web & Mobile
- **Web MP4**: H.264, 1280x720, 30fps, 2.5Mbps
- **WebM**: VP9, 1280x720, 30fps, 2Mbps
- **Mobile High**: H.264, 1280x720, 30fps, 3Mbps
- **Mobile Low**: H.264, 854x480, 30fps, 1Mbps

### Job Management
- **Priority Queue**: Support for Low, Normal, High, and Critical priorities
- **Concurrent Exports**: Configurable maximum concurrent jobs
- **Job Tracking**: Status, progress, duration, and error tracking
- **Statistics**: Overall job statistics and performance metrics

## Usage

### Basic Export

```rust
use video_editor_export::{ExportEngine, ExportSettings, ExportProgress};
use std::sync::{Arc, Mutex};

// Create export settings
let settings = ExportSettings::default();

// Create export engine
let engine = ExportEngine::new(settings);

// Export timeline
let timeline = Arc::new(timeline);
let progress = Arc::new(Mutex::new(MyProgressTracker));
engine.export(timeline, progress).await?;
```

### Export with Preset

```rust
use video_editor_export::{ExportEngine, ExportPreset};

ExportEngine::export_with_preset(
    ExportPreset::YouTube1080p,
    PathBuf::from("output.mp4"),
    timeline,
    progress,
).await?;
```

### Custom Export Settings

```rust
use video_editor_export::{ExportSettingsBuilder, Quality};

let settings = ExportSettingsBuilder::new()
    .resolution(1280, 720)
    .fps(60.0)
    .bitrate(5_000_000)
    .quality(Quality::High)
    .video_codec("h265")
    .build();
```

### Job Management

```rust
use video_editor_export::{ExportJobManager, JobPriority};

// Create job manager
let manager = Arc::new(ExportJobManager::new(4)); // Max 4 concurrent jobs

// Queue export job
let job_id = manager.create_job(
    "My Export".to_string(),
    settings,
    JobPriority::Normal,
);

// Check job status
let job = manager.get_job(job_id).unwrap();
println!("Status: {:?}, Progress: {:.1}%", job.status, job.progress);

// Get statistics
let stats = manager.get_statistics();
println!("Active jobs: {}", stats.active_jobs);
```

## Architecture

### Pipeline Flow
```
Timeline → Frame Renderer → Effect Processor → Video Encoder → Output File
              ↓                    ↓                ↓
         (Parallel)           (Parallel)      (Sequential)
```

### Thread Model
- **Timeline Renderer**: Single thread extracting frames
- **Effect Processors**: Multiple worker threads (CPU count)
- **Video Encoder**: Single thread encoding frames sequentially
- **Job Manager**: Manages multiple concurrent export pipelines

## Performance Considerations

1. **Hardware Encoding**: Automatically uses hardware encoders when available for significant performance improvement
2. **Multi-threading**: Effect processing utilizes all CPU cores
3. **Buffering**: Pipeline buffering prevents stalls between stages
4. **Memory Usage**: Frame buffers are recycled to minimize allocations

## Future Enhancements

- [ ] Audio encoding support
- [ ] GPU-accelerated effects processing
- [ ] Distributed rendering support
- [ ] Additional codec support (ProRes on non-Apple platforms)
- [ ] Export queue persistence
- [ ] Network rendering support