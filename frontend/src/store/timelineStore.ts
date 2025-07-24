import { create } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';

export interface Clip {
  id: string;
  name: string;
  type: 'video' | 'audio';
  startTime: number;
  duration: number;
  sourceIn: number;
  sourceOut: number;
  selected: boolean;
  locked: boolean;
  thumbnailUrl?: string;
  waveformData?: number[];
}

export interface Track {
  id: string;
  name: string;
  type: 'video' | 'audio';
  clips: Clip[];
  visible: boolean;
  locked: boolean;
  muted: boolean;
  solo: boolean;
}

interface TimelineState {
  // Timeline data
  tracks: Track[];
  duration: number;
  
  // Playback state
  currentTime: number;
  isPlaying: boolean;
  playbackRate: number;
  
  // View state
  zoom: number;
  offset: number;
  selectedClipIds: string[];
  
  // Actions
  addTrack: (type: 'video' | 'audio', name: string) => void;
  removeTrack: (trackId: string) => void;
  
  addClip: (trackId: string, clip: Omit<Clip, 'id'>) => void;
  removeClip: (trackId: string, clipId: string) => void;
  moveClip: (sourceTrackId: string, targetTrackId: string, clipId: string, newStartTime: number) => void;
  updateClip: (trackId: string, clipId: string, updates: Partial<Clip>) => void;
  
  selectClip: (clipId: string, addToSelection?: boolean) => void;
  deselectAll: () => void;
  
  play: () => void;
  pause: () => void;
  stop: () => void;
  seekTo: (time: number) => void;
  setCurrentTime: (time: number) => void;
  
  setZoom: (zoom: number) => void;
  setOffset: (offset: number) => void;
  
  updateTrack: (trackId: string, updates: Partial<Track>) => void;
  reorderTracks: (fromIndex: number, toIndex: number) => void;
}

export const useTimelineStore = create<TimelineState>()(
  devtools(
    subscribeWithSelector(
      immer((set, get) => ({
        // Initial state
        tracks: [],
        duration: 300, // 5 minutes default
        
        currentTime: 0,
        isPlaying: false,
        playbackRate: 1,
        
        zoom: 50,
        offset: 0,
        selectedClipIds: [],
        
        // Track actions
        addTrack: (type, name) => set((state) => {
          const newTrack: Track = {
            id: `track-${Date.now()}`,
            name,
            type,
            clips: [],
            visible: true,
            locked: false,
            muted: false,
            solo: false,
          };
          state.tracks.push(newTrack);
        }),
        
        removeTrack: (trackId) => set((state) => {
          state.tracks = state.tracks.filter(track => track.id !== trackId);
        }),
        
        // Clip actions
        addClip: (trackId, clipData) => set((state) => {
          const track = state.tracks.find(t => t.id === trackId);
          if (track) {
            const newClip: Clip = {
              ...clipData,
              id: `clip-${Date.now()}`,
            };
            track.clips.push(newClip);
            
            // Update timeline duration if needed
            const clipEnd = newClip.startTime + newClip.duration;
            if (clipEnd > state.duration) {
              state.duration = clipEnd + 60; // Add 1 minute buffer
            }
          }
        }),
        
        removeClip: (trackId, clipId) => set((state) => {
          const track = state.tracks.find(t => t.id === trackId);
          if (track) {
            track.clips = track.clips.filter(clip => clip.id !== clipId);
          }
        }),
        
        moveClip: (sourceTrackId, targetTrackId, clipId, newStartTime) => set((state) => {
          const sourceTrack = state.tracks.find(t => t.id === sourceTrackId);
          const targetTrack = state.tracks.find(t => t.id === targetTrackId);
          
          if (sourceTrack && targetTrack) {
            const clipIndex = sourceTrack.clips.findIndex(c => c.id === clipId);
            if (clipIndex !== -1) {
              const [clip] = sourceTrack.clips.splice(clipIndex, 1);
              clip.startTime = newStartTime;
              targetTrack.clips.push(clip);
              
              // Sort clips by start time
              targetTrack.clips.sort((a, b) => a.startTime - b.startTime);
            }
          }
        }),
        
        updateClip: (trackId, clipId, updates) => set((state) => {
          const track = state.tracks.find(t => t.id === trackId);
          if (track) {
            const clip = track.clips.find(c => c.id === clipId);
            if (clip) {
              Object.assign(clip, updates);
            }
          }
        }),
        
        // Selection actions
        selectClip: (clipId, addToSelection = false) => set((state) => {
          if (addToSelection) {
            if (!state.selectedClipIds.includes(clipId)) {
              state.selectedClipIds.push(clipId);
            }
          } else {
            state.selectedClipIds = [clipId];
          }
          
          // Update clip selected state
          state.tracks.forEach(track => {
            track.clips.forEach(clip => {
              clip.selected = state.selectedClipIds.includes(clip.id);
            });
          });
        }),
        
        deselectAll: () => set((state) => {
          state.selectedClipIds = [];
          state.tracks.forEach(track => {
            track.clips.forEach(clip => {
              clip.selected = false;
            });
          });
        }),
        
        // Playback actions
        play: () => set({ isPlaying: true }),
        pause: () => set({ isPlaying: false }),
        stop: () => set({ isPlaying: false, currentTime: 0 }),
        seekTo: (time) => set({ currentTime: Math.max(0, Math.min(time, get().duration)) }),
        setCurrentTime: (time) => set({ currentTime: Math.max(0, Math.min(time, get().duration)) }),
        
        // View actions
        setZoom: (zoom) => set({ zoom: Math.max(10, Math.min(200, zoom)) }),
        setOffset: (offset) => set({ offset: Math.max(0, offset) }),
        
        // Track management
        updateTrack: (trackId, updates) => set((state) => {
          const track = state.tracks.find(t => t.id === trackId);
          if (track) {
            Object.assign(track, updates);
          }
        }),
        
        reorderTracks: (fromIndex, toIndex) => set((state) => {
          const [removed] = state.tracks.splice(fromIndex, 1);
          state.tracks.splice(toIndex, 0, removed);
        }),
      }))
    ),
    {
      name: 'timeline-store',
    }
  )
);

// Playback timer
let playbackInterval: number | null = null;

useTimelineStore.subscribe(
  (state) => state.isPlaying,
  (isPlaying) => {
    if (isPlaying) {
      playbackInterval = setInterval(() => {
        const { currentTime, duration, playbackRate } = useTimelineStore.getState();
        const newTime = currentTime + (0.033 * playbackRate); // ~30fps
        
        if (newTime >= duration) {
          useTimelineStore.getState().stop();
        } else {
          useTimelineStore.getState().seekTo(newTime);
        }
      }, 33);
    } else if (playbackInterval) {
      clearInterval(playbackInterval);
      playbackInterval = null;
    }
  }
);