import React, { useRef, useEffect } from 'react';
import './VideoTimeline.css';

interface VideoTimelineProps {
  zoomLevel: number;
  currentTime: number;
}

const VideoTimeline: React.FC<VideoTimelineProps> = ({ zoomLevel, currentTime }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const timelineRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    drawTimeline();
  }, [zoomLevel, currentTime]);

  const drawTimeline = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Draw timeline ruler
    ctx.strokeStyle = '#444';
    ctx.fillStyle = '#888';
    ctx.font = '12px Arial';

    const pixelsPerSecond = 50 * zoomLevel;
    const totalSeconds = canvas.width / pixelsPerSecond;
    
    // Draw time markers
    for (let i = 0; i <= totalSeconds; i++) {
      const x = i * pixelsPerSecond;
      
      // Major tick every 5 seconds
      if (i % 5 === 0) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, 20);
        ctx.stroke();
        
        // Time label
        const minutes = Math.floor(i / 60);
        const seconds = i % 60;
        const timeStr = `${minutes}:${seconds.toString().padStart(2, '0')}`;
        ctx.fillText(timeStr, x - 15, 35);
      } else {
        // Minor tick
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, 10);
        ctx.stroke();
      }
    }

    // Draw playhead
    const playheadX = (currentTime / 1000) * pixelsPerSecond;
    ctx.strokeStyle = '#ff0000';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(playheadX, 0);
    ctx.lineTo(playheadX, canvas.height);
    ctx.stroke();
  };

  return (
    <div className="video-timeline" ref={timelineRef}>
      <div className="timeline-header">
        <canvas
          ref={canvasRef}
          width={1200}
          height={50}
          className="timeline-ruler"
        />
      </div>
      <div className="timeline-tracks">
        <div className="track video-track">
          <div className="track-label">Video</div>
          <div className="track-content">
            {/* Video clips will be rendered here */}
            <div className="placeholder-clip" style={{ width: `${200 * zoomLevel}px` }}>
              Video Clip 1
            </div>
          </div>
        </div>
        <div className="track audio-track">
          <div className="track-label">Audio</div>
          <div className="track-content">
            {/* Audio clips will be rendered here */}
            <div className="placeholder-clip audio" style={{ width: `${150 * zoomLevel}px` }}>
              Audio Clip 1
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default VideoTimeline;