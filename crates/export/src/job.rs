//! Export job management and queue system

use crate::{ExportError, Result, ExportSettings, ExportProgress};
use dashmap::DashMap;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use uuid::Uuid;
use tracing::{info, debug, warn, error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Export job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Queued,
    Preparing,
    Rendering,
    Encoding,
    Finalizing,
    Completed,
    Failed,
    Cancelled,
}

impl JobStatus {
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            JobStatus::Preparing | JobStatus::Rendering | JobStatus::Encoding | JobStatus::Finalizing
        )
    }
    
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            JobStatus::Completed | JobStatus::Failed | JobStatus::Cancelled
        )
    }
}

/// Export job priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JobPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Export job information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportJob {
    pub id: Uuid,
    pub name: String,
    pub settings: ExportSettings,
    pub status: JobStatus,
    pub priority: JobPriority,
    pub progress: f32,
    pub progress_message: String,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub estimated_time_remaining: Option<Duration>,
    pub output_size: Option<u64>,
}

impl ExportJob {
    /// Create a new export job
    pub fn new(name: String, settings: ExportSettings, priority: JobPriority) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            settings,
            status: JobStatus::Queued,
            priority,
            progress: 0.0,
            progress_message: "Queued".to_string(),
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
            estimated_time_remaining: None,
            output_size: None,
        }
    }
    
    /// Get the duration of the job
    pub fn duration(&self) -> Option<Duration> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some((end - start).to_std().ok()?),
            (Some(start), None) if self.status.is_active() => {
                Some((Utc::now() - start).to_std().ok()?)
            }
            _ => None,
        }
    }
    
    /// Update job status
    pub fn update_status(&mut self, status: JobStatus) {
        self.status = status;
        
        match status {
            JobStatus::Preparing | JobStatus::Rendering => {
                if self.started_at.is_none() {
                    self.started_at = Some(Utc::now());
                }
            }
            JobStatus::Completed | JobStatus::Failed | JobStatus::Cancelled => {
                self.completed_at = Some(Utc::now());
            }
            _ => {}
        }
    }
    
    /// Update progress
    pub fn update_progress(&mut self, progress: f32, message: String) {
        self.progress = progress.clamp(0.0, 100.0);
        self.progress_message = message;
        
        // Estimate time remaining based on progress
        if let Some(duration) = self.duration() {
            if self.progress > 0.0 && self.progress < 100.0 {
                let elapsed_secs = duration.as_secs_f32();
                let total_estimated = elapsed_secs / (self.progress / 100.0);
                let remaining_secs = total_estimated - elapsed_secs;
                self.estimated_time_remaining = Some(Duration::from_secs_f32(remaining_secs));
            }
        }
    }
}

/// Progress tracker for export jobs
struct JobProgressTracker {
    job_id: Uuid,
    manager: Arc<ExportJobManager>,
    start_time: Instant,
}

impl ExportProgress for JobProgressTracker {
    fn on_progress(&mut self, percent: f32, message: &str) {
        self.manager.update_job_progress(self.job_id, percent, message.to_string());
    }
    
    fn on_complete(&mut self) {
        self.manager.complete_job(self.job_id);
    }
    
    fn on_error(&mut self, error: &ExportError) {
        self.manager.fail_job(self.job_id, error.to_string());
    }
}

/// Export job manager
pub struct ExportJobManager {
    jobs: Arc<DashMap<Uuid, ExportJob>>,
    job_queue: Arc<Mutex<Vec<Uuid>>>,
    active_jobs: Arc<Mutex<Vec<Uuid>>>,
    max_concurrent_jobs: usize,
}

impl ExportJobManager {
    /// Create a new job manager
    pub fn new(max_concurrent_jobs: usize) -> Self {
        Self {
            jobs: Arc::new(DashMap::new()),
            job_queue: Arc::new(Mutex::new(Vec::new())),
            active_jobs: Arc::new(Mutex::new(Vec::new())),
            max_concurrent_jobs,
        }
    }
    
