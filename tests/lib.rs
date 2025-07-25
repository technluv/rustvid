//! Comprehensive Test Suite for Rust Video Editor
//! 
//! This test suite covers:
//! - Unit tests for all modules
//! - Integration tests for system components
//! - Cross-platform compatibility tests
//! - Performance benchmarks
//! - Accessibility compliance
//! - Memory leak detection
//! - Stress testing

#![cfg(test)]

pub mod helpers;
pub mod fixtures;

use std::sync::Once;
use std::path::PathBuf;
use once_cell::sync::Lazy;
use tempfile::TempDir;
use log::{info, debug};
use env_logger;

// Global test initialization
static INIT: Once = Once::new();

// Shared test resources
pub static TEST_TEMP_DIR: Lazy<TempDir> = Lazy::new(|| {
    TempDir::new().expect("Failed to create temp directory")
});

pub static TEST_VIDEO_DIR: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("videos")
});

pub static TEST_IMAGE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("images")
});

pub static TEST_AUDIO_DIR: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("audio")
});

/// Initialize test environment
pub fn init_tests() {
    INIT.call_once(|| {
        // Initialize logger
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
        
        info!("Initializing test environment");
        
        // Create test directories
        std::fs::create_dir_all(&*TEST_VIDEO_DIR).ok();
        std::fs::create_dir_all(&*TEST_IMAGE_DIR).ok();
        std::fs::create_dir_all(&*TEST_AUDIO_DIR).ok();
        
        // Generate test fixtures if they don't exist
        fixtures::generate_test_fixtures();
        
        debug!("Test environment initialized");
    });
}

/// Test result reporting
#[derive(Debug, Clone)]
pub struct TestReport {
    pub name: String,
    pub category: TestCategory,
    pub platform: Platform,
    pub result: TestResult,
    pub duration: std::time::Duration,
    pub memory_usage: Option<MemoryUsage>,
    pub performance_metrics: Option<PerformanceMetrics>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestCategory {
    Unit,
    Integration,
    Platform,
    Performance,
    Accessibility,
    E2E,
    Smoke,
    Stress,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Web,
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestResult {
    Passed,
    Failed(String),
    Skipped(String),
    Timeout,
}

#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub peak_rss: u64,
    pub average_rss: u64,
    pub leak_detected: bool,
    pub allocations: u64,
    pub deallocations: u64,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub fps: f64,
    pub frame_time_avg: std::time::Duration,
    pub frame_time_p99: std::time::Duration,
    pub cpu_usage: f32,
    pub gpu_usage: Option<f32>,
}

/// Test report collector
pub struct TestReporter {
    reports: Vec<TestReport>,
}

impl TestReporter {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
        }
    }
    
    pub fn add_report(&mut self, report: TestReport) {
        self.reports.push(report);
    }
    
    pub fn generate_html_report(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;
        
        let mut file = File::create(output_path)?;
        
        writeln!(file, "<!DOCTYPE html>")?;
        writeln!(file, "<html>")?;
        writeln!(file, "<head>")?;
        writeln!(file, "    <title>Rust Video Editor Test Report</title>")?;
        writeln!(file, "    <style>")?;
        writeln!(file, "        body {{ font-family: Arial, sans-serif; margin: 20px; }}")?;
        writeln!(file, "        .passed {{ color: green; }}")?;
        writeln!(file, "        .failed {{ color: red; }}")?;
        writeln!(file, "        .skipped {{ color: orange; }}")?;
        writeln!(file, "        table {{ border-collapse: collapse; width: 100%; }}")?;
        writeln!(file, "        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}")?;
        writeln!(file, "        th {{ background-color: #f2f2f2; }}")?;
        writeln!(file, "        .summary {{ margin: 20px 0; padding: 20px; background-color: #f9f9f9; }}")?;
        writeln!(file, "    </style>")?;
        writeln!(file, "</head>")?;
        writeln!(file, "<body>")?;
        writeln!(file, "    <h1>Rust Video Editor Test Report</h1>")?;
        writeln!(file, "    <p>Generated: {}</p>", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
        
        // Summary
        let total = self.reports.len();
        let passed = self.reports.iter().filter(|r| matches!(r.result, TestResult::Passed)).count();
        let failed = self.reports.iter().filter(|r| matches!(r.result, TestResult::Failed(_))).count();
        let skipped = self.reports.iter().filter(|r| matches!(r.result, TestResult::Skipped(_))).count();
        
        writeln!(file, "    <div class='summary'>")?;
        writeln!(file, "        <h2>Summary</h2>")?;
        writeln!(file, "        <p>Total Tests: {}</p>", total)?;
        writeln!(file, "        <p class='passed'>Passed: {}</p>", passed)?;
        writeln!(file, "        <p class='failed'>Failed: {}</p>", failed)?;
        writeln!(file, "        <p class='skipped'>Skipped: {}</p>", skipped)?;
        writeln!(file, "        <p>Pass Rate: {:.2}%</p>", (passed as f64 / total as f64) * 100.0)?;
        writeln!(file, "    </div>")?;
        
        // Detailed results
        writeln!(file, "    <h2>Detailed Results</h2>")?;
        writeln!(file, "    <table>")?;
        writeln!(file, "        <tr>")?;
        writeln!(file, "            <th>Test Name</th>")?;
        writeln!(file, "            <th>Category</th>")?;
        writeln!(file, "            <th>Platform</th>")?;
        writeln!(file, "            <th>Result</th>")?;
        writeln!(file, "            <th>Duration</th>")?;
        writeln!(file, "            <th>Memory Peak</th>")?;
        writeln!(file, "            <th>Performance</th>")?;
        writeln!(file, "        </tr>")?;
        
        for report in &self.reports {
            let result_class = match &report.result {
                TestResult::Passed => "passed",
                TestResult::Failed(_) => "failed",
                TestResult::Skipped(_) => "skipped",
                TestResult::Timeout => "failed",
            };
            
            let result_text = match &report.result {
                TestResult::Passed => "PASSED".to_string(),
                TestResult::Failed(msg) => format!("FAILED: {}", msg),
                TestResult::Skipped(msg) => format!("SKIPPED: {}", msg),
                TestResult::Timeout => "TIMEOUT".to_string(),
            };
            
            writeln!(file, "        <tr>")?;
            writeln!(file, "            <td>{}</td>", report.name)?;
            writeln!(file, "            <td>{:?}</td>", report.category)?;
            writeln!(file, "            <td>{:?}</td>", report.platform)?;
            writeln!(file, "            <td class='{}'>{}</td>", result_class, result_text)?;
            writeln!(file, "            <td>{:.2}s</td>", report.duration.as_secs_f64())?;
            
            if let Some(mem) = &report.memory_usage {
                writeln!(file, "            <td>{:.2} MB</td>", mem.peak_rss as f64 / 1024.0 / 1024.0)?;
            } else {
                writeln!(file, "            <td>N/A</td>")?;
            }
            
            if let Some(perf) = &report.performance_metrics {
                writeln!(file, "            <td>{:.2} FPS</td>", perf.fps)?;
            } else {
                writeln!(file, "            <td>N/A</td>")?;
            }
            
            writeln!(file, "        </tr>")?;
        }
        
        writeln!(file, "    </table>")?;
        writeln!(file, "</body>")?;
        writeln!(file, "</html>")?;
        
        Ok(())
    }
    
    pub fn generate_json_report(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        
        let file = File::create(output_path)?;
        serde_json::to_writer_pretty(file, &self.reports)?;
        
        Ok(())
    }
}

