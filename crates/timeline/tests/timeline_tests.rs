//! Comprehensive tests for timeline functionality

use video_editor_timeline::*;
use std::time::Duration;
use uuid::Uuid;

#[cfg(test)]
mod basic_timeline_tests {
    use super::*;
    
    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline::new("Test Timeline".to_string());
        
        assert_eq!(timeline.name, "Test Timeline");
        assert_eq!(timeline.duration, Duration::from_secs(0));
        assert_eq!(timeline.tracks.len(), 0);
        assert!(!timeline.id.is_nil());
    }
    
    #[test]
    fn test_track_addition() {
        let mut timeline = Timeline::new("Test".to_string());
        
        let track = timeline.add_track("Video Track 1".to_string());
        assert_eq!(track.name, "Video Track 1");
        assert!(track.enabled);
        assert!(!track.locked);
        assert_eq!(track.clips.len(), 0);
        
        assert_eq!(timeline.tracks.len(), 1);
    }
    
    #[test]
    fn test_multiple_tracks() {
        let mut timeline = Timeline::new("Multi-track Test".to_string());
        
        timeline.add_track("Video 1".to_string());
        timeline.add_track("Audio 1".to_string());
        timeline.add_track("Video 2".to_string());
        
        assert_eq!(timeline.tracks.len(), 3);
        assert_eq!(timeline.tracks[0].name, "Video 1");
        assert_eq!(timeline.tracks[1].name, "Audio 1");
        assert_eq!(timeline.tracks[2].name, "Video 2");
    }
}

#[cfg(test)]
mod clip_tests {
    use super::*;
    
    fn create_test_clip(start_time: Duration, duration: Duration) -> Clip {
        Clip {
            id: Uuid::new_v4(),
            start_time,
            duration,
            source_path: "test_video.mp4".to_string(),
            in_point: Duration::from_secs(0),
            out_point: duration,
        }
    }
    
    #[test]
    fn test_clip_creation() {
        let clip = create_test_clip(Duration::from_secs(10), Duration::from_secs(5));
        
        assert_eq!(clip.start_time, Duration::from_secs(10));
        assert_eq!(clip.duration, Duration::from_secs(5));
        assert_eq!(clip.source_path, "test_video.mp4");
        assert_eq!(clip.in_point, Duration::from_secs(0));
        assert_eq!(clip.out_point, Duration::from_secs(5));
    }
    
    #[test]
    fn test_clip_timing() {
        let clip = create_test_clip(Duration::from_secs(30), Duration::from_secs(15));
        
        // Clip should end at start_time + duration
        let end_time = clip.start_time + clip.duration;
        assert_eq!(end_time, Duration::from_secs(45));
    }
    
    #[test]
    fn test_clip_trimming() {
        let mut clip = create_test_clip(Duration::from_secs(0), Duration::from_secs(10));
        
        // Trim the clip (adjust in/out points)
        clip.in_point = Duration::from_secs(2);
        clip.out_point = Duration::from_secs(8);
        clip.duration = clip.out_point - clip.in_point;
        
        assert_eq!(clip.duration, Duration::from_secs(6));
        assert_eq!(clip.in_point, Duration::from_secs(2));
        assert_eq!(clip.out_point, Duration::from_secs(8));
    }
}

#[cfg(test)]
mod track_manipulation_tests {
    use super::*;
    
    fn setup_track_with_clips() -> Track {
        let mut track = Track {
            id: Uuid::new_v4(),
            name: "Test Track".to_string(),
            clips: Vec::new(),
            enabled: true,
            locked: false,
        };
        
        // Add clips at different positions
        track.clips.push(Clip {
            id: Uuid::new_v4(),
            start_time: Duration::from_secs(0),
            duration: Duration::from_secs(5),
            source_path: "clip1.mp4".to_string(),
            in_point: Duration::from_secs(0),
            out_point: Duration::from_secs(5),
        });
        
        track.clips.push(Clip {
            id: Uuid::new_v4(),
            start_time: Duration::from_secs(10),
            duration: Duration::from_secs(3),
            source_path: "clip2.mp4".to_string(),
            in_point: Duration::from_secs(0),
            out_point: Duration::from_secs(3),
        });
        
        track
    }
    
