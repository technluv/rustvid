//! Platform-specific testing utilities

use std::process::Command;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Web,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "macos") {
            Platform::MacOS
        } else if cfg!(target_os = "linux") {
            Platform::Linux
        } else if cfg!(target_arch = "wasm32") {
            Platform::Web
        } else {
            Platform::Linux // Default
        }
    }
    
    pub fn is_ci() -> bool {
        std::env::var("CI").is_ok()
    }
    
    pub fn has_gpu() -> bool {
        match Self::current() {
            Platform::Windows => Self::check_windows_gpu(),
            Platform::MacOS => Self::check_macos_gpu(),
            Platform::Linux => Self::check_linux_gpu(),
            Platform::Web => false,
        }
    }
    
    fn check_windows_gpu() -> bool {
        Command::new("wmic")
            .args(&["path", "win32_VideoController", "get", "name"])
            .output()
            .map(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str.contains("NVIDIA") || output_str.contains("AMD") || output_str.contains("Intel")
            })
            .unwrap_or(false)
    }
    
    fn check_macos_gpu() -> bool {
        Command::new("system_profiler")
            .args(&["SPDisplaysDataType"])
            .output()
            .map(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str.contains("Metal") || output_str.contains("GPU")
            })
            .unwrap_or(false)
    }
    
    fn check_linux_gpu() -> bool {
        Command::new("lspci")
            .output()
            .map(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str.contains("VGA") || output_str.contains("3D")
            })
            .unwrap_or(false)
    }
    
    pub fn has_ffmpeg() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    pub fn ffmpeg_version() -> Option<String> {
        Command::new("ffmpeg")
            .arg("-version")
            .output()
            .ok()
            .and_then(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str.lines().next().map(|s| s.to_string())
            })
    }
    
    pub fn available_cores() -> usize {
        num_cpus::get()
    }
    
    pub fn available_memory_gb() -> f64 {
        sysinfo::System::new_all().total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
    }
}

/// Platform-specific path handling
pub mod paths {
    use std::path::{Path, PathBuf};
    use super::Platform;
    
    pub fn executable_extension() -> &'static str {
        match Platform::current() {
            Platform::Windows => ".exe",
            _ => "",
        }
    }
    
    pub fn shared_library_extension() -> &'static str {
        match Platform::current() {
            Platform::Windows => ".dll",
            Platform::MacOS => ".dylib",
            Platform::Linux => ".so",
            Platform::Web => ".wasm",
        }
    }
    
    pub fn normalize_path(path: &Path) -> PathBuf {
        // Convert to platform-specific path separators
        let path_str = path.to_string_lossy();
        
        match Platform::current() {
            Platform::Windows => PathBuf::from(path_str.replace("/", "\\")),
            _ => PathBuf::from(path_str.replace("\\", "/")),
        }
    }
    
    pub fn temp_dir() -> PathBuf {
        std::env::temp_dir()
    }
}

/// Platform-specific process utilities
pub mod process {
    use std::process::Command;
    use super::Platform;
    
    pub fn kill_process(name: &str) -> Result<(), std::io::Error> {
        match Platform::current() {
            Platform::Windows => {
                Command::new("taskkill")
                    .args(&["/F", "/IM", &format!("{}.exe", name)])
                    .output()?;
            }
            Platform::MacOS | Platform::Linux => {
                Command::new("pkill")
                    .arg(name)
                    .output()?;
            }
            Platform::Web => {
                // Not applicable for web
            }
        }
        Ok(())
    }
    
    pub fn is_process_running(name: &str) -> bool {
        match Platform::current() {
            Platform::Windows => {
                Command::new("tasklist")
                    .output()
                    .map(|output| {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        output_str.contains(&format!("{}.exe", name))
                    })
                    .unwrap_or(false)
            }
            Platform::MacOS | Platform::Linux => {
                Command::new("pgrep")
                    .arg(name)
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            }
            Platform::Web => false,
        }
    }
}

/// Platform-specific GUI testing
pub mod gui {
    use super::Platform;
    
    pub fn is_headless() -> bool {
        match Platform::current() {
            Platform::Linux => std::env::var("DISPLAY").is_err(),
            Platform::Web => true,
            _ => false,
        }
    }
    
    pub fn screen_resolution() -> Option<(u32, u32)> {
        if is_headless() {
            return None;
        }
        
        match Platform::current() {
            Platform::Windows => {
                // Would use Windows API
                Some((1920, 1080)) // Default
            }
            Platform::MacOS => {
                // Would use Core Graphics
                Some((2560, 1600)) // Default
            }
            Platform::Linux => {
                // Would use X11/Wayland
                Some((1920, 1080)) // Default
            }
            Platform::Web => None,
        }
    }
}

/// Skip test if condition not met
#[macro_export]
macro_rules! skip_if {
    ($condition:expr, $reason:expr) => {
        if $condition {
            eprintln!("Skipping test: {}", $reason);
            return;
        }
    };
}

/// Run test only on specific platforms
#[macro_export]
macro_rules! test_on_platform {
    ($platform:expr, $test_body:block) => {
        if $crate::helpers::platform::Platform::current() == $platform {
            $test_body
        } else {
            eprintln!("Skipping test: not on platform {:?}", $platform);
        }
    };
}

/// Platform capability requirements
pub struct Requirements {
    pub min_memory_gb: f64,
    pub min_cores: usize,
    pub needs_gpu: bool,
    pub needs_ffmpeg: bool,
    pub platforms: Vec<Platform>,
}

impl Requirements {
    pub fn check(&self) -> Result<(), String> {
        // Check platform
        if !self.platforms.is_empty() && !self.platforms.contains(&Platform::current()) {
            return Err(format!("Platform {:?} not supported", Platform::current()));
        }
        
        // Check memory
        if Platform::available_memory_gb() < self.min_memory_gb {
            return Err(format!(
                "Insufficient memory: {:.1} GB available, {:.1} GB required",
                Platform::available_memory_gb(),
                self.min_memory_gb
            ));
        }
        
        // Check cores
        if Platform::available_cores() < self.min_cores {
            return Err(format!(
                "Insufficient CPU cores: {} available, {} required",
                Platform::available_cores(),
                self.min_cores
            ));
        }
        
        // Check GPU
        if self.needs_gpu && !Platform::has_gpu() {
            return Err("GPU required but not available".to_string());
        }
        
        // Check FFmpeg
        if self.needs_ffmpeg && !Platform::has_ffmpeg() {
            return Err("FFmpeg required but not installed".to_string());
        }
        
        Ok(())
    }
}