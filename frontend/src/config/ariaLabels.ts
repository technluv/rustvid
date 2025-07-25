// ARIA labels configuration for screen reader support
export const ariaLabels = {
  // Timeline components
  timeline: {
    container: 'Timeline editor with tracks and clips',
    track: (type: string, name: string) => `${type} track: ${name}`,
    clip: (name: string, startTime: number, duration: number) => 
      `Clip: ${name}, starts at ${formatTime(startTime)}, duration ${formatTime(duration)}`,
    playhead: (time: number) => `Playhead at ${formatTime(time)}`,
    scrubber: 'Timeline scrubber, drag to seek',
    zoomSlider: 'Timeline zoom control',
    addTrack: 'Add new track',
    deleteTrack: (name: string) => `Delete track ${name}`,
    muteTrack: (name: string) => `Mute track ${name}`,
    soloTrack: (name: string) => `Solo track ${name}`,
    lockTrack: (name: string) => `Lock track ${name}`,
  },

  // Playback controls
  playback: {
    playPause: (isPlaying: boolean) => isPlaying ? 'Pause' : 'Play',
    stop: 'Stop playback',
    nextFrame: 'Next frame',
    prevFrame: 'Previous frame',
    jumpStart: 'Jump to start',
    jumpEnd: 'Jump to end',
    loop: (isLooping: boolean) => `Loop playback ${isLooping ? 'enabled' : 'disabled'}`,
    speed: (speed: number) => `Playback speed: ${speed}x`,
    volume: (volume: number) => `Volume: ${Math.round(volume * 100)}%`,
  },

  // Editing controls
  editing: {
    cut: 'Cut selected clips',
    copy: 'Copy selected clips',
    paste: 'Paste clips',
    delete: 'Delete selected clips',
    undo: 'Undo last action',
    redo: 'Redo last undone action',
    selectAll: 'Select all clips',
    deselectAll: 'Deselect all clips',
    splitClip: 'Split clip at playhead',
    trimStart: 'Trim start to playhead',
    trimEnd: 'Trim end to playhead',
    rippleDelete: 'Ripple delete',
  },

  // Preview window
  preview: {
    container: 'Video preview window',
    fullscreen: 'Toggle fullscreen',
    qualitySelector: 'Preview quality',
    fitToWindow: 'Fit preview to window',
    actualSize: 'Show actual size',
  },

  // Properties panel
  properties: {
    container: 'Properties panel',
    clipProperties: (clipName: string) => `Properties for ${clipName}`,
    position: 'Position controls',
    scale: 'Scale controls',
    rotation: 'Rotation controls',
    opacity: 'Opacity control',
    effects: 'Effects list',
    addEffect: 'Add effect',
    removeEffect: (effectName: string) => `Remove ${effectName} effect`,
  },

  // Media library
  media: {
    container: 'Media library',
    importMedia: 'Import media files',
    searchMedia: 'Search media library',
    mediaItem: (name: string, type: string) => `${type}: ${name}`,
    mediaPreview: (name: string) => `Preview of ${name}`,
    addToTimeline: (name: string) => `Add ${name} to timeline`,
  },

  // Export/render
  export: {
    container: 'Export settings',
    format: 'Export format',
    quality: 'Export quality',
    resolution: 'Export resolution',
    framerate: 'Export framerate',
    startExport: 'Start export',
    cancelExport: 'Cancel export',
    exportProgress: (percent: number) => `Export progress: ${percent}%`,
  },

  // Accessibility
  accessibility: {
    menu: 'Accessibility settings',
    highContrast: 'High contrast mode',
    fontSize: 'Font size adjustment',
    reducedMotion: 'Reduced motion',
    screenReader: 'Screen reader mode',
    keyboardShortcuts: 'Keyboard shortcuts',
    resetSettings: 'Reset accessibility settings',
  },

  // Notifications
  notifications: {
    success: 'Success notification',
    error: 'Error notification',
    warning: 'Warning notification',
    info: 'Information notification',
  },

  // Dialog/Modal
  dialog: {
    close: 'Close dialog',
    save: 'Save changes',
    cancel: 'Cancel',
    confirm: 'Confirm',
  },
};

// Helper function to format time for screen readers
function formatTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  const frames = Math.floor((seconds % 1) * 30); // Assuming 30fps

  const parts = [];
  if (hours > 0) parts.push(`${hours} hour${hours !== 1 ? 's' : ''}`);
  if (minutes > 0) parts.push(`${minutes} minute${minutes !== 1 ? 's' : ''}`);
  if (secs > 0) parts.push(`${secs} second${secs !== 1 ? 's' : ''}`);
  if (frames > 0) parts.push(`${frames} frame${frames !== 1 ? 's' : ''}`);

  return parts.join(', ') || '0 seconds';
}

// Helper function to apply ARIA labels to elements
export function applyAriaLabels(container: HTMLElement, labelMap: Record<string, string>) {
  Object.entries(labelMap).forEach(([selector, label]) => {
    const elements = container.querySelectorAll(selector);
    elements.forEach(element => {
      element.setAttribute('aria-label', label);
    });
  });
}

// Helper function to announce dynamic changes
export function announceChange(message: string, priority: 'polite' | 'assertive' = 'polite') {
  const announcement = document.createElement('div');
  announcement.setAttribute('role', 'status');
  announcement.setAttribute('aria-live', priority);
  announcement.className = 'sr-only';
  announcement.textContent = message;
  
  document.body.appendChild(announcement);
  
  setTimeout(() => {
    document.body.removeChild(announcement);
  }, 1000);
}