    #[test]
    fn test_track_clip_sorting() {
        let mut track = setup_track_with_clips();
        
        // Add a clip that should be first chronologically
        track.clips.push(Clip {
            id: Uuid::new_v4(),
            start_time: Duration::from_millis(500),
            duration: Duration::from_secs(2),
            source_path: "early_clip.mp4".to_string(),
            in_point: Duration::from_secs(0),
            out_point: Duration::from_secs(2),
        });
        
        // Sort clips by start time
        track.clips.sort_by_key(|clip| clip.start_time);
        
        assert_eq!(track.clips[0].start_time, Duration::from_secs(0));
        assert_eq!(track.clips[1].start_time, Duration::from_millis(500));
        assert_eq!(track.clips[2].start_time, Duration::from_secs(10));
    }
    
    #[test]
    fn test_track_enabled_disabled() {
        let mut track = setup_track_with_clips();
        
        assert!(track.enabled);
        
        track.enabled = false;
        assert!(!track.enabled);
    }
    
    #[test]
    fn test_track_locked() {
        let mut track = setup_track_with_clips();
        
        assert!(!track.locked);
        
        track.locked = true;
        assert!(track.locked);
    }
}

#[cfg(test)]
mod timeline_operations_tests {
    use super::*;
    
    fn create_timeline_with_content() -> Timeline {
        let mut timeline = Timeline::new("Test Project".to_string());
        
        let video_track = timeline.add_track("Video".to_string());
        let video_track_id = video_track.id;
        
        // Find the track and add clips
        if let Some(track) = timeline.tracks.iter_mut().find(|t| t.id == video_track_id) {
            track.clips.push(Clip {
                id: Uuid::new_v4(),
                start_time: Duration::from_secs(0),
                duration: Duration::from_secs(10),
                source_path: "intro.mp4".to_string(),
                in_point: Duration::from_secs(0),
                out_point: Duration::from_secs(10),
            });
            
            track.clips.push(Clip {
                id: Uuid::new_v4(),
                start_time: Duration::from_secs(12),
                duration: Duration::from_secs(8),
                source_path: "main.mp4".to_string(),
                in_point: Duration::from_secs(5),
                out_point: Duration::from_secs(13),
            });
        }
        
        timeline
    }
    
    #[test]
    fn test_timeline_duration_calculation() {
        let timeline = create_timeline_with_content();
        
        // Find the latest end time across all tracks
        let mut max_end = Duration::from_secs(0);
        for track in &timeline.tracks {
            for clip in &track.clips {
                let clip_end = clip.start_time + clip.duration;
                if clip_end > max_end {
                    max_end = clip_end;
                }
            }
        }
        
        // Should be 12 + 8 = 20 seconds (start of last clip + its duration)
        assert_eq!(max_end, Duration::from_secs(20));
    }
    
    #[test]
    fn test_find_clips_at_time() {
        let timeline = create_timeline_with_content();
        
        // Find clips at various time points
        let clips_at_5s: Vec<_> = timeline.tracks
            .iter()
            .flat_map(|track| &track.clips)
            .filter(|clip| {
                let clip_start = clip.start_time;
                let clip_end = clip_start + clip.duration;
                Duration::from_secs(5) >= clip_start && Duration::from_secs(5) < clip_end
            })
            .collect();
        
        assert_eq!(clips_at_5s.len(), 1); // Only first clip
        assert_eq!(clips_at_5s[0].source_path, "intro.mp4");
        
        let clips_at_15s: Vec<_> = timeline.tracks
            .iter()
            .flat_map(|track| &track.clips)
            .filter(|clip| {
                let clip_start = clip.start_time;
                let clip_end = clip_start + clip.duration;
                Duration::from_secs(15) >= clip_start && Duration::from_secs(15) < clip_end
            })
            .collect();
        
        assert_eq!(clips_at_15s.len(), 1); // Only second clip
        assert_eq!(clips_at_15s[0].source_path, "main.mp4");
    }
    
