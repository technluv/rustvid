import React, { memo, useCallback } from 'react';
import { TimelineClip } from './TimelineClip';
import { Track, Clip } from '../../store/timelineStore';

interface TimelineTrackProps {
  track: Track;
  trackIndex: number;
  zoom: number;
  offset: number;
  duration: number;
  onDragStart: (clip: Clip, trackId: string) => void;
  onDragEnd: () => void;
  onDrop: (trackId: string, position: number) => void;
  isDragging: boolean;
  draggedClip: Clip | null;
  isMobile?: boolean;
}

export const TimelineTrack = memo(({
  track,
  trackIndex,
  zoom,
  offset,
  duration,
  onDragStart,
  onDragEnd,
  onDrop,
  isDragging,
  draggedClip,
  isMobile = false,
}: TimelineTrackProps) => {
  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
  }, []);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left + offset;
    const position = x / zoom;
    onDrop(track.id, position);
  }, [track.id, offset, zoom, onDrop]);

  const trackHeight = isMobile 
    ? (track.type === 'video' ? 100 : 80)
    : (track.type === 'video' ? 80 : 60);

  return (
    <div
      className={`relative border-b border-gray-700 transition-colors focus-within:bg-gray-800 ${
        isDragging ? 'bg-gray-800 drag-over' : 'bg-gray-850'
      }`}
      style={{ height: `${trackHeight}px` }}
      onDragOver={handleDragOver}
      onDrop={handleDrop}
      role="row"
      aria-rowindex={trackIndex + 1}
      aria-label={`Track ${trackIndex + 1}: ${track.name}, ${track.type} track, ${track.clips.length} clips`}
    >
      {/* Track Header */}
      <div className={`absolute left-0 top-0 bottom-0 bg-gray-800 border-r border-gray-700 flex items-center px-3 z-10 ${
        isMobile ? 'w-32' : 'w-40'
      }`}>
        <div className={`flex items-center gap-2 ${isMobile ? 'flex-col' : ''}`}>
          <div className="flex items-center gap-1">
            <button 
              className={`w-4 h-4 rounded transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                track.visible ? 'bg-blue-500 hover:bg-blue-600' : 'bg-gray-600 hover:bg-gray-500'
              }`}
              aria-label={`${track.visible ? 'Hide' : 'Show'} track ${track.name}`}
              title={`${track.visible ? 'Hide' : 'Show'} track`}
            />
            <button 
              className={`w-4 h-4 rounded transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                track.locked ? 'bg-red-500 hover:bg-red-600' : 'bg-gray-600 hover:bg-gray-500'
              }`}
              aria-label={`${track.locked ? 'Unlock' : 'Lock'} track ${track.name}`}
              title={`${track.locked ? 'Unlock' : 'Lock'} track`}
            />
          </div>
          <span className={`text-gray-300 truncate ${
            isMobile ? 'text-xs max-w-20' : 'text-sm'
          }`}>
            {track.name}
          </span>
        </div>
      </div>

      {/* Track Content */}
      <div className={`absolute right-0 top-0 bottom-0 overflow-hidden ${
        isMobile ? 'left-32' : 'left-40'
      }`}>
        {track.clips.map((clip, clipIndex) => (
          <TimelineClip
            key={clip.id}
            clip={clip}
            trackId={track.id}
            zoom={zoom}
            offset={offset}
            trackHeight={trackHeight}
            onDragStart={() => onDragStart(clip, track.id)}
            onDragEnd={onDragEnd}
            isSelected={clip.selected}
            isMobile={isMobile}
          />
        ))}
        
        {/* Empty track indicator */}
        {track.clips.length === 0 && (
          <div 
            className="flex items-center justify-center h-full text-gray-500 text-sm opacity-50"
            role="status"
            aria-label="Empty track"
          >
            Drop clips here
          </div>
        )}
      </div>
    </div>
  );
});

TimelineTrack.displayName = 'TimelineTrack';