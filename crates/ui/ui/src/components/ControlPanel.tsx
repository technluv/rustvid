import React from 'react';
import './ControlPanel.css';

interface ControlPanelProps {
  isPlaying: boolean;
  onTogglePlayback: () => void;
  currentTime: number;
  zoomLevel: number;
  onZoomChange: (zoom: number) => void;
}

const ControlPanel: React.FC<ControlPanelProps> = ({
  isPlaying,
  onTogglePlayback,
  currentTime,
  zoomLevel,
  onZoomChange,
}) => {
  const formatTime = (timeMs: number) => {
    const totalSeconds = Math.floor(timeMs / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    const ms = Math.floor((timeMs % 1000) / 10);
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${ms.toString().padStart(2, '0')}`;
  };

  return (
    <div className="control-panel">
      <div className="playback-controls">
        <button className="control-button" onClick={onTogglePlayback}>
          {isPlaying ? '⏸️ Pause' : '▶️ Play'}
        </button>
        <button className="control-button">⏹️ Stop</button>
        <button className="control-button">⏮️ Previous</button>
        <button className="control-button">⏭️ Next</button>
      </div>

      <div className="time-display">
        <span className="current-time">{formatTime(currentTime)}</span>
      </div>

      <div className="zoom-controls">
        <label htmlFor="zoom-slider">Zoom:</label>
        <input
          id="zoom-slider"
          type="range"
          min="0.5"
          max="3"
          step="0.1"
          value={zoomLevel}
          onChange={(e) => onZoomChange(parseFloat(e.target.value))}
        />
        <span className="zoom-value">{(zoomLevel * 100).toFixed(0)}%</span>
      </div>
    </div>
  );
};

export default ControlPanel;