    #[test]
    fn test_gap_detection() {
        let timeline = create_timeline_with_content();
        
        // There should be a gap between clips (10-12 seconds)
        let track = &timeline.tracks[0];
        let clips: Vec<_> = track.clips.iter().collect();
        
        if clips.len() >= 2 {
            let first_end = clips[0].start_time + clips[0].duration;
            let second_start = clips[1].start_time;
            
            assert!(second_start > first_end, "Expected a gap between clips");
            let gap_duration = second_start - first_end;
            assert_eq!(gap_duration, Duration::from_secs(2));
        }
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;
    
    #[test]
    fn test_timeline_serialization() {
        let timeline = Timeline::new("Serialization Test".to_string());
        
        let json = serde_json::to_string(&timeline).unwrap();
        let deserialized: Timeline = serde_json::from_str(&json).unwrap();
        
        assert_eq!(timeline.name, deserialized.name);
        assert_eq!(timeline.duration, deserialized.duration);
        assert_eq!(timeline.tracks.len(), deserialized.tracks.len());
    }
    
    #[test]
    fn test_track_serialization() {
        let track = Track {
            id: Uuid::new_v4(),
            name: "Test Track".to_string(),
            clips: Vec::new(),
            enabled: true,
            locked: false,
        };
        
        let json = serde_json::to_string(&track).unwrap();
        let deserialized: Track = serde_json::from_str(&json).unwrap();
        
        assert_eq!(track.name, deserialized.name);
        assert_eq!(track.enabled, deserialized.enabled);
        assert_eq!(track.locked, deserialized.locked);
    }
    
    #[test]
    fn test_clip_serialization() {
        let clip = Clip {
            id: Uuid::new_v4(),
            start_time: Duration::from_secs(10),
            duration: Duration::from_secs(5),
            source_path: "test.mp4".to_string(),
            in_point: Duration::from_secs(2),
            out_point: Duration::from_secs(7),
        };
        
        let json = serde_json::to_string(&clip).unwrap();
        let deserialized: Clip = serde_json::from_str(&json).unwrap();
        
        assert_eq!(clip.start_time, deserialized.start_time);
        assert_eq!(clip.duration, deserialized.duration);
        assert_eq!(clip.source_path, deserialized.source_path);
        assert_eq!(clip.in_point, deserialized.in_point);
        assert_eq!(clip.out_point, deserialized.out_point);
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;
    
    #[test]
    fn test_timeline_error_types() {
        use TimelineError::*;
        
        let invalid_pos = InvalidPosition;
        let track_not_found = TrackNotFound(Uuid::new_v4());
        let clip_overlap = ClipOverlap;
        
        // Test error display
        assert!(!format!("{}", invalid_pos).is_empty());
        assert!(!format!("{}", track_not_found).is_empty());
        assert!(!format!("{}", clip_overlap).is_empty());
    }
    
    #[test]
    fn test_result_type() {
        fn test_function() -> Result<i32> {
            Ok(42)
        }
        
        let result = test_function();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_large_timeline_creation() {
        let start = Instant::now();
        
        let mut timeline = Timeline::new("Performance Test".to_string());
        
        // Create 10 tracks with 100 clips each
        for track_idx in 0..10 {
            let track = timeline.add_track(format!("Track {}", track_idx));
            let track_id = track.id;
            
            if let Some(track) = timeline.tracks.iter_mut().find(|t| t.id == track_id) {
                for clip_idx in 0..100 {
                    track.clips.push(Clip {
                        id: Uuid::new_v4(),
                        start_time: Duration::from_secs(clip_idx * 5),
                        duration: Duration::from_secs(4),
                        source_path: format!("clip_{}.mp4", clip_idx),
                        in_point: Duration::from_secs(0),
                        out_point: Duration::from_secs(4),
                    });
                }
            }
        }
        
        let elapsed = start.elapsed();
        println!("Created timeline with 10 tracks and 1000 clips in {:?}", elapsed);
        
        assert_eq!(timeline.tracks.len(), 10);
        assert!(timeline.tracks.iter().all(|t| t.clips.len() == 100));
        assert!(elapsed.as_millis() < 1000); // Should be under 1 second
    }
    
    #[test]
    fn test_clip_search_performance() {
        let mut timeline = Timeline::new("Search Test".to_string());
        let track = timeline.add_track("Search Track".to_string());
        let track_id = track.id;
        
        // Add 1000 clips
        if let Some(track) = timeline.tracks.iter_mut().find(|t| t.id == track_id) {
            for i in 0..1000 {
                track.clips.push(Clip {
                    id: Uuid::new_v4(),
                    start_time: Duration::from_secs(i * 2),
                    duration: Duration::from_secs(1),
                    source_path: format!("clip_{}.mp4", i),
                    in_point: Duration::from_secs(0),
                    out_point: Duration::from_secs(1),
                });
            }
        }
        
        let start = Instant::now();
        
        // Search for clips at various times
        for search_time in (0..2000).step_by(100) {
            let search_duration = Duration::from_secs(search_time);
            let _clips: Vec<_> = timeline.tracks
                .iter()
                .flat_map(|track| &track.clips)
                .filter(|clip| {
                    let clip_start = clip.start_time;
                    let clip_end = clip_start + clip.duration;
                    search_duration >= clip_start && search_duration < clip_end
                })
                .collect();
        }
        
        let elapsed = start.elapsed();
        println!("Performed 20 searches across 1000 clips in {:?}", elapsed);
        
        assert!(elapsed.as_millis() < 100); // Should be very fast
    }
}