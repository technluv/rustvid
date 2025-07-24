import { useCallback, useRef, useState } from 'react';

export function useZoomPan(
  zoom: number,
  offset: number,
  setZoom: (zoom: number) => void,
  setOffset: (offset: number) => void
) {
  const [isPanning, setIsPanning] = useState(false);
  const panStartRef = useRef({ x: 0, offset: 0 });

  // Handle mouse wheel for zooming
  const handleWheel = useCallback((e: React.WheelEvent) => {
    if (e.ctrlKey || e.metaKey) {
      e.preventDefault();
      
      // Calculate zoom factor
      const zoomFactor = e.deltaY > 0 ? 0.9 : 1.1;
      const newZoom = Math.max(10, Math.min(200, zoom * zoomFactor));
      
      // Adjust offset to keep mouse position stable
      const mouseX = e.clientX - (e.currentTarget as HTMLElement).getBoundingClientRect().left;
      const timelineX = mouseX + offset;
      const newTimelineX = timelineX * (newZoom / zoom);
      const newOffset = newTimelineX - mouseX;
      
      setZoom(newZoom);
      setOffset(Math.max(0, newOffset));
    } else {
      // Horizontal scroll
      const scrollAmount = e.deltaY;
      setOffset(Math.max(0, offset + scrollAmount));
    }
  }, [zoom, offset, setZoom, setOffset]);

  // Handle panning with middle mouse button
  const handlePan = useCallback((e: React.MouseEvent) => {
    if (e.buttons === 4) { // Middle mouse button
      if (!isPanning) {
        setIsPanning(true);
        panStartRef.current = { x: e.clientX, offset };
      } else {
        const deltaX = panStartRef.current.x - e.clientX;
        setOffset(Math.max(0, panStartRef.current.offset + deltaX));
      }
    } else if (isPanning) {
      setIsPanning(false);
    }
  }, [isPanning, offset, setOffset]);

  // Keyboard shortcuts for zoom
  const handleKeyboardZoom = useCallback((e: KeyboardEvent) => {
    if (e.ctrlKey || e.metaKey) {
      switch (e.key) {
        case '+':
        case '=':
          e.preventDefault();
          setZoom(Math.min(200, zoom * 1.2));
          break;
        case '-':
          e.preventDefault();
          setZoom(Math.max(10, zoom / 1.2));
          break;
        case '0':
          e.preventDefault();
          setZoom(50); // Reset zoom
          setOffset(0);
          break;
      }
    }
  }, [zoom, setZoom, setOffset]);

  // Fit timeline to view
  const fitToView = useCallback((containerWidth: number, duration: number) => {
    const newZoom = containerWidth / duration;
    setZoom(Math.max(10, Math.min(200, newZoom * 0.95))); // 95% to add some padding
    setOffset(0);
  }, [setZoom, setOffset]);

  return {
    isPanning,
    handleWheel,
    handlePan,
    handleKeyboardZoom,
    fitToView,
  };
}