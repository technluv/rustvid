//! Test Helper Utilities

pub mod memory;
pub mod performance;
pub mod fixtures;
pub mod platform;
pub mod accessibility;

use std::path::{Path, PathBuf};
use std::time::Duration;
use tempfile::TempDir;

/// Create a temporary directory for test files
pub fn create_test_dir(prefix: &str) -> TempDir {
    tempfile::Builder::new()
        .prefix(prefix)
        .tempdir()
        .expect("Failed to create temp directory")
}

/// Get path to test asset
pub fn test_asset_path(category: &str, filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(category)
        .join(filename)
}

/// Create test video file using FFmpeg
pub async fn create_test_video(
    path: &Path,
    width: u32,
    height: u32,
    duration_secs: u32,
    fps: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::process::Command;
    
    let status = Command::new("ffmpeg")
        .args(&[
            "-f", "lavfi",
            "-i", &format!("testsrc=duration={}:size={}x{}:rate={}", duration_secs, width, height, fps),
            "-c:v", "libx264",
            "-preset", "ultrafast",
            "-pix_fmt", "yuv420p",
            "-y",
            path.to_str().unwrap(),
        ])
        .status()
        .await?;
    
    if !status.success() {
        return Err("Failed to create test video".into());
    }
    
    Ok(())
}

/// Create test image file
pub fn create_test_image(
    path: &Path,
    width: u32,
    height: u32,
    format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use image::{RgbImage, ImageBuffer};
    
    let img: RgbImage = ImageBuffer::from_fn(width, height, |x, y| {
        // Create a gradient pattern
        let r = (x * 255 / width) as u8;
        let g = (y * 255 / height) as u8;
        let b = ((x + y) * 255 / (width + height)) as u8;
        image::Rgb([r, g, b])
    });
    
    img.save_with_format(path, format.parse()?)?;
    Ok(())
}

/// Create test audio file using FFmpeg
pub async fn create_test_audio(
    path: &Path,
    duration_secs: u32,
    sample_rate: u32,
    channels: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::process::Command;
    
    let status = Command::new("ffmpeg")
        .args(&[
            "-f", "lavfi",
            "-i", &format!("sine=frequency=440:duration={}:sample_rate={}:channels={}", 
                           duration_secs, sample_rate, channels),
            "-codec:a", "aac",
            "-y",
            path.to_str().unwrap(),
        ])
        .status()
        .await?;
    
    if !status.success() {
        return Err("Failed to create test audio".into());
    }
    
    Ok(())
}

/// Wait for condition with timeout
pub async fn wait_for_condition<F>(
    condition: F,
    timeout: Duration,
    poll_interval: Duration,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn() -> bool,
{
    let start = std::time::Instant::now();
    
    while !condition() {
        if start.elapsed() > timeout {
            return Err("Timeout waiting for condition".into());
        }
        tokio::time::sleep(poll_interval).await;
    }
    
    Ok(())
}

/// Retry operation with exponential backoff
pub async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_retries: u32,
    initial_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Display,
{
    let mut delay = initial_delay;
    
    for attempt in 0..max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == max_retries - 1 {
                    return Err(e);
                }
                
                log::warn!("Attempt {} failed: {}. Retrying in {:?}...", attempt + 1, e, delay);
                tokio::time::sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
        }
    }
    
    unreachable!()
}

/// Compare two images pixel by pixel
pub fn compare_images(
    img1: &Path,
    img2: &Path,
    tolerance: f32,
) -> Result<bool, Box<dyn std::error::Error>> {
    use image::GenericImageView;
    
    let img1 = image::open(img1)?;
    let img2 = image::open(img2)?;
    
    if img1.dimensions() != img2.dimensions() {
        return Ok(false);
    }
    
    let (width, height) = img1.dimensions();
    let mut diff_sum = 0.0;
    let pixel_count = (width * height) as f32;
    
    for y in 0..height {
        for x in 0..width {
            let p1 = img1.get_pixel(x, y);
            let p2 = img2.get_pixel(x, y);
            
            let diff = ((p1[0] as f32 - p2[0] as f32).abs() +
                       (p1[1] as f32 - p2[1] as f32).abs() +
                       (p1[2] as f32 - p2[2] as f32).abs()) / (255.0 * 3.0);
            
            diff_sum += diff;
        }
    }
    
    let avg_diff = diff_sum / pixel_count;
    Ok(avg_diff <= tolerance)
}

/// Measure function execution time
pub fn measure_time<F, R>(name: &str, f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    
    log::info!("{} completed in {:?}", name, duration);
    (result, duration)
}

/// Assert that a value is within a range
#[macro_export]
macro_rules! assert_in_range {
    ($value:expr, $min:expr, $max:expr) => {
        assert!(
            $value >= $min && $value <= $max,
            "Value {} is not in range [{}, {}]",
            $value, $min, $max
        );
    };
}

/// Assert that two floats are approximately equal
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $tolerance:expr) => {
        assert!(
            ($a - $b).abs() <= $tolerance,
            "Values {} and {} differ by more than {}",
            $a, $b, $tolerance
        );
    };
}