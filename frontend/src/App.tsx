import React, { useEffect, useCallback, useState } from 'react';
import { Timeline } from './components/timeline';
import { useTimelineStore } from './store/timelineStore';
import { ErrorBoundary } from './components/ErrorBoundary';
import { LoadingSpinner } from './components/LoadingSpinner';
import './App.css';

function App() {
  const { addTrack, addClip, play, pause, isPlaying, seekTo, currentTime, duration, setCurrentTime } = useTimelineStore();
  const [isLoading, setIsLoading] = useState(true);
  const [highContrast, setHighContrast] = useState(false);
  const [, /*setUndoStack*/] = useState<any[]>([]);

  // Initialize keyboard shortcuts
  const handleKeyDown = useCallback((event: KeyboardEvent) => {
    // Prevent default browser shortcuts when focused on our app
    if (event.target === document.body || (event.target as HTMLElement).closest('[data-video-editor]')) {
      switch (event.code) {
        case 'Space':
          event.preventDefault();
          if (isPlaying) {
            pause();
          } else {
            play();
          }
          break;
        case 'ArrowLeft':
          event.preventDefault();
          seekTo(Math.max(0, currentTime - 1));
          break;
        case 'ArrowRight':
          event.preventDefault();
          seekTo(Math.min(duration, currentTime + 1));
          break;
        case 'KeyZ':
          if (event.ctrlKey || event.metaKey) {
            event.preventDefault();
            // Implement undo functionality
            console.log('Undo triggered');
          }
          break;
        case 'KeyH':
          if (event.ctrlKey || event.metaKey) {
            event.preventDefault();
            setHighContrast(!highContrast);
          }
          break;
      }
    }
  }, [isPlaying, play, pause, seekTo, currentTime, duration, highContrast]);

  // Add keyboard event listeners
  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  // Apply high contrast mode
  useEffect(() => {
    document.documentElement.classList.toggle('high-contrast', highContrast);
  }, [highContrast]);

  // Initialize with sample data
  useEffect(() => {
    setIsLoading(true);
    // Add sample tracks
    addTrack('video', 'Video 1');
    addTrack('video', 'Video 2');
    addTrack('audio', 'Audio 1');
    addTrack('audio', 'Audio 2');

    // Add sample clips after tracks are created
    setTimeout(() => {
      const tracks = useTimelineStore.getState().tracks;
      
      if (tracks[0]) {
        addClip(tracks[0].id, {
          name: 'Intro.mp4',
          type: 'video',
          startTime: 0,
          duration: 10,
          sourceIn: 0,
          sourceOut: 10,
          selected: false,
          locked: false,
        });
        
        addClip(tracks[0].id, {
          name: 'Main Content.mp4',
          type: 'video',
          startTime: 12,
          duration: 45,
          sourceIn: 0,
          sourceOut: 45,
          selected: false,
          locked: false,
        });
      }
      
      if (tracks[1]) {
        addClip(tracks[1].id, {
          name: 'B-Roll 1.mp4',
          type: 'video',
          startTime: 5,
          duration: 8,
          sourceIn: 0,
          sourceOut: 8,
          selected: false,
          locked: false,
        });
        
        addClip(tracks[1].id, {
          name: 'B-Roll 2.mp4',
          type: 'video',
          startTime: 25,
          duration: 15,
          sourceIn: 0,
          sourceOut: 15,
          selected: false,
          locked: false,
        });
      }
      
      if (tracks[2]) {
        addClip(tracks[2].id, {
          name: 'Background Music.mp3',
          type: 'audio',
          startTime: 0,
          duration: 60,
          sourceIn: 0,
          sourceOut: 60,
          selected: false,
          locked: false,
          waveformData: Array.from({ length: 200 }, () => Math.random()),
        });
      }
      
      if (tracks[3]) {
        addClip(tracks[3].id, {
          name: 'Voiceover.mp3',
          type: 'audio',
          startTime: 10,
          duration: 30,
          sourceIn: 0,
          sourceOut: 30,
          selected: false,
          locked: false,
          waveformData: Array.from({ length: 150 }, () => Math.random() * 0.8),
        });
      }
    }, 100);
    
    // Simulate loading time
    setTimeout(() => {
      setIsLoading(false);
    }, 1500);
  }, []);

  if (isLoading) {
    return <LoadingSpinner message="Loading Video Editor..." />;
  }

  return (
    <div 
      className={`h-screen bg-gray-900 text-white transition-colors duration-300 ${
        highContrast ? 'high-contrast-theme' : ''
      }`}
      data-video-editor
      role="application"
      aria-label="Rust Video Editor Application"
    >
      <header 
        className="h-12 bg-gray-800 border-b border-gray-700 flex items-center justify-between px-4"
        role="banner"
      >
        <h1 className="text-lg font-semibold">Rust Video Editor</h1>
        
        {/* Accessibility Controls */}
        <div className="flex items-center gap-2">
          <button
            onClick={() => setHighContrast(!highContrast)}
            className="px-3 py-1 text-sm rounded bg-gray-700 hover:bg-gray-600 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
            aria-label={`${highContrast ? 'Disable' : 'Enable'} high contrast mode`}
            title="Toggle high contrast mode (Ctrl+H)"
          >
            {highContrast ? '🌙' : '☀️'} {highContrast ? 'Normal' : 'High Contrast'}
          </button>
        </div>
      </header>
      
      <main 
        className="h-[calc(100vh-3rem)] flex flex-col lg:flex-row"
        role="main"
        aria-label="Video editing workspace"
      >
        <ErrorBoundary>
          {/* Preview Panel */}
          <section 
            className="flex-1 lg:w-1/2 bg-gray-850 border-b lg:border-b-0 lg:border-r border-gray-700 flex items-center justify-center min-h-[300px]"
            role="region"
            aria-label="Video preview area"
          >
            <div className="text-gray-500 text-center">
              <div className="mb-2 text-4xl opacity-50">📺</div>
              <div>Video Preview</div>
              <div className="text-sm mt-1 opacity-70">
                Preview will appear here when playing
              </div>
            </div>
          </section>
          
          {/* Timeline Panel */}
          <section 
            className="flex-1 lg:w-1/2 flex flex-col min-h-[400px]"
            role="region"
            aria-label="Timeline editor"
          >
            <Timeline className="flex-1" />
          </section>
        </ErrorBoundary>
      </main>
      
      {/* Keyboard Shortcuts Help */}
      <div 
        className="fixed bottom-4 right-4 bg-gray-800 rounded-lg p-2 text-xs opacity-70 hover:opacity-100 transition-opacity"
        role="complementary"
        aria-label="Keyboard shortcuts"
      >
        <div className="font-semibold mb-1">Shortcuts:</div>
        <div>Space: Play/Pause</div>
        <div>← →: Frame step</div>
        <div>Ctrl+Z: Undo</div>
        <div>Ctrl+H: High contrast</div>
      </div>
    </div>
  );
}

export default App;