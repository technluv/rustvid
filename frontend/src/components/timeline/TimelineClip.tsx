import React, { memo, useCallback, useMemo, useRef, useState } from 'react';
import { Clip } from '../../store/timelineStore';

interface TimelineClipProps {
  clip: Clip;
  trackId: string;
  zoom: number;
  offset: number;
  trackHeight: number;
  onDragStart: () => void;
  onDragEnd: () => void;
  isSelected: boolean;
  isMobile?: boolean;
}

export const TimelineClip = memo(({
  clip,
  // trackId,
  zoom,
  offset,
  trackHeight,
  onDragStart,
  onDragEnd,
  isSelected,
  isMobile = false,
}: TimelineClipProps) => {
  const clipRef = useRef<HTMLDivElement>(null);
  const [, /*setIsResizing*/] = useState<'start' | 'end' | null>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [isFocused, setIsFocused] = useState(false);
  const [isHovered, setIsHovered] = useState(false);

  // Calculate clip position and dimensions
  const clipStyle = useMemo(() => {
    const left = clip.startTime * zoom - offset;
    const width = clip.duration * zoom;
    
    return {
      left: `${left}px`,
      width: `${width}px`,
      height: `${trackHeight - 8}px`,
      top: '4px',
    };
  }, [clip.startTime, clip.duration, zoom, offset, trackHeight]);

  // Handle drag start
  const handleDragStart = useCallback((e: React.DragEvent) => {
    e.dataTransfer.effectAllowed = 'move';
    e.dataTransfer.setData('text/plain', ''); // Required for Firefox
    setIsDragging(true);
    onDragStart();
  }, [onDragStart]);

  // Handle drag end
  const handleDragEnd = useCallback(() => {
    setIsDragging(false);
    onDragEnd();
  }, [onDragEnd]);

  // Handle resize start
  const handleResizeStart = useCallback((e: React.MouseEvent, side: 'start' | 'end') => {
    e.preventDefault();
    e.stopPropagation();
    console.log('Resize', side);
    // setIsResizing(side);
  }, []);

  // Handle keyboard interactions
  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    switch (e.key) {
      case 'Enter':
      case ' ':
        e.preventDefault();
        // Toggle selection or trigger edit mode
        console.log('Clip activated:', clip.name);
        break;
      case 'Delete':
      case 'Backspace':
        e.preventDefault();
        console.log('Delete clip:', clip.name);
        break;
      case 'Escape':
        e.preventDefault();
        clipRef.current?.blur();
        break;
    }
  }, [clip.name]);

  // Handle focus events
  const handleFocus = useCallback(() => {
    setIsFocused(true);
  }, []);

  const handleBlur = useCallback(() => {
    setIsFocused(false);
  }, []);

  const handleMouseEnter = useCallback(() => {
    setIsHovered(true);
  }, []);

  const handleMouseLeave = useCallback(() => {
    setIsHovered(false);
  }, []);

  // Get thumbnail style based on clip type
  const thumbnailStyle = useMemo(() => {
    if (clip.type === 'video' && clip.thumbnailUrl) {
      return { backgroundImage: `url(${clip.thumbnailUrl})` };
    }
    return {};
  }, [clip.type, clip.thumbnailUrl]);

  // Generate waveform for audio clips
  const waveform = useMemo(() => {
    if (clip.type === 'audio' && clip.waveformData) {
      return clip.waveformData.map((value, index) => (
        <div
          key={index}
          className="inline-block w-px bg-green-400 opacity-70 waveform-bar"
          style={{ 
            height: `${value * (trackHeight - 16)}px`,
            animationDelay: `${index * 0.05}s`
          }}
          aria-hidden="true"
        />
      ));
    }
    return null;
  }, [clip.type, clip.waveformData, trackHeight]);

  const clipTypeIcon = clip.type === 'video' ? '🎥' : '🎵';

  return (
    <div
      ref={clipRef}
      className={`timeline-clip absolute rounded cursor-move transition-all duration-200 focus:outline-none ${
        isSelected ? 'ring-2 ring-blue-500 ring-offset-1 ring-offset-gray-900' : ''
      } ${isDragging ? 'opacity-50 drag-preview' : ''} ${isHovered ? 'shadow-lg transform -translate-y-0.5' : ''} ${
        isFocused ? 'ring-2 ring-blue-400 ring-offset-1 ring-offset-gray-900' : ''
      } ${
        clip.type === 'video' ? 'bg-blue-600 hover:bg-blue-700' : 'bg-green-600 hover:bg-green-700'
      } ${isMobile ? 'min-h-[48px]' : ''}`}
      style={clipStyle}
      draggable
      tabIndex={0}
      role="button"
      aria-label={`${clip.type} clip: ${clip.name}, duration: ${formatDuration(clip.duration)}, starts at ${formatDuration(clip.startTime)}`}
      aria-selected={isSelected}
      aria-describedby={`clip-details-${clip.id}`}
      onDragStart={handleDragStart}
      onDragEnd={handleDragEnd}
      onKeyDown={handleKeyDown}
      onFocus={handleFocus}
      onBlur={handleBlur}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {/* Resize Handles */}
      {!isMobile && (
        <>
          <div
            className="absolute left-0 top-0 bottom-0 w-2 cursor-ew-resize hover:bg-white hover:bg-opacity-40 transition-all duration-200 focus:bg-white focus:bg-opacity-50 focus:outline-none"
            role="button"
            aria-label="Resize clip start time"
            tabIndex={-1}
            onMouseDown={(e) => handleResizeStart(e, 'start')}
          />
          <div
            className="absolute right-0 top-0 bottom-0 w-2 cursor-ew-resize hover:bg-white hover:bg-opacity-40 transition-all duration-200 focus:bg-white focus:bg-opacity-50 focus:outline-none"
            role="button"
            aria-label="Resize clip end time"
            tabIndex={-1}
            onMouseDown={(e) => handleResizeStart(e, 'end')}
          />
        </>
      )}

      {/* Clip Content */}
      <div className="relative h-full overflow-hidden rounded px-2">
        {/* Video Thumbnail */}
        {clip.type === 'video' && (
          <div
            className="absolute inset-0 bg-cover bg-center opacity-30 transition-opacity duration-200"
            style={thumbnailStyle}
            aria-hidden="true"
          />
        )}

        {/* Audio Waveform */}
        {clip.type === 'audio' && (
          <div 
            className="absolute inset-0 flex items-center justify-center"
            role="img"
            aria-label="Audio waveform visualization"
          >
            {waveform}
          </div>
        )}

        {/* Clip Name */}
        <div className="relative z-10 flex items-center h-full px-1">
          <span className="text-xs mr-1" aria-hidden="true">{clipTypeIcon}</span>
          <span className="text-xs text-white truncate font-medium flex-1">
            {clip.name}
          </span>
        </div>

        {/* Duration Indicator */}
        <div 
          className="absolute bottom-1 right-2 text-xs text-white opacity-70 bg-black bg-opacity-30 px-1 rounded"
          aria-hidden="true"
        >
          {formatDuration(clip.duration)}
        </div>
      </div>
      
      {/* Hidden description for screen readers */}
      <div id={`clip-details-${clip.id}`} className="sr-only">
        {clip.type === 'video' ? 'Video' : 'Audio'} clip titled "{clip.name}". 
        Duration: {formatDuration(clip.duration)}. 
        Starts at: {formatDuration(clip.startTime)}. 
        {isSelected ? 'Currently selected.' : 'Not selected.'}
        {clip.locked ? 'This clip is locked.' : 'This clip can be edited.'}
        Press Enter to edit, Delete to remove, or use arrow keys to navigate.
      </div>
    </div>
  );
});

TimelineClip.displayName = 'TimelineClip';

// Helper function to format duration
function formatDuration(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}