    /// Create and queue a new export job
    pub fn create_job(
        &self,
        name: String,
        settings: ExportSettings,
        priority: JobPriority,
    ) -> Uuid {
        let job = ExportJob::new(name, settings, priority);
        let job_id = job.id;
        
        self.jobs.insert(job_id, job);
        
        // Add to queue
        let mut queue = self.job_queue.lock().unwrap();
        queue.push(job_id);
        
        // Sort queue by priority (highest first)
        queue.sort_by(|a, b| {
            let job_a = self.jobs.get(a);
            let job_b = self.jobs.get(b);
            
            match (job_a, job_b) {
                (Some(a), Some(b)) => b.priority.cmp(&a.priority),
                _ => std::cmp::Ordering::Equal,
            }
        });
        
        info!("Created export job: {} ({})", job_id, name);
        job_id
    }
    
    /// Get a job by ID
    pub fn get_job(&self, job_id: Uuid) -> Option<ExportJob> {
        self.jobs.get(&job_id).map(|job| job.clone())
    }
    
    /// Get all jobs
    pub fn get_all_jobs(&self) -> Vec<ExportJob> {
        self.jobs.iter().map(|entry| entry.value().clone()).collect()
    }
    
    /// Get jobs by status
    pub fn get_jobs_by_status(&self, status: JobStatus) -> Vec<ExportJob> {
        self.jobs
            .iter()
            .filter(|entry| entry.value().status == status)
            .map(|entry| entry.value().clone())
            .collect()
    }
    
    /// Cancel a job
    pub fn cancel_job(&self, job_id: Uuid) -> Result<()> {
        let mut job = self.jobs.get_mut(&job_id)
            .ok_or_else(|| ExportError::ExportFailed("Job not found".to_string()))?;
        
        if job.status.is_terminal() {
            return Err(ExportError::ExportFailed("Job already completed".to_string()));
        }
        
        job.update_status(JobStatus::Cancelled);
        
        // Remove from queues
        let mut queue = self.job_queue.lock().unwrap();
        queue.retain(|id| *id != job_id);
        
        let mut active = self.active_jobs.lock().unwrap();
        active.retain(|id| *id != job_id);
        
        info!("Cancelled export job: {}", job_id);
        Ok(())
    }
    
    /// Get the next job from the queue
    pub fn get_next_job(&self) -> Option<Uuid> {
        let mut queue = self.job_queue.lock().unwrap();
        let active = self.active_jobs.lock().unwrap();
        
        if active.len() >= self.max_concurrent_jobs {
            return None;
        }
        
        queue.pop()
    }
    
    /// Start a job
    pub fn start_job(&self, job_id: Uuid) -> Result<Arc<Mutex<dyn ExportProgress>>> {
        let mut job = self.jobs.get_mut(&job_id)
            .ok_or_else(|| ExportError::ExportFailed("Job not found".to_string()))?;
        
        if job.status != JobStatus::Queued {
            return Err(ExportError::ExportFailed("Job not in queued state".to_string()));
        }
        
        job.update_status(JobStatus::Preparing);
        drop(job); // Release the lock
        
        // Add to active jobs
        let mut active = self.active_jobs.lock().unwrap();
        active.push(job_id);
        
        // Create progress tracker
        let tracker = JobProgressTracker {
            job_id,
            manager: Arc::new(self.clone()),
            start_time: Instant::now(),
        };
        
        Ok(Arc::new(Mutex::new(tracker)))
    }
    
    /// Update job progress
    pub fn update_job_progress(&self, job_id: Uuid, progress: f32, message: String) {
        if let Some(mut job) = self.jobs.get_mut(&job_id) {
            job.update_progress(progress, message.clone());
            
            // Update status based on progress
            if progress < 50.0 && job.status != JobStatus::Rendering {
                job.update_status(JobStatus::Rendering);
            } else if progress >= 50.0 && progress < 95.0 && job.status != JobStatus::Encoding {
                job.update_status(JobStatus::Encoding);
            } else if progress >= 95.0 && job.status != JobStatus::Finalizing {
                job.update_status(JobStatus::Finalizing);
            }
            
            debug!("Job {} progress: {}% - {}", job_id, progress, message);
        }
    }
    
    /// Mark job as completed
    pub fn complete_job(&self, job_id: Uuid) {
        if let Some(mut job) = self.jobs.get_mut(&job_id) {
            job.update_status(JobStatus::Completed);
            job.progress = 100.0;
            job.progress_message = "Export completed".to_string();
            
            // Get output file size
            if let Ok(metadata) = std::fs::metadata(&job.settings.output_path) {
                job.output_size = Some(metadata.len());
            }
            
            info!("Export job completed: {} ({})", job_id, job.name);
        }
        
        // Remove from active jobs
        let mut active = self.active_jobs.lock().unwrap();
        active.retain(|id| *id != job_id);
    }
    
