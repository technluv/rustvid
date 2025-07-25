//! Performance testing utilities

use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

/// Performance benchmark runner
pub struct BenchmarkRunner {
    name: String,
    iterations: usize,
    warmup_iterations: usize,
    results: Vec<Duration>,
}

impl BenchmarkRunner {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            iterations: 100,
            warmup_iterations: 10,
            results: Vec::new(),
        }
    }
    
    pub fn iterations(mut self, count: usize) -> Self {
        self.iterations = count;
        self
    }
    
    pub fn warmup(mut self, count: usize) -> Self {
        self.warmup_iterations = count;
        self
    }
    
    pub fn run<F>(&mut self, mut f: F) -> BenchmarkResult
    where
        F: FnMut(),
    {
        println!("Running benchmark: {}", self.name);
        
        // Warmup
        println!("  Warming up ({} iterations)...", self.warmup_iterations);
        for _ in 0..self.warmup_iterations {
            f();
        }
        
        // Actual benchmark
        println!("  Benchmarking ({} iterations)...", self.iterations);
        self.results.clear();
        
        for i in 0..self.iterations {
            let start = Instant::now();
            f();
            let duration = start.elapsed();
            self.results.push(duration);
            
            if (i + 1) % (self.iterations / 10).max(1) == 0 {
                print!(".");
                use std::io::{self, Write};
                io::stdout().flush().ok();
            }
        }
        println!();
        
        self.analyze_results()
    }
    
    fn analyze_results(&self) -> BenchmarkResult {
        let mut sorted = self.results.clone();
        sorted.sort();
        
        let total: Duration = self.results.iter().sum();
        let mean = total / self.results.len() as u32;
        
        let median = if self.results.len() % 2 == 0 {
            let mid = self.results.len() / 2;
            (sorted[mid - 1] + sorted[mid]) / 2
        } else {
            sorted[self.results.len() / 2]
        };
        
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        
        let p95_index = (self.results.len() as f64 * 0.95) as usize;
        let p99_index = (self.results.len() as f64 * 0.99) as usize;
        let p95 = sorted[p95_index.min(sorted.len() - 1)];
        let p99 = sorted[p99_index.min(sorted.len() - 1)];
        
        // Calculate standard deviation
        let variance = self.results.iter()
            .map(|&d| {
                let diff = d.as_secs_f64() - mean.as_secs_f64();
                diff * diff
            })
            .sum::<f64>() / self.results.len() as f64;
        let std_dev = Duration::from_secs_f64(variance.sqrt());
        
        BenchmarkResult {
            name: self.name.clone(),
            iterations: self.iterations,
            mean,
            median,
            min,
            max,
            p95,
            p99,
            std_dev,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub mean: Duration,
    pub median: Duration,
    pub min: Duration,
    pub max: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub std_dev: Duration,
}

impl BenchmarkResult {
    pub fn print_summary(&self) {
        println!("\nBenchmark Results: {}", self.name);
        println!("  Iterations: {}", self.iterations);
        println!("  Mean:       {:?}", self.mean);
        println!("  Median:     {:?}", self.median);
        println!("  Min:        {:?}", self.min);
        println!("  Max:        {:?}", self.max);
        println!("  P95:        {:?}", self.p95);
        println!("  P99:        {:?}", self.p99);
        println!("  Std Dev:    {:?}", self.std_dev);
    }
    
    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{}",
            self.name,
            self.iterations,
            self.mean.as_micros(),
            self.median.as_micros(),
            self.min.as_micros(),
            self.max.as_micros(),
            self.p95.as_micros(),
            self.p99.as_micros(),
            self.std_dev.as_micros()
        )
    }
}

/// CPU usage monitor
pub struct CpuMonitor {
    start_time: Instant,
    start_cpu_time: Duration,
}

