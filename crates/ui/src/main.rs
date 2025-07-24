// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use video_editor_core::{VideoEngine, EngineConfig};
use video_editor_timeline::Timeline;
use video_editor_ui::{AppState, UiConfig};
use tauri::Manager;
use std::sync::{Arc, Mutex};

// State wrapper for Tauri
struct AppStateWrapper(Arc<Mutex<AppState>>);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_app_state(state: tauri::State<AppStateWrapper>) -> Result<AppState, String> {
    state.0.lock()
        .map_err(|e| format!("Failed to lock app state: {}", e))
        .map(|guard| guard.clone())
}

#[tauri::command]
fn set_playing(playing: bool, state: tauri::State<AppStateWrapper>) -> Result<(), String> {
    let mut app_state = state.0.lock()
        .map_err(|e| format!("Failed to lock app state: {}", e))?;
    app_state.is_playing = playing;
    Ok(())
}

#[tauri::command]
fn set_zoom_level(zoom: f32, state: tauri::State<AppStateWrapper>) -> Result<(), String> {
    let mut app_state = state.0.lock()
        .map_err(|e| format!("Failed to lock app state: {}", e))?;
    app_state.zoom_level = zoom;
    Ok(())
}

#[tauri::command]
fn create_new_project(name: String, state: tauri::State<AppStateWrapper>) -> Result<(), String> {
    let mut app_state = state.0.lock()
        .map_err(|e| format!("Failed to lock app state: {}", e))?;
    app_state.project_name = name;
    app_state.is_playing = false;
    app_state.current_time = std::time::Duration::from_secs(0);
    Ok(())
}

fn main() {
    // Initialize app state
    let app_state = AppStateWrapper(Arc::new(Mutex::new(AppState::default())));

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_app_state,
            set_playing,
            set_zoom_level,
            create_new_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Add Clone derive to AppState for the command
impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            project_name: self.project_name.clone(),
            is_playing: self.is_playing,
            current_time: self.current_time,
            zoom_level: self.zoom_level,
        }
    }
}