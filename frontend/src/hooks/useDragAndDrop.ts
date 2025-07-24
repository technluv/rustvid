import { useState, useCallback, useEffect } from 'react';
import { useTimelineStore, Clip } from '../store/timelineStore';

export function useDragAndDrop() {
  const [isDragging, setIsDragging] = useState(false);
  const [draggedClip, setDraggedClip] = useState<Clip | null>(null);
  const [sourceTrackId, setSourceTrackId] = useState<string | null>(null);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });
  
  const { moveClip, updateClip } = useTimelineStore();

  // Handle drag start
  const handleDragStart = useCallback((clip: Clip, trackId: string) => {
    setIsDragging(true);
    setDraggedClip(clip);
    setSourceTrackId(trackId);
  }, []);

  // Handle drag end
  const handleDragEnd = useCallback(() => {
    setIsDragging(false);
    setDraggedClip(null);
    setSourceTrackId(null);
    setDragOffset({ x: 0, y: 0 });
  }, []);

  // Handle drop
  const handleDrop = useCallback((targetTrackId: string, position: number) => {
    if (!draggedClip || !sourceTrackId) return;

    // Calculate snap position (snap to nearest frame at 30fps)
    const frameTime = 1 / 30;
    const snappedPosition = Math.round(position / frameTime) * frameTime;

    // Check for overlaps
    const targetTrack = useTimelineStore.getState().tracks.find(t => t.id === targetTrackId);
    if (targetTrack) {
      const hasOverlap = targetTrack.clips.some(clip => {
        if (clip.id === draggedClip.id) return false;
        const clipEnd = clip.startTime + clip.duration;
        const draggedEnd = snappedPosition + draggedClip.duration;
        return (
          (snappedPosition >= clip.startTime && snappedPosition < clipEnd) ||
          (draggedEnd > clip.startTime && draggedEnd <= clipEnd) ||
          (snappedPosition <= clip.startTime && draggedEnd >= clipEnd)
        );
      });

      if (!hasOverlap) {
        moveClip(sourceTrackId, targetTrackId, draggedClip.id, snappedPosition);
      }
    }

    handleDragEnd();
  }, [draggedClip, sourceTrackId, moveClip, handleDragEnd]);

  // Handle keyboard shortcuts for nudging
  useEffect(() => {
    if (!draggedClip || !sourceTrackId) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      const nudgeAmount = e.shiftKey ? 1 : 1 / 30; // Shift for 1 second, normal for 1 frame
      
      switch (e.key) {
        case 'ArrowLeft':
          e.preventDefault();
          updateClip(sourceTrackId, draggedClip.id, {
            startTime: Math.max(0, draggedClip.startTime - nudgeAmount)
          });
          break;
        case 'ArrowRight':
          e.preventDefault();
          updateClip(sourceTrackId, draggedClip.id, {
            startTime: draggedClip.startTime + nudgeAmount
          });
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [draggedClip, sourceTrackId, updateClip]);

  return {
    isDragging,
    draggedClip,
    dragOffset,
    handleDragStart,
    handleDragEnd,
    handleDrop,
  };
}