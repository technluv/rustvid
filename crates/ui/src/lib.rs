//! User interface for Rust Video Editor
//! 
//! This crate provides the UI layer for the video editor,
//! with support for multiple UI frameworks.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("UI initialization failed: {0}")]
    InitializationError(String),
    
    #[error("Render error: {0}")]
    RenderError(String),
    
    #[error("Event handling error: {0}")]
    EventError(String),
}

pub type Result<T> = std::result::Result<T, UiError>;

/// Main application state
#[derive(Debug)]
pub struct AppState {
    pub project_name: String,
    pub is_playing: bool,
    pub current_time: std::time::Duration,
    pub zoom_level: f32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            project_name: "Untitled Project".to_string(),
            is_playing: false,
            current_time: std::time::Duration::from_secs(0),
            zoom_level: 1.0,
        }
    }
}

/// Trait for UI implementations
pub trait UserInterface {
    fn initialize(&mut self) -> Result<()>;
    fn run(&mut self) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
}

/// UI configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UiConfig {
    pub window_title: String,
    pub initial_width: u32,
    pub initial_height: u32,
    pub theme: Theme,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            window_title: "Rust Video Editor".to_string(),
            initial_width: 1280,
            initial_height: 720,
            theme: Theme::Dark,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert_eq!(state.project_name, "Untitled Project");
        assert!(!state.is_playing);
        assert_eq!(state.zoom_level, 1.0);
    }
    
    #[test]
    fn test_ui_config_serialization() {
        let config = UiConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: UiConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.window_title, deserialized.window_title);
        assert_eq!(config.initial_width, deserialized.initial_width);
    }
}