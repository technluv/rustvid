//! Timeline management for Rust Video Editor
//! 
//! This crate provides timeline functionality including tracks, clips,
//! transitions, and timeline manipulation operations.

use thiserror::Error;
use uuid::Uuid;
use std::time::Duration;

#[derive(Error, Debug)]
pub enum TimelineError {
    #[error("Invalid clip position")]
    InvalidPosition,
    
    #[error("Track not found: {0}")]
    TrackNotFound(Uuid),
    
    #[error("Clip overlap detected")]
    ClipOverlap,
}

pub type Result<T> = std::result::Result<T, TimelineError>;

/// Represents a clip on the timeline
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Clip {
    pub id: Uuid,
    pub start_time: Duration,
    pub duration: Duration,
    pub source_path: String,
    pub in_point: Duration,
    pub out_point: Duration,
}

/// Represents a track containing clips
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Track {
    pub id: Uuid,
    pub name: String,
    pub clips: Vec<Clip>,
    pub enabled: bool,
    pub locked: bool,
}

/// The main timeline structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Timeline {
    pub id: Uuid,
    pub name: String,
    pub duration: Duration,
    pub tracks: Vec<Track>,
}

impl Timeline {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            duration: Duration::from_secs(0),
            tracks: Vec::new(),
        }
    }
    
    pub fn add_track(&mut self, name: String) -> &Track {
        let track = Track {
            id: Uuid::new_v4(),
            name,
            clips: Vec::new(),
            enabled: true,
            locked: false,
        };
        self.tracks.push(track);
        self.tracks.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_creation() {
        let mut timeline = Timeline::new("My Project".to_string());
        timeline.add_track("Video Track 1".to_string());
        
        assert_eq!(timeline.tracks.len(), 1);
        assert_eq!(timeline.tracks[0].name, "Video Track 1");
    }
}