impl CpuMonitor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            start_cpu_time: get_process_cpu_time(),
        }
    }
    
    pub fn get_cpu_usage(&self) -> f32 {
        let elapsed = self.start_time.elapsed();
        let cpu_time = get_process_cpu_time() - self.start_cpu_time;
        
        if elapsed.as_secs_f64() > 0.0 {
            (cpu_time.as_secs_f64() / elapsed.as_secs_f64() * 100.0) as f32
        } else {
            0.0
        }
    }
}

fn get_process_cpu_time() -> Duration {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        
        if let Ok(stat) = fs::read_to_string("/proc/self/stat") {
            let fields: Vec<&str> = stat.split_whitespace().collect();
            if fields.len() > 14 {
                if let (Ok(utime), Ok(stime)) = (fields[13].parse::<u64>(), fields[14].parse::<u64>()) {
                    let ticks = utime + stime;
                    let ticks_per_second = 100; // Typical value, could use sysconf
                    return Duration::from_millis(ticks * 1000 / ticks_per_second);
                }
            }
        }
    }
    
    Duration::from_secs(0)
}

/// Frame rate counter
pub struct FpsCounter {
    frame_count: Arc<AtomicU64>,
    start_time: Instant,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frame_count: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
        }
    }
    
    pub fn frame(&self) {
        self.frame_count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_fps(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let frames = self.frame_count.load(Ordering::Relaxed) as f64;
        
        if elapsed > 0.0 {
            frames / elapsed
        } else {
            0.0
        }
    }
    
    pub fn reset(&mut self) {
        self.frame_count.store(0, Ordering::Relaxed);
        self.start_time = Instant::now();
    }
}

/// Throughput measurement
pub struct ThroughputMeter {
    bytes_processed: Arc<AtomicU64>,
    start_time: Instant,
}

impl ThroughputMeter {
    pub fn new() -> Self {
        Self {
            bytes_processed: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
        }
    }
    
    pub fn add_bytes(&self, bytes: u64) {
        self.bytes_processed.fetch_add(bytes, Ordering::Relaxed);
    }
    
    pub fn get_throughput_mbps(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let bytes = self.bytes_processed.load(Ordering::Relaxed) as f64;
        
        if elapsed > 0.0 {
            (bytes / elapsed) / (1024.0 * 1024.0) // MB/s
        } else {
            0.0
        }
    }
}

/// Performance profiler
pub struct Profiler {
    sections: Vec<ProfileSection>,
    current_section: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ProfileSection {
    pub name: String,
    pub start: Instant,
    pub duration: Option<Duration>,
    pub parent: Option<usize>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            current_section: None,
        }
    }
    
    pub fn start_section(&mut self, name: impl Into<String>) {
        let section = ProfileSection {
            name: name.into(),
            start: Instant::now(),
            duration: None,
            parent: self.current_section,
        };
        
        let index = self.sections.len();
        self.sections.push(section);
        self.current_section = Some(index);
    }
    
    pub fn end_section(&mut self) {
        if let Some(index) = self.current_section {
            let duration = self.sections[index].start.elapsed();
            self.sections[index].duration = Some(duration);
            self.current_section = self.sections[index].parent;
        }
    }
    
    pub fn print_report(&self) {
        println!("\nPerformance Profile:");
        for (i, section) in self.sections.iter().enumerate() {
            let indent = self.get_depth(i) * 2;
            let indent_str = " ".repeat(indent);
            
            if let Some(duration) = section.duration {
                println!("{}{}: {:?}", indent_str, section.name, duration);
            } else {
                println!("{}{}: <incomplete>", indent_str, section.name);
            }
        }
    }
    
    fn get_depth(&self, index: usize) -> usize {
        let mut depth = 0;
        let mut current = self.sections[index].parent;
        
        while let Some(parent_index) = current {
            depth += 1;
            current = self.sections[parent_index].parent;
        }
        
        depth
    }
}

/// Macro for easy benchmarking
#[macro_export]
macro_rules! benchmark {
    ($name:expr, $iterations:expr, $code:block) => {{
        let mut runner = $crate::helpers::performance::BenchmarkRunner::new($name)
            .iterations($iterations);
        
        let result = runner.run(|| $code);
        result.print_summary();
        result
    }};
}