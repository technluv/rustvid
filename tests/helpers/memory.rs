//! Memory testing utilities

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Get current memory usage in bytes
pub fn get_current_memory_usage() -> u64 {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            return kb * 1024; // Convert KB to bytes
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        if let Ok(output) = Command::new("ps")
            .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
            .output()
        {
            if let Ok(s) = String::from_utf8(output.stdout) {
                if let Ok(kb) = s.trim().parse::<u64>() {
                    return kb * 1024; // Convert KB to bytes
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
        use std::mem;
        
        unsafe {
            let mut pmc: PROCESS_MEMORY_COUNTERS = mem::zeroed();
            pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
            
            if GetProcessMemoryInfo(
                GetCurrentProcess(),
                &mut pmc as *mut _,
                pmc.cb
            ) != 0 {
                return pmc.WorkingSetSize as u64;
            }
        }
    }
    
    0 // Fallback
}

/// Memory leak detector
pub struct MemoryLeakDetector {
    initial_memory: u64,
    peak_memory: Arc<AtomicU64>,
    allocation_count: Arc<AtomicU64>,
    deallocation_count: Arc<AtomicU64>,
}

impl MemoryLeakDetector {
    pub fn new() -> Self {
        Self {
            initial_memory: get_current_memory_usage(),
            peak_memory: Arc::new(AtomicU64::new(0)),
            allocation_count: Arc::new(AtomicU64::new(0)),
            deallocation_count: Arc::new(AtomicU64::new(0)),
        }
    }
    
    pub fn start_monitoring(&self) -> MemoryMonitor {
        MemoryMonitor {
            peak_memory: Arc::clone(&self.peak_memory),
            allocation_count: Arc::clone(&self.allocation_count),
            deallocation_count: Arc::clone(&self.deallocation_count),
        }
    }
    
    pub fn check_for_leaks(&self, threshold_bytes: u64) -> Result<(), String> {
        let current_memory = get_current_memory_usage();
        let memory_increase = current_memory.saturating_sub(self.initial_memory);
        
        if memory_increase > threshold_bytes {
            let allocations = self.allocation_count.load(Ordering::Relaxed);
            let deallocations = self.deallocation_count.load(Ordering::Relaxed);
            let unfreed = allocations.saturating_sub(deallocations);
            
            return Err(format!(
                "Memory leak detected: {} bytes increase (threshold: {} bytes). \
                 Allocations: {}, Deallocations: {}, Unfreed: {}",
                memory_increase, threshold_bytes, allocations, deallocations, unfreed
            ));
        }
        
        Ok(())
    }
    
    pub fn get_peak_memory(&self) -> u64 {
        self.peak_memory.load(Ordering::Relaxed)
    }
    
    pub fn get_memory_stats(&self) -> MemoryStats {
        MemoryStats {
            initial: self.initial_memory,
            current: get_current_memory_usage(),
            peak: self.peak_memory.load(Ordering::Relaxed),
            allocations: self.allocation_count.load(Ordering::Relaxed),
            deallocations: self.deallocation_count.load(Ordering::Relaxed),
        }
    }
}

pub struct MemoryMonitor {
    peak_memory: Arc<AtomicU64>,
    allocation_count: Arc<AtomicU64>,
    deallocation_count: Arc<AtomicU64>,
}

impl MemoryMonitor {
    pub fn update(&self) {
        let current = get_current_memory_usage();
        self.peak_memory.fetch_max(current, Ordering::Relaxed);
    }
    
    pub fn record_allocation(&self) {
        self.allocation_count.fetch_add(1, Ordering::Relaxed);
        self.update();
    }
    
    pub fn record_deallocation(&self) {
        self.deallocation_count.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub initial: u64,
    pub current: u64,
    pub peak: u64,
    pub allocations: u64,
    pub deallocations: u64,
}

impl MemoryStats {
    pub fn memory_increase(&self) -> u64 {
        self.current.saturating_sub(self.initial)
    }
    
    pub fn unfreed_allocations(&self) -> u64 {
        self.allocations.saturating_sub(self.deallocations)
    }
    
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Custom allocator for tracking allocations
#[cfg(feature = "memory-tracking")]
pub mod tracking_allocator {
    use std::alloc::{GlobalAlloc, Layout, System};
    use super::*;
    
    pub struct TrackingAllocator {
        monitor: Option<MemoryMonitor>,
    }
    
    impl TrackingAllocator {
        pub const fn new() -> Self {
            Self { monitor: None }
        }
        
        pub fn set_monitor(&mut self, monitor: MemoryMonitor) {
            self.monitor = Some(monitor);
        }
    }
    
    unsafe impl GlobalAlloc for TrackingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ptr = System.alloc(layout);
            if !ptr.is_null() {
                if let Some(monitor) = &self.monitor {
                    monitor.record_allocation();
                }
            }
            ptr
        }
        
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            if let Some(monitor) = &self.monitor {
                monitor.record_deallocation();
            }
            System.dealloc(ptr, layout);
        }
    }
}

/// Macro for memory leak tests
#[macro_export]
macro_rules! test_no_memory_leak {
    ($test_name:ident, $test_body:block) => {
        #[test]
        fn $test_name() {
            let detector = $crate::helpers::memory::MemoryLeakDetector::new();
            let monitor = detector.start_monitoring();
            
            // Run test
            $test_body
            
            // Force cleanup
            drop(monitor);
            
            // Allow some time for deferred cleanup
            std::thread::sleep(std::time::Duration::from_millis(100));
            
            // Check for leaks (10MB threshold)
            if let Err(e) = detector.check_for_leaks(10 * 1024 * 1024) {
                panic!("{}", e);
            }
        }
    };
}