/// Platform detection
pub fn current_platform() -> Platform {
    if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::MacOS
    } else if cfg!(target_os = "linux") {
        Platform::Linux
    } else if cfg!(target_arch = "wasm32") {
        Platform::Web
    } else {
        Platform::All
    }
}

/// Test macros
#[macro_export]
macro_rules! test_all_platforms {
    ($name:ident, $test_fn:expr) => {
        #[test]
        fn $name() {
            $crate::init_tests();
            let platform = $crate::current_platform();
            println!("Running test on platform: {:?}", platform);
            $test_fn(platform);
        }
    };
}

#[macro_export]
macro_rules! benchmark_test {
    ($name:ident, $test_fn:expr) => {
        #[test]
        fn $name() {
            $crate::init_tests();
            let start = std::time::Instant::now();
            $test_fn();
            let duration = start.elapsed();
            println!("Benchmark {} completed in {:?}", stringify!($name), duration);
        }
    };
}

#[macro_export]
macro_rules! memory_test {
    ($name:ident, $test_fn:expr) => {
        #[test]
        fn $name() {
            $crate::init_tests();
            
            // Take initial memory snapshot
            let initial_mem = $crate::helpers::memory::get_current_memory_usage();
            
            $test_fn();
            
            // Take final memory snapshot
            let final_mem = $crate::helpers::memory::get_current_memory_usage();
            
            // Check for leaks
            let leak = final_mem.saturating_sub(initial_mem);
            if leak > 10 * 1024 * 1024 { // 10MB threshold
                panic!("Memory leak detected: {} bytes", leak);
            }
        }
    };
}