    /// Mark job as failed
    pub fn fail_job(&self, job_id: Uuid, error: String) {
        if let Some(mut job) = self.jobs.get_mut(&job_id) {
            job.update_status(JobStatus::Failed);
            job.error_message = Some(error.clone());
            
            error!("Export job failed: {} - {}", job_id, error);
        }
        
        // Remove from active jobs
        let mut active = self.active_jobs.lock().unwrap();
        active.retain(|id| *id != job_id);
    }
    
    /// Get job statistics
    pub fn get_statistics(&self) -> JobStatistics {
        let jobs: Vec<_> = self.jobs.iter().map(|entry| entry.value().clone()).collect();
        
        JobStatistics {
            total_jobs: jobs.len(),
            queued_jobs: jobs.iter().filter(|j| j.status == JobStatus::Queued).count(),
            active_jobs: jobs.iter().filter(|j| j.status.is_active()).count(),
            completed_jobs: jobs.iter().filter(|j| j.status == JobStatus::Completed).count(),
            failed_jobs: jobs.iter().filter(|j| j.status == JobStatus::Failed).count(),
            cancelled_jobs: jobs.iter().filter(|j| j.status == JobStatus::Cancelled).count(),
            average_duration: {
                let durations: Vec<_> = jobs.iter()
                    .filter_map(|j| j.duration())
                    .collect();
                
                if durations.is_empty() {
                    None
                } else {
                    let total: Duration = durations.iter().sum();
                    Some(total / durations.len() as u32)
                }
            },
        }
    }
}

impl Clone for ExportJobManager {
    fn clone(&self) -> Self {
        Self {
            jobs: self.jobs.clone(),
            job_queue: self.job_queue.clone(),
            active_jobs: self.active_jobs.clone(),
            max_concurrent_jobs: self.max_concurrent_jobs,
        }
    }
}

/// Job statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStatistics {
    pub total_jobs: usize,
    pub queued_jobs: usize,
    pub active_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
    pub cancelled_jobs: usize,
    pub average_duration: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_job_creation() {
        let manager = ExportJobManager::new(2);
        let settings = ExportSettings::default();
        
        let job_id = manager.create_job(
            "Test Export".to_string(),
            settings,
            JobPriority::Normal,
        );
        
        let job = manager.get_job(job_id).unwrap();
        assert_eq!(job.name, "Test Export");
        assert_eq!(job.status, JobStatus::Queued);
        assert_eq!(job.priority, JobPriority::Normal);
    }
    
    #[test]
    fn test_job_priority_queue() {
        let manager = ExportJobManager::new(2);
        let settings = ExportSettings::default();
        
        let low_job = manager.create_job("Low".to_string(), settings.clone(), JobPriority::Low);
        let high_job = manager.create_job("High".to_string(), settings.clone(), JobPriority::High);
        let normal_job = manager.create_job("Normal".to_string(), settings, JobPriority::Normal);
        
        // High priority job should be first
        assert_eq!(manager.get_next_job(), Some(high_job));
        assert_eq!(manager.get_next_job(), Some(normal_job));
        assert_eq!(manager.get_next_job(), Some(low_job));
    }
    
    #[test]
    fn test_job_cancellation() {
        let manager = ExportJobManager::new(2);
        let settings = ExportSettings::default();
        
        let job_id = manager.create_job("Test".to_string(), settings, JobPriority::Normal);
        
        assert!(manager.cancel_job(job_id).is_ok());
        
        let job = manager.get_job(job_id).unwrap();
        assert_eq!(job.status, JobStatus::Cancelled);
        
        // Can't cancel again
        assert!(manager.cancel_job(job_id).is_err());
    }
    
    #[test]
    fn test_job_statistics() {
        let manager = ExportJobManager::new(2);
        let settings = ExportSettings::default();
        
        // Create some jobs
        for i in 0..5 {
            manager.create_job(format!("Job {}", i), settings.clone(), JobPriority::Normal);
        }
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_jobs, 5);
        assert_eq!(stats.queued_jobs, 5);
        assert_eq!(stats.active_jobs, 0);
        assert_eq!(stats.completed_jobs, 0);
    }
}