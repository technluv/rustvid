import React, { useCallback, useEffect, useRef, useState, memo } from 'react';
import { TimelineTrack } from './TimelineTrack';
import { TimelineRuler } from './TimelineRuler';
import { TimelineControls } from './TimelineControls';
import { useTimelineStore } from '../../store/timelineStore';
import { useDragAndDrop } from '../../hooks/useDragAndDrop';
import { useZoomPan } from '../../hooks/useZoomPan';
import { InlineSpinner } from '../LoadingSpinner';

interface TimelineProps {
  className?: string;
}

export const Timeline = memo(({ className = '' }: TimelineProps) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const scrollRef = useRef<HTMLDivElement>(null);
  const [isLoading] = useState(false);
  const [isMobile, setIsMobile] = useState(false);
  
  const {
    tracks,
    zoom,
    offset,
    duration,
    currentTime,
    setZoom,
    setOffset,
    setCurrentTime,
  } = useTimelineStore();

  const { isDragging, draggedClip, handleDragStart, handleDragEnd, handleDrop } = useDragAndDrop();
  const { handleWheel, handlePan } = useZoomPan(zoom, offset, setZoom, setOffset);

  const [viewportWidth, setViewportWidth] = useState(0);

  // Update viewport width and mobile state on resize
  useEffect(() => {
    const updateViewport = () => {
      if (containerRef.current) {
        setViewportWidth(containerRef.current.clientWidth);
        setIsMobile(window.innerWidth < 768);
      }
    };

    updateViewport();
    const resizeObserver = new ResizeObserver(updateViewport);
    if (containerRef.current) {
      resizeObserver.observe(containerRef.current);
    }
    window.addEventListener('resize', updateViewport);
    
    return () => {
      resizeObserver.disconnect();
      window.removeEventListener('resize', updateViewport);
    };
  }, []);

  // Handle timeline click to set current time
  const handleTimelineClick = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
    if (!scrollRef.current) return;
    
    const rect = scrollRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left + offset;
    const time = (x / zoom) * (duration / viewportWidth);
    setCurrentTime(Math.max(0, Math.min(duration, time)));
  }, [offset, zoom, duration, viewportWidth, setCurrentTime]);

  // Calculate timeline width based on zoom
  const timelineWidth = duration * zoom;

  return (
    <div 
      ref={containerRef}
      className={`flex flex-col h-full bg-gray-900 ${className}`}
      onWheel={handleWheel}
      role="region"
      aria-label="Video timeline editor"
      tabIndex={0}
    >
      {/* Timeline Controls */}
      <TimelineControls isMobile={isMobile} />
      
      {/* Timeline Ruler */}
      <div 
        className="sticky top-0 z-20 bg-gray-800 border-b border-gray-700"
        role="toolbar"
        aria-label="Timeline scrubber"
      >
        <TimelineRuler
          duration={duration}
          zoom={zoom}
          offset={offset}
          currentTime={currentTime}
          width={viewportWidth}
          onTimeClick={handleTimelineClick}
          isMobile={isMobile}
        />
      </div>
      
      {/* Timeline Tracks */}
      <div 
        ref={scrollRef}
        className={`flex-1 overflow-x-auto overflow-y-auto scrollbar-thin scrollbar-thumb-gray-600 scrollbar-track-gray-800 ${
          isMobile ? 'touch-pan-x' : ''
        }`}
        onMouseMove={handlePan}
        role="grid"
        aria-label="Timeline tracks"
        aria-rowcount={tracks.length}
        tabIndex={-1}
      >
        <div 
          className="relative"
          style={{ width: `${timelineWidth}px`, minHeight: '100%' }}
        >
          {/* Current Time Indicator */}
          <div
            className="absolute top-0 bottom-0 w-0.5 bg-red-500 z-10 pointer-events-none shadow-lg"
            style={{ left: `${(currentTime / duration) * timelineWidth - offset}px` }}
            role="presentation"
            aria-hidden="true"
          />
          
          {/* Tracks */}
          <div className="relative">
            {isLoading ? (
              <div className="flex items-center justify-center p-8">
                <InlineSpinner message="Loading tracks..." size="md" />
              </div>
            ) : tracks.length === 0 ? (
              <div 
                className="flex items-center justify-center p-8 text-gray-500 text-center"
                role="status"
                aria-live="polite"
              >
                <div>
                  <div className="text-4xl mb-2 opacity-50">🎥</div>
                  <div>No tracks yet</div>
                  <div className="text-sm mt-1 opacity-70">
                    Add video or audio tracks to get started
                  </div>
                </div>
              </div>
            ) : (
              tracks.map((track, index) => (
                <TimelineTrack
                  key={track.id}
                  track={track}
                  trackIndex={index}
                  zoom={zoom}
                  offset={offset}
                  duration={duration}
                  onDragStart={handleDragStart}
                  onDragEnd={handleDragEnd}
                  onDrop={handleDrop}
                  isDragging={isDragging}
                  draggedClip={draggedClip}
                  isMobile={isMobile}
                />
              ))
            )}
          </div>
        </div>
      </div>
      
      {/* Drag Ghost */}
      {isDragging && draggedClip && (
        <div 
          className="fixed pointer-events-none z-50 opacity-75 transform-gpu transition-transform duration-150"
          style={{ transform: 'translate3d(0, 0, 0)' }}
          role="presentation"
          aria-hidden="true"
        >
          <div className="bg-blue-500 rounded px-3 py-2 text-white text-sm shadow-lg border border-blue-400">
            🎥 {draggedClip.name}
          </div>
        </div>
      )}
      
      {/* Mobile scroll indicators */}
      {isMobile && (
        <div 
          className="absolute bottom-4 left-1/2 transform -translate-x-1/2 bg-gray-800 rounded-full px-3 py-1 text-xs text-gray-300 opacity-70"
          role="status"
          aria-live="polite"
        >
          ← Swipe to navigate →
        </div>
      )}
    </div>
  );
});

Timeline.displayName = 'Timeline';