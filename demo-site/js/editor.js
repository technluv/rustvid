// Video Editor JavaScript

let currentVideo = null;
let isPlaying = false;
let currentZoom = 100;
let selectedClip = null;

// Initialize editor
document.addEventListener('DOMContentLoaded', () => {
    initializeEditor();
    setupDragAndDrop();
    setupKeyboardShortcuts();
    generateTimelineRuler();
});

function initializeEditor() {
    console.log('Video Editor initialized');
    
    // Set up video player events
    const videoPlayer = document.getElementById('video-player');
    if (videoPlayer) {
        videoPlayer.addEventListener('timeupdate', updateTimeDisplay);
        videoPlayer.addEventListener('loadedmetadata', onVideoLoaded);
    }
}

// File Upload Handling
function handleFileUpload(event) {
    const files = event.target.files;
    if (files.length > 0) {
        showLoading();
        
        // Simulate processing
        setTimeout(() => {
            for (let file of files) {
                processFile(file);
            }
            hideLoading();
        }, 1000);
    }
}

function processFile(file) {
    console.log('Processing file:', file.name);
    
    if (file.type.startsWith('video/')) {
        loadVideo(file);
        addToTimeline(file, 'video');
    } else if (file.type.startsWith('audio/')) {
        addToTimeline(file, 'audio');
    } else if (file.type.startsWith('image/')) {
        addToTimeline(file, 'video');
    } else {
        showError('Unsupported file type: ' + file.type);
    }
}

function loadVideo(file) {
    const videoPlayer = document.getElementById('video-player');
    const url = URL.createObjectURL(file);
    videoPlayer.src = url;
    currentVideo = file;
    
    // Show success message
    showNotification('Video loaded successfully');
}

// Drag and Drop
function setupDragAndDrop() {
    const uploadArea = document.getElementById('upload-area');
    
    uploadArea.addEventListener('dragover', (e) => {
        e.preventDefault();
        uploadArea.classList.add('drag-over');
    });
    
    uploadArea.addEventListener('dragleave', () => {
        uploadArea.classList.remove('drag-over');
    });
    
    uploadArea.addEventListener('drop', (e) => {
        e.preventDefault();
        uploadArea.classList.remove('drag-over');
        
        const files = e.dataTransfer.files;
        if (files.length > 0) {
            document.getElementById('video-upload').files = files;
            handleFileUpload({ target: { files } });
        }
    });
}

// Timeline Functions
function addToTimeline(file, trackType) {
    const track = document.querySelector(`[data-track="${trackType}"] .track-content`);
    if (!track) return;
    
    const clip = document.createElement('div');
    clip.className = 'timeline-clip';
    clip.innerHTML = `
        <div class="clip-content">
            <span class="clip-name">${file.name}</span>
        </div>
    `;
    clip.style.width = '200px'; // Default width
    clip.style.left = '0px';
    clip.onclick = () => selectClip(clip);
    
    track.appendChild(clip);
}

function selectClip(clip) {
    // Remove previous selection
    document.querySelectorAll('.timeline-clip').forEach(c => c.classList.remove('selected'));
    
    // Select new clip
    clip.classList.add('selected');
    selectedClip = clip;
    
    // Update properties panel
    updatePropertiesPanel(clip);
}

function generateTimelineRuler() {
    const ruler = document.getElementById('timeline-ruler');
    const duration = 300; // 5 minutes in seconds
    
    for (let i = 0; i <= duration; i += 10) {
        const marker = document.createElement('div');
        marker.className = 'time-marker';
        marker.style.left = (i * 10) + 'px';
        marker.innerHTML = `<span>${formatTime(i)}</span>`;
        ruler.appendChild(marker);
    }
}

// Playback Controls
function togglePlayback() {
    const videoPlayer = document.getElementById('video-player');
    const playIcon = document.getElementById('play-icon');
    
    if (isPlaying) {
        videoPlayer.pause();
        playIcon.innerHTML = '<path d="M6 4l12 8-12 8z" fill="currentColor"/>';
    } else {
        videoPlayer.play();
        playIcon.innerHTML = '<path d="M6 6h4v12H6zM14 6h4v12h-4z" fill="currentColor"/>';
    }
    
    isPlaying = !isPlaying;
}

function updateTimeDisplay() {
    const videoPlayer = document.getElementById('video-player');
    const currentTimeEl = document.getElementById('current-time');
    const durationEl = document.getElementById('duration');
    
    currentTimeEl.textContent = formatTime(videoPlayer.currentTime);
    
    // Update playhead position
    const playhead = document.getElementById('playhead');
    const progress = (videoPlayer.currentTime / videoPlayer.duration) * 100;
    playhead.style.left = `calc(100px + ${progress}%)`;
}

