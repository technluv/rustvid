import React, { memo, useMemo } from 'react';

interface TimelineRulerProps {
  duration: number;
  zoom: number;
  offset: number;
  currentTime: number;
  width: number;
  onTimeClick: (e: React.MouseEvent) => void;
}

export const TimelineRuler = memo(({
  duration,
  zoom,
  offset,
  currentTime,
  width,
  onTimeClick,
}: TimelineRulerProps) => {
  // Calculate time markers based on zoom level
  const markers = useMemo(() => {
    const pixelsPerSecond = zoom;
    const interval = getTimeInterval(pixelsPerSecond);
    const markers: Array<{ time: number; label: string; major: boolean }> = [];
    
    const startTime = Math.max(0, offset / zoom);
    const endTime = Math.min(duration, (offset + width) / zoom);
    
    for (let time = Math.floor(startTime / interval) * interval; time <= endTime; time += interval) {
      if (time >= 0) {
        markers.push({
          time,
          label: formatTime(time),
          major: time % (interval * 5) === 0,
        });
      }
    }
    
    return markers;
  }, [duration, zoom, offset, width]);

  return (
    <div 
      className="relative h-8 bg-gray-800 cursor-pointer select-none"
      onClick={onTimeClick}
    >
      {/* Time Markers */}
      {markers.map(({ time, label, major }) => {
        const x = time * zoom - offset;
        return (
          <div
            key={time}
            className="absolute top-0"
            style={{ left: `${x}px` }}
          >
            <div className={`w-px bg-gray-600 ${major ? 'h-full' : 'h-4'}`} />
            {major && (
              <span className="absolute top-1 left-1 text-xs text-gray-400 whitespace-nowrap">
                {label}
              </span>
            )}
          </div>
        );
      })}
      
      {/* Current Time Marker */}
      <div
        className="absolute top-0 h-full w-0.5 bg-red-500"
        style={{ left: `${currentTime * zoom - offset}px` }}
      >
        <div className="absolute -top-1 -left-3 w-0 h-0 border-l-[6px] border-l-transparent border-r-[6px] border-r-transparent border-t-[6px] border-t-red-500" />
      </div>
      
      {/* Screen reader announcements */}
      <div className="sr-only" aria-live="polite">
        Timeline position: {formatTime(currentTime)} of {formatTime(duration)}
      </div>
    </div>
  );
});

TimelineRuler.displayName = 'TimelineRuler';

// Helper functions
function getTimeInterval(pixelsPerSecond: number): number {
  if (pixelsPerSecond > 100) return 0.1;
  if (pixelsPerSecond > 50) return 0.5;
  if (pixelsPerSecond > 20) return 1;
  if (pixelsPerSecond > 10) return 5;
  if (pixelsPerSecond > 5) return 10;
  return 30;
}

function formatTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  const frames = Math.floor((seconds % 1) * 30); // Assuming 30fps
  
  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  return `${minutes}:${secs.toString().padStart(2, '0')}:${frames.toString().padStart(2, '0')}`;
}