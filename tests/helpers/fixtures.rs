//! Test fixtures generation and management

use std::path::{Path, PathBuf};
use std::fs;
use log::info;

/// Generate all test fixtures
pub fn generate_test_fixtures() {
    info!("Generating test fixtures...");
    
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures");
    
    // Create directories
    fs::create_dir_all(fixtures_dir.join("videos")).ok();
    fs::create_dir_all(fixtures_dir.join("images")).ok();
    fs::create_dir_all(fixtures_dir.join("audio")).ok();
    
    // Generate fixture metadata
    generate_fixture_metadata(&fixtures_dir);
    
    info!("Test fixtures ready");
}

fn generate_fixture_metadata(fixtures_dir: &Path) {
    let metadata = r#"{
    "videos": [
        {
            "name": "test_720p.mp4",
            "width": 1280,
            "height": 720,
            "fps": 30,
            "duration": 5,
            "codec": "h264"
        },
        {
            "name": "test_1080p.mp4",
            "width": 1920,
            "height": 1080,
            "fps": 60,
            "duration": 3,
            "codec": "h264"
        },
        {
            "name": "test_4k.mp4",
            "width": 3840,
            "height": 2160,
            "fps": 30,
            "duration": 2,
            "codec": "h265"
        }
    ],
    "images": [
        {
            "name": "test_hd.jpg",
            "width": 1920,
            "height": 1080,
            "format": "jpeg"
        },
        {
            "name": "test_4k.png",
            "width": 3840,
            "height": 2160,
            "format": "png"
        },
        {
            "name": "test_square.webp",
            "width": 1024,
            "height": 1024,
            "format": "webp"
        }
    ],
    "audio": [
        {
            "name": "test_stereo.aac",
            "duration": 10,
            "sample_rate": 44100,
            "channels": 2,
            "codec": "aac"
        },
        {
            "name": "test_mono.mp3",
            "duration": 5,
            "sample_rate": 48000,
            "channels": 1,
            "codec": "mp3"
        }
    ]
}"#;
    
    fs::write(fixtures_dir.join("metadata.json"), metadata).ok();
}

/// Test data patterns for video generation
pub mod patterns {
    use image::{RgbImage, Rgb};
    
    /// Generate color bars pattern
    pub fn color_bars(width: u32, height: u32) -> RgbImage {
        let colors = [
            Rgb([255, 255, 255]), // White
            Rgb([255, 255, 0]),   // Yellow
            Rgb([0, 255, 255]),   // Cyan
            Rgb([0, 255, 0]),     // Green
            Rgb([255, 0, 255]),   // Magenta
            Rgb([255, 0, 0]),     // Red
            Rgb([0, 0, 255]),     // Blue
            Rgb([0, 0, 0]),       // Black
        ];
        
        let mut img = RgbImage::new(width, height);
        let bar_width = width / colors.len() as u32;
        
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let bar_index = (x / bar_width).min(colors.len() as u32 - 1) as usize;
            *pixel = colors[bar_index];
        }
        
        img
    }
    
    /// Generate gradient pattern
    pub fn gradient(width: u32, height: u32) -> RgbImage {
        RgbImage::from_fn(width, height, |x, y| {
            let r = (x * 255 / width) as u8;
            let g = (y * 255 / height) as u8;
            let b = ((x + y) * 255 / (width + height)) as u8;
            Rgb([r, g, b])
        })
    }
    
    /// Generate checkerboard pattern
    pub fn checkerboard(width: u32, height: u32, square_size: u32) -> RgbImage {
        RgbImage::from_fn(width, height, |x, y| {
            let is_white = ((x / square_size) + (y / square_size)) % 2 == 0;
            if is_white {
                Rgb([255, 255, 255])
            } else {
                Rgb([0, 0, 0])
            }
        })
    }
    
    /// Generate noise pattern
    pub fn noise(width: u32, height: u32) -> RgbImage {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        RgbImage::from_fn(width, height, |_, _| {
            let val = rng.gen::<u8>();
            Rgb([val, val, val])
        })
    }
}

/// Fixture file manager
pub struct FixtureManager {
    base_dir: PathBuf,
}

impl FixtureManager {
    pub fn new() -> Self {
        Self {
            base_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("fixtures"),
        }
    }
    
    pub fn video_path(&self, name: &str) -> PathBuf {
        self.base_dir.join("videos").join(name)
    }
    
    pub fn image_path(&self, name: &str) -> PathBuf {
        self.base_dir.join("images").join(name)
    }
    
    pub fn audio_path(&self, name: &str) -> PathBuf {
        self.base_dir.join("audio").join(name)
    }
    
    pub fn ensure_video(&self, name: &str, width: u32, height: u32, duration: u32) -> PathBuf {
        let path = self.video_path(name);
        
        if !path.exists() {
            // Create on demand using FFmpeg
            std::process::Command::new("ffmpeg")
                .args(&[
                    "-f", "lavfi",
                    "-i", &format!("testsrc=duration={}:size={}x{}:rate=30", duration, width, height),
                    "-c:v", "libx264",
                    "-preset", "ultrafast",
                    "-pix_fmt", "yuv420p",
                    "-y",
                    path.to_str().unwrap(),
                ])
                .output()
                .ok();
        }
        
        path
    }
    
    pub fn ensure_image(&self, name: &str, width: u32, height: u32) -> PathBuf {
        let path = self.image_path(name);
        
        if !path.exists() {
            let img = patterns::color_bars(width, height);
            img.save(&path).ok();
        }
        
        path
    }
    
    pub fn ensure_audio(&self, name: &str, duration: u32) -> PathBuf {
        let path = self.audio_path(name);
        
        if !path.exists() {
            std::process::Command::new("ffmpeg")
                .args(&[
                    "-f", "lavfi",
                    "-i", &format!("sine=frequency=440:duration={}:sample_rate=44100", duration),
                    "-codec:a", "aac",
                    "-y",
                    path.to_str().unwrap(),
                ])
                .output()
                .ok();
        }
        
        path
    }
    
    pub fn cleanup(&self) {
        // Clean up generated fixtures after tests
        fs::remove_dir_all(&self.base_dir).ok();
    }
}

/// Corrupt file generator for error testing
pub mod corrupt {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    
    /// Create a corrupt video file
    pub fn create_corrupt_video(path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        // Write invalid MP4 header
        file.write_all(b"CORRUPT_VIDEO_FILE")?;
        file.write_all(&vec![0xFF; 1024])?; // Random data
        Ok(())
    }
    
    /// Create a corrupt image file
    pub fn create_corrupt_image(path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        // Write invalid JPEG header
        file.write_all(b"\xFF\xD8\xFF\xE0")?; // Start of JPEG
        file.write_all(b"CORRUPT")?; // Invalid JFIF marker
        file.write_all(&vec![0x00; 512])?; // Zeros
        Ok(())
    }
    
    /// Create a file with specific size
    pub fn create_large_file(path: &Path, size_mb: u64) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let chunk = vec![0u8; 1024 * 1024]; // 1MB chunk
        
        for _ in 0..size_mb {
            file.write_all(&chunk)?;
        }
        
        Ok(())
    }
}