import React, { useEffect, useCallback, useState } from 'react';
import { Timeline } from './components/timeline';
import { useTimelineStore } from './store/timelineStore';
import { ErrorBoundary } from './components/ErrorBoundary';
import { LoadingSpinner } from './components/LoadingSpinner';
import { AccessibilityMenu } from './components/AccessibilityMenu';
import { useKeyboardShortcuts } from './hooks/useKeyboardShortcuts';
import { accessibilityManager } from './utils/accessibility';
import './App.css';

function App() {
  const { addTrack, addClip, play, pause, isPlaying, seekTo, currentTime, duration, setCurrentTime, tracks, clips, selectedClips } = useTimelineStore();
  const [isLoading, setIsLoading] = useState(true);
  const [isAccessibilityMenuOpen, setAccessibilityMenuOpen] = useState(false);
  const [, /*setUndoStack*/] = useState<any[]>([]);
  const [copiedClips, setCopiedClips] = useState<any[]>([]);

  // Implement action handlers for keyboard shortcuts
  const actions = {
    // Playback actions
    playPause: useCallback(() => {
      if (isPlaying) {
        pause();
      } else {
        play();
      }
    }, [isPlaying, play, pause]),
    
    stop: useCallback(() => {
      pause();
      seekTo(0);
    }, [pause, seekTo]),
    
    nextFrame: useCallback(() => {
      seekTo(Math.min(duration, currentTime + 1/30)); // Assuming 30fps
    }, [seekTo, currentTime, duration]),
    
    prevFrame: useCallback(() => {
      seekTo(Math.max(0, currentTime - 1/30));
    }, [seekTo, currentTime]),
    
    // Editing actions
    cut: useCallback(() => {
      // Implement cut functionality
      accessibilityManager.announce('Cut selected clips');
      console.log('Cut:', selectedClips);
    }, [selectedClips]),
    
    copy: useCallback(() => {
      setCopiedClips(selectedClips);
      accessibilityManager.announce(`Copied ${selectedClips.length} clips`);
    }, [selectedClips]),
    
    paste: useCallback(() => {
      if (copiedClips.length > 0) {
        // Implement paste functionality
        accessibilityManager.announce(`Pasted ${copiedClips.length} clips`);
        console.log('Paste:', copiedClips);
      }
    }, [copiedClips]),
    
    delete: useCallback(() => {
      // Implement delete functionality
      accessibilityManager.announce(`Deleted ${selectedClips.length} clips`);
      console.log('Delete:', selectedClips);
    }, [selectedClips]),
    
    undo: useCallback(() => {
      accessibilityManager.announce('Undo last action');
      console.log('Undo triggered');
    }, []),
    
    redo: useCallback(() => {
      accessibilityManager.announce('Redo last action');
      console.log('Redo triggered');
    }, []),
    
    // View actions
    zoomIn: useCallback(() => {
      accessibilityManager.announce('Zoomed in timeline');
      console.log('Zoom in');
    }, []),
    
    zoomOut: useCallback(() => {
      accessibilityManager.announce('Zoomed out timeline');
      console.log('Zoom out');
    }, []),
    
    fitToWindow: useCallback(() => {
      accessibilityManager.announce('Fit timeline to window');
      console.log('Fit to window');
    }, []),
  };

  // Initialize keyboard shortcuts hook
  const { isShortcutsDialogOpen, setShortcutsDialogOpen } = useKeyboardShortcuts(actions);

  // Handle shortcuts dialog from keyboard shortcut
  useEffect(() => {
    if (isShortcutsDialogOpen) {
      setAccessibilityMenuOpen(true);
    }
  }, [isShortcutsDialogOpen]);

  // Create skip navigation links on mount
  useEffect(() => {
    const skipLink = accessibilityManager.createSkipLink('timeline', 'Skip to timeline');
    skipLink.className = 'skip-link sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-50 focus:px-4 focus:py-2 focus:bg-blue-600 focus:text-white focus:rounded';
    document.body.insertBefore(skipLink, document.body.firstChild);
    
    return () => {
      if (skipLink.parentNode) {
        skipLink.parentNode.removeChild(skipLink);
      }
    };
  }, []);

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
      className="h-screen bg-gray-900 text-white transition-colors duration-300"
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
        <nav className="flex items-center gap-2" role="navigation" aria-label="Accessibility controls">
          <button
            onClick={() => setAccessibilityMenuOpen(true)}
            className="px-3 py-1 text-sm rounded bg-gray-700 hover:bg-gray-600 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
            aria-label="Open accessibility settings"
            title="Accessibility settings (?)"
          >
            ♿ Accessibility
          </button>
          <button
            onClick={() => accessibilityManager.toggleHighContrast()}
            className="px-3 py-1 text-sm rounded bg-gray-700 hover:bg-gray-600 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
            aria-label={`${accessibilityManager.isHighContrastEnabled() ? 'Disable' : 'Enable'} high contrast mode`}
            title="Toggle high contrast mode (Alt+H)"
          >
            {accessibilityManager.isHighContrastEnabled() ? '🌙' : '☀️'} Contrast
          </button>
        </nav>
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
            data-component="preview"
            tabIndex={0}
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
            id="timeline"
            className="flex-1 lg:w-1/2 flex flex-col min-h-[400px]"
            role="region"
            aria-label="Timeline editor"
            data-component="timeline"
            tabIndex={0}
          >
            <Timeline className="flex-1" />
          </section>
        </ErrorBoundary>
      </main>
      
      {/* Keyboard Shortcuts Help */}
      <aside 
        className="fixed bottom-4 right-4 bg-gray-800 rounded-lg p-2 text-xs opacity-70 hover:opacity-100 transition-opacity"
        role="complementary"
        aria-label="Quick keyboard shortcuts reference"
      >
        <div className="font-semibold mb-1">Quick Shortcuts:</div>
        <div>Space: Play/Pause</div>
        <div>← →: Frame step</div>
        <div>Ctrl+Z: Undo</div>
        <div>Alt+H: High contrast</div>
        <div>?: All shortcuts</div>
      </aside>
      
      {/* Accessibility Menu */}
      <AccessibilityMenu 
        isOpen={isAccessibilityMenuOpen || isShortcutsDialogOpen}
        onClose={() => {
          setAccessibilityMenuOpen(false);
          setShortcutsDialogOpen(false);
        }}
      />
    </div>
  );
}

export default App;