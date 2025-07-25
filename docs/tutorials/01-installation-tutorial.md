# Installation Tutorial Script
**Duration**: 3-5 minutes  
**Target Audience**: New users, developers interested in Rust video editing

## Video Structure

### Opening (0:00-0:15)
**Screen**: Logo animation with title "Rust Video Editor - Installation Guide"
**Narration**: "Welcome to the Rust Video Editor! In this tutorial, we'll walk you through installing and setting up your development environment in just a few minutes."

### Section 1: Prerequisites (0:15-0:45)
**Screen**: Split screen showing Rust website and terminal
**Highlight Areas**: 
- Rust installation page
- Terminal window
- System requirements list

**Narration**: 
"Before we begin, let's ensure you have the prerequisites:
- First, you'll need Rust installed. Visit rustup.rs to get the latest version
- We recommend at least 8GB of RAM and 10GB of free disk space
- A modern GPU is helpful for video processing, but not required for development"

**Key Points**:
- Show rustup installation command
- Demonstrate checking Rust version: `rustc --version`
- Show cargo version: `cargo --version`

### Section 2: Cloning the Repository (0:45-1:30)
**Screen**: Terminal with git commands
**Highlight Areas**: 
- Git clone command
- Directory navigation
- Project structure overview

**Narration**:
"Let's clone the Rust Video Editor repository. Open your terminal and run:
`git clone https://github.com/yourusername/rust-video-editor.git`
Then navigate into the project directory:
`cd rust-video-editor`"

**Key Points**:
- Show successful clone output
- Use `tree -L 2` to show project structure
- Highlight important directories: src/, assets/, examples/

### Section 3: Installing Dependencies (1:30-2:30)
**Screen**: Terminal showing dependency installation
**Highlight Areas**:
- Cargo.toml file
- cargo build output
- FFmpeg installation

**Narration**:
"Now let's install the project dependencies. The Rust Video Editor uses FFmpeg for media processing. 
On Ubuntu/Debian: `sudo apt-get install ffmpeg libavcodec-dev libavformat-dev`
On macOS: `brew install ffmpeg`
On Windows: Download from ffmpeg.org"

**Key Points**:
- Show platform-specific installation
- Explain why FFmpeg is needed
- Show cargo fetching dependencies

### Section 4: Building the Project (2:30-3:30)
**Screen**: Terminal with build process
**Highlight Areas**:
- cargo build command
- Build progress
- Successful compilation message

**Narration**:
"Let's build the project. Run:
`cargo build --release`
This will compile the video editor with optimizations. The first build might take a few minutes as Cargo downloads and compiles all dependencies."

**Key Points**:
- Explain debug vs release builds
- Show build progress
- Point out where the binary is created

### Section 5: Verification & First Run (3:30-4:30)
**Screen**: Running the application
**Highlight Areas**:
- Launch command
- Initial UI
- Sample project loading

**Narration**:
"Let's verify our installation by running the editor:
`cargo run --release`
You should see the main window appear. Let's load a sample video to ensure everything is working correctly."

**Key Points**:
- Show successful launch
- Load a sample video
- Verify playback works

### Closing (4:30-5:00)
**Screen**: Main editor interface with contact info
**Narration**:
"Congratulations! You've successfully installed the Rust Video Editor. In our next tutorial, we'll explore the basic editing workflow. Check the description for links to documentation and our community Discord. Happy editing!"

## Recording Tips
- Use a clean desktop environment
- Increase terminal font size for visibility
- Have sample media files ready
- Test all commands before recording
- Keep mouse movements smooth and deliberate

## Sample Commands Checklist
```bash
# Prerequisites check
rustc --version
cargo --version
git --version

# Installation
git clone https://github.com/yourusername/rust-video-editor.git
cd rust-video-editor

# Dependencies (choose based on OS)
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install ffmpeg libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev

# macOS
brew install ffmpeg

# Build
cargo build --release

# Run
cargo run --release

# Optional: Run tests
cargo test
```

## Troubleshooting Section (Optional Extended Content)
- Address common installation issues
- FFmpeg version compatibility
- GPU driver requirements
- Build error solutions