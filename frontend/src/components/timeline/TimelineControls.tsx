import React, { memo, useCallback } from 'react';
import { useTimelineStore } from '../../store/timelineStore';
import { 
  PlayIcon, 
  PauseIcon, 
  Square as StopIcon, 
  SkipBackIcon, 
  SkipForwardIcon,
  ZoomInIcon,
  ZoomOutIcon,
  RefreshCwIcon,
} from 'lucide-react';

interface TimelineControlsProps {
  isMobile?: boolean;
}

export const TimelineControls = memo(({ isMobile = false }: TimelineControlsProps) => {
  const {
    isPlaying,
    currentTime,
    duration,
    zoom,
    play,
    pause,
    stop,
    seekTo,
    setZoom,
    addTrack,
  } = useTimelineStore();

  const handlePlayPause = useCallback(() => {
    if (isPlaying) {
      pause();
    } else {
      play();
    }
  }, [isPlaying, play, pause]);

  const handleStop = useCallback(() => {
    stop();
    seekTo(0);
  }, [stop, seekTo]);

  const handleSkipBack = useCallback(() => {
    seekTo(Math.max(0, currentTime - 5));
  }, [currentTime, seekTo]);

  const handleSkipForward = useCallback(() => {
    seekTo(Math.min(duration, currentTime + 5));
  }, [currentTime, duration, seekTo]);

  const handleZoomIn = useCallback(() => {
    setZoom(Math.min(200, zoom * 1.2));
  }, [zoom, setZoom]);

  const handleZoomOut = useCallback(() => {
    setZoom(Math.max(10, zoom / 1.2));
  }, [zoom, setZoom]);

  const handleZoomReset = useCallback(() => {
    setZoom(50);
  }, [setZoom]);

  const handleAddVideoTrack = useCallback(() => {
    addTrack('video', 'Video Track');
  }, [addTrack]);

  const handleAddAudioTrack = useCallback(() => {
    addTrack('audio', 'Audio Track');
  }, [addTrack]);

  return (
    <div 
      className={`flex items-center justify-between p-2 bg-gray-800 border-b border-gray-700 ${
        isMobile ? 'flex-wrap gap-2' : ''
      }`}
      role="toolbar"
      aria-label="Timeline playback and editing controls"
    >
      {/* Playback Controls */}
      <div 
        className={`flex items-center gap-2 ${isMobile ? 'flex-wrap' : ''}`}
        role="group"
        aria-label="Playback controls"
      >
        <button
          onClick={handleSkipBack}
          className="p-2 rounded hover:bg-gray-700 focus:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
          title="Skip Back 5s"
          aria-label="Skip back 5 seconds"
        >
          <SkipBackIcon className="w-4 h-4 text-gray-300" aria-hidden="true" />
        </button>
        
        <button
          onClick={handlePlayPause}
          className="p-2 rounded bg-blue-600 hover:bg-blue-700 focus:bg-blue-700 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-400 transform hover:scale-105 active:scale-95"
          title={isPlaying ? 'Pause (Space)' : 'Play (Space)'}
          aria-label={isPlaying ? 'Pause playback' : 'Start playback'}
          aria-pressed={isPlaying}
        >
          {isPlaying ? (
            <PauseIcon className="w-4 h-4 text-white" aria-hidden="true" />
          ) : (
            <PlayIcon className="w-4 h-4 text-white" aria-hidden="true" />
          )}
        </button>
        
        <button
          onClick={handleStop}
          className="p-2 rounded hover:bg-gray-700 focus:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
          title="Stop and reset to beginning"
          aria-label="Stop playback and reset to beginning"
        >
          <StopIcon className="w-4 h-4 text-gray-300" aria-hidden="true" />
        </button>
        
        <button
          onClick={handleSkipForward}
          className="p-2 rounded hover:bg-gray-700 focus:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
          title="Skip Forward 5s"
          aria-label="Skip forward 5 seconds"
        >
          <SkipForwardIcon className="w-4 h-4 text-gray-300" aria-hidden="true" />
        </button>
        
        {/* Time Display */}
        <div 
          className={`${isMobile ? 'ml-2' : 'ml-4'} text-sm text-gray-300 font-mono`}
          role="timer"
          aria-live="polite"
          aria-label={`Current time: ${formatTime(currentTime)} of ${formatTime(duration)}`}
        >
          <span aria-hidden="true">{formatTime(currentTime)} / {formatTime(duration)}</span>
        </div>
      </div>

      {/* Track Controls */}
      <div 
        className={`flex items-center gap-2 ${isMobile ? 'order-last w-full justify-center mt-2' : ''}`}
        role="group"
        aria-label="Track management"
      >
        <button
          onClick={handleAddVideoTrack}
          className="px-3 py-1 text-sm rounded bg-blue-600 hover:bg-blue-700 focus:bg-blue-700 transition-all duration-200 text-white focus:outline-none focus:ring-2 focus:ring-blue-400 transform hover:scale-105 active:scale-95"
          aria-label="Add new video track to timeline"
        >
          {isMobile ? '+ Video' : 'Add Video Track'}
        </button>
        
        <button
          onClick={handleAddAudioTrack}
          className="px-3 py-1 text-sm rounded bg-green-600 hover:bg-green-700 focus:bg-green-700 transition-all duration-200 text-white focus:outline-none focus:ring-2 focus:ring-green-400 transform hover:scale-105 active:scale-95"
          aria-label="Add new audio track to timeline"
        >
          {isMobile ? '+ Audio' : 'Add Audio Track'}
        </button>
      </div>

      {/* Zoom Controls */}
      <div 
        className="flex items-center gap-2"
        role="group"
        aria-label="Zoom controls"
      >
        <button
          onClick={handleZoomOut}
          className="p-2 rounded hover:bg-gray-700 focus:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
          title="Zoom Out"
          aria-label="Zoom out timeline view"
          disabled={zoom <= 10}
        >
          <ZoomOutIcon className="w-4 h-4 text-gray-300" aria-hidden="true" />
        </button>
        
        <button
          onClick={handleZoomReset}
          className="p-2 rounded hover:bg-gray-700 focus:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
          title="Reset Zoom to Default"
          aria-label="Reset zoom to default level"
        >
          <RefreshCwIcon className="w-4 h-4 text-gray-300" aria-hidden="true" />
        </button>
        
        <button
          onClick={handleZoomIn}
          className="p-2 rounded hover:bg-gray-700 focus:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
          title="Zoom In"
          aria-label="Zoom in timeline view"
          disabled={zoom >= 200}
        >
          <ZoomInIcon className="w-4 h-4 text-gray-300" aria-hidden="true" />
        </button>
        
        <span 
          className={`${isMobile ? 'ml-1' : 'ml-2'} text-sm text-gray-400 min-w-[3rem] text-center font-mono`}
          role="status"
          aria-label={`Current zoom level: ${Math.round(zoom)} percent`}
        >
          <span aria-hidden="true">{Math.round(zoom)}%</span>
        </span>
      </div>
    </div>
  );
});

TimelineControls.displayName = 'TimelineControls';

// Helper function to format time
function formatTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  
  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  return `${minutes}:${secs.toString().padStart(2, '0')}`;
}