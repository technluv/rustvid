import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import VideoTimeline from "./components/VideoTimeline";
import ControlPanel from "./components/ControlPanel";
import "./App.css";

interface AppState {
  project_name: string;
  is_playing: boolean;
  current_time: number;
  zoom_level: number;
}

function App() {
  const [appState, setAppState] = useState<AppState | null>(null);
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  useEffect(() => {
    // Load initial app state
    loadAppState();
  }, []);

  async function loadAppState() {
    try {
      const state = await invoke<AppState>("get_app_state");
      setAppState(state);
    } catch (error) {
      console.error("Failed to load app state:", error);
    }
  }

  async function togglePlayback() {
    if (appState) {
      try {
        await invoke("set_playing", { playing: !appState.is_playing });
        await loadAppState(); // Reload state
      } catch (error) {
        console.error("Failed to toggle playback:", error);
      }
    }
  }

  async function updateZoom(zoom: number) {
    try {
      await invoke("set_zoom_level", { zoom });
      await loadAppState(); // Reload state
    } catch (error) {
      console.error("Failed to update zoom:", error);
    }
  }

  async function createNewProject(projectName: string) {
    try {
      await invoke("create_new_project", { name: projectName });
      await loadAppState(); // Reload state
    } catch (error) {
      console.error("Failed to create new project:", error);
    }
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <header className="header">
        <h1>Rust Video Editor</h1>
        {appState && (
          <div className="project-info">
            <span>Project: {appState.project_name}</span>
            <button onClick={() => {
              const name = prompt("Enter project name:");
              if (name) createNewProject(name);
            }}>
              New Project
            </button>
          </div>
        )}
      </header>

      <main className="main-content">
        <div className="editor-container">
          {appState && (
            <>
              <ControlPanel
                isPlaying={appState.is_playing}
                onTogglePlayback={togglePlayback}
                currentTime={appState.current_time}
                zoomLevel={appState.zoom_level}
                onZoomChange={updateZoom}
              />
              <VideoTimeline
                zoomLevel={appState.zoom_level}
                currentTime={appState.current_time}
              />
            </>
          )}
        </div>

        <div className="demo-section">
          <h2>Tauri + React Demo</h2>
          <form
            className="row"
            onSubmit={(e) => {
              e.preventDefault();
              greet();
            }}
          >
            <input
              id="greet-input"
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder="Enter a name..."
            />
            <button type="submit">Greet</button>
          </form>
          <p>{greetMsg}</p>
        </div>
      </main>
    </div>
  );
}

export default App;