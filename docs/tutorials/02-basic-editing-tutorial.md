# Basic Editing Workflow Tutorial Script
**Duration**: 5-7 minutes  
**Target Audience**: New users ready to start editing

## Video Structure

### Opening (0:00-0:20)
**Screen**: Editor interface with welcome screen
**Narration**: "Welcome back! In this tutorial, we'll cover the essential editing workflow in Rust Video Editor. By the end, you'll be comfortable creating your first video project from start to finish."

### Section 1: Creating a New Project (0:20-1:00)
**Screen**: File menu and new project dialog
**Highlight Areas**:
- File → New Project menu
- Project settings dialog
- Resolution and framerate options
- Project location selection

**Narration**:
"Let's start by creating a new project. Click File → New Project.
Name your project 'My First Edit'.
Select your desired resolution - we'll use 1920x1080 at 30fps for this tutorial.
Choose a location to save your project file."

**Key Points**:
- Explain project file vs media files
- Show different resolution presets
- Mention fps implications

### Section 2: Importing Media (1:00-2:00)
**Screen**: Media import process
**Highlight Areas**:
- Import button/menu
- File browser
- Media bin/library
- Thumbnail generation

**Narration**:
"Now let's import some media. Click the Import button or press Ctrl+I.
You can select multiple files at once - videos, images, and audio.
Notice how the editor generates thumbnails and analyzes the media properties.
Your imported files appear in the Media Bin on the left."

**Key Points**:
- Demonstrate multi-select
- Show supported formats
- Explain proxy generation option
- Show media properties panel

### Section 3: Timeline Basics (2:00-3:30)
**Screen**: Timeline panel
**Highlight Areas**:
- Timeline tracks (V1, V2, A1, A2)
- Playhead
- Zoom controls
- Track headers

**Narration**:
"The timeline is where we'll build our video. 
- Video tracks (V1, V2) stack visually, with higher tracks on top
- Audio tracks (A1, A2) mix together
- The playhead shows your current position
- Use the zoom slider or scroll wheel to zoom in/out"

**Key Points**:
- Drag clip to timeline
- Show snapping behavior
- Explain track hierarchy
- Demonstrate timeline navigation

### Section 4: Basic Editing Operations (3:30-5:00)
**Screen**: Performing edits on timeline
**Highlight Areas**:
- Selection tool
- Razor/cut tool
- Trim handles
- Ripple/roll edit modes

**Narration**:
"Let's perform some basic edits:
1. Select a clip with the Selection tool (V)
2. Position the playhead and press 'S' to split
3. Delete unwanted sections with Delete key
4. Trim clips by dragging the edges
5. Rearrange clips by dragging them"

**Key Points**:
- Show keyboard shortcuts in action
- Demonstrate ripple delete
- Show gap removal
- Explain magnetic timeline

**Demo Sequence**:
```
1. Import 3 clips
2. Rough cut to remove unwanted parts
3. Rearrange for better flow
4. Fine-tune edit points
```

### Section 5: Adding Transitions (5:00-5:45)
**Screen**: Transitions panel and timeline
**Highlight Areas**:
- Transitions browser
- Drag & drop zone between clips
- Transition properties

**Narration**:
"To smooth the cuts between clips, let's add transitions.
Find the Transitions panel, usually on the right.
Drag a Cross Dissolve between two clips.
Adjust the duration by dragging the transition edges."

**Key Points**:
- Show popular transitions
- Explain transition alignment
- Demonstrate property adjustment

### Section 6: Basic Audio Editing (5:45-6:30)
**Screen**: Audio tracks and controls
**Highlight Areas**:
- Audio waveforms
- Volume line
- Audio meters
- Fade handles

**Narration**:
"Good audio is crucial. Let's adjust our audio:
- See the waveforms for visual reference
- Adjust volume by dragging the yellow line
- Create fades by dragging the top corners
- Monitor levels in the audio meters - aim for -12 to -6 dB"

**Key Points**:
- Show keyframe creation
- Demonstrate audio transitions
- Explain proper levels

### Section 7: Preview and Playback (6:30-7:00)
**Screen**: Preview window and controls
**Highlight Areas**:
- Play/pause controls
- Quality settings
- Full-screen preview
- JKL playback

**Narration**:
"Let's preview our edit:
- Press Space to play/pause
- Use J-K-L for professional playback control
- Adjust preview quality if playback stutters
- Press F for fullscreen preview"

### Closing (7:00-7:30)
**Screen**: Completed timeline with final result
**Narration**:
"Great job! You've learned the basic editing workflow. You can now import media, make cuts, add transitions, and adjust audio. In our next tutorial, we'll explore effects and color correction. Keep practicing these fundamentals - they're the foundation of all video editing!"

## Recording Tips
- Pre-arrange media files in a folder
- Use consistent, high-quality sample footage
- Keep edits simple but visible
- Show both mouse and keyboard actions
- Pause briefly after each major action

## Sample Project Structure
```
My First Edit/
├── Footage/
│   ├── clip1_interview.mp4 (30 seconds)
│   ├── clip2_broll.mp4 (45 seconds)
│   └── clip3_ending.mp4 (20 seconds)
├── Audio/
│   └── background_music.mp3
└── Images/
    └── title_card.png
```

## Keyboard Shortcuts Reference
- **Import**: Ctrl/Cmd + I
- **Split**: S
- **Delete**: Delete/Backspace
- **Selection Tool**: V
- **Razor Tool**: C
- **Play/Pause**: Space
- **Save**: Ctrl/Cmd + S
- **Undo**: Ctrl/Cmd + Z
- **Redo**: Ctrl/Cmd + Shift + Z

## Common Beginner Mistakes to Address
1. Not saving project regularly
2. Ignoring audio levels
3. Cutting too quickly
4. Overusing transitions
5. Not previewing edits