function onVideoLoaded() {
    const videoPlayer = document.getElementById('video-player');
    const durationEl = document.getElementById('duration');
    durationEl.textContent = formatTime(videoPlayer.duration);
}

function setVolume(value) {
    const videoPlayer = document.getElementById('video-player');
    videoPlayer.volume = value / 100;
}

// Effects
function applyEffect(effectType) {
    if (!currentVideo) {
        showError('Please load a video first');
        return;
    }
    
    showLoading();
    
    // Simulate effect application
    setTimeout(() => {
        hideLoading();
        showNotification(`${effectType} effect applied`);
        
        // Add effect to timeline
        addEffectToTimeline(effectType);
    }, 500);
}

function addEffectToTimeline(effectType) {
    const effectsTrack = document.querySelector('[data-track="effects"] .track-content');
    const effect = document.createElement('div');
    effect.className = 'timeline-effect';
    effect.innerHTML = `<span>${effectType}</span>`;
    effect.style.width = '100px';
    effect.style.left = '50px';
    effectsTrack.appendChild(effect);
}

// Transitions
function addTransition(transitionType) {
    showNotification(`${transitionType} transition added`);
}

// Zoom Controls
function zoomIn() {
    currentZoom = Math.min(currentZoom + 10, 200);
    updateZoom();
}

function zoomOut() {
    currentZoom = Math.max(currentZoom - 10, 50);
    updateZoom();
}

function updateZoom() {
    document.querySelector('.zoom-level').textContent = currentZoom + '%';
    // Apply zoom to preview
}

// Timeline Zoom
function timelineZoomIn() {
    // Implement timeline zoom
}

function timelineZoomOut() {
    // Implement timeline zoom
}

// Edit Operations
function undo() {
    showNotification('Undo');
}

function redo() {
    showNotification('Redo');
}

function splitClip() {
    if (!selectedClip) {
        showError('Please select a clip to split');
        return;
    }
    showNotification('Clip split');
}

function deleteClip() {
    if (!selectedClip) {
        showError('Please select a clip to delete');
        return;
    }
    selectedClip.remove();
    selectedClip = null;
    showNotification('Clip deleted');
}

// Save/Export
function saveProject() {
    showLoading();
    setTimeout(() => {
        hideLoading();
        showNotification('Project saved');
    }, 1000);
}

function exportVideo() {
    if (!currentVideo) {
        showError('No video to export');
        return;
    }
    
    showLoading();
    setTimeout(() => {
        hideLoading();
        showNotification('Export started. This may take a few minutes...');
    }, 1500);
}

// Properties Panel
function updatePropertiesPanel(clip) {
    const propertiesContent = document.getElementById('properties-content');
    propertiesContent.innerHTML = `
        <div class="property-group">
            <label>Clip Name</label>
            <input type="text" value="${clip.querySelector('.clip-name').textContent}" class="property-input">
        </div>
        <div class="property-group">
            <label>Duration</label>
            <input type="text" value="00:10" class="property-input">
        </div>
        <div class="property-group">
            <label>Position</label>
            <input type="number" value="0" class="property-input">
        </div>
        <div class="property-group">
            <label>Opacity</label>
            <input type="range" min="0" max="100" value="100" class="property-slider">
        </div>
    `;
}

// Keyboard Shortcuts
function setupKeyboardShortcuts() {
    document.addEventListener('keydown', (e) => {
        if (e.ctrlKey || e.metaKey) {
            switch(e.key) {
                case 'z':
                    e.preventDefault();
                    undo();
                    break;
                case 'y':
                    e.preventDefault();
                    redo();
                    break;
                case 's':
                    e.preventDefault();
                    saveProject();
                    break;
            }
        } else {
            switch(e.key) {
                case ' ':
                    e.preventDefault();
                    togglePlayback();
                    break;
                case 's':
                    splitClip();
                    break;
                case 'Delete':
                    deleteClip();
                    break;
            }
        }
    });
}

// UI Helpers
function showLoading() {
    document.getElementById('loading-overlay').style.display = 'flex';
}

function hideLoading() {
    document.getElementById('loading-overlay').style.display = 'none';
}

function showError(message) {
    const errorContainer = document.getElementById('error-container');
    const errorText = document.getElementById('error-text');
    
    errorText.textContent = message;
    errorContainer.style.display = 'block';
    
    setTimeout(() => {
        errorContainer.style.display = 'none';
    }, 5000);
}

function hideError() {
    document.getElementById('error-container').style.display = 'none';
}

function showNotification(message) {
    // For now, just log to console
    console.log('Notification:', message);
}

function formatTime(seconds) {
    const minutes = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

// Launch editor from homepage
function launchEditor() {
    window.location.href = 'editor.html';
}