# Export Settings Tutorial Script
**Duration**: 3-5 minutes  
**Target Audience**: Users ready to export their finished projects

## Video Structure

### Opening (0:00-0:15)
**Screen**: Finished timeline ready for export
**Narration**: "You've edited your masterpiece - now let's get it out into the world! This tutorial covers everything you need to know about exporting from Rust Video Editor with the best quality and file sizes."

### Section 1: Export Dialog Overview (0:15-0:45)
**Screen**: Export dialog window
**Highlight Areas**:
- Format dropdown
- Preset selector
- Custom settings area
- Output location
- Export queue

**Narration**:
"Access export via File → Export or press Ctrl+M.
The export dialog has three main sections:
1. Format selection - choose your container
2. Presets - quick settings for common uses  
3. Custom parameters - full control over encoding"

**Key Points**:
- Explain format vs codec
- Show preset categories
- Highlight time estimate
- Point out file size prediction

### Section 2: Choosing the Right Format (0:45-1:30)
**Screen**: Format dropdown expanded
**Highlight Areas**:
- MP4 (H.264/H.265)
- WebM (VP9)
- ProRes
- DNxHD/DNxHR
- Image sequences

**Narration**:
"Choosing the right format depends on your destination:
- **MP4 with H.264**: Universal compatibility, good for web
- **MP4 with H.265**: Better quality, smaller files, less compatible
- **WebM**: Open source, great for web streaming
- **ProRes**: Professional editing, large files
- **Image Sequence**: For VFX or further processing"

**Quick Guide Table**:
| Use Case | Format | Codec |
|----------|---------|--------|
| YouTube | MP4 | H.264 |
| Web | WebM | VP9 |
| Editing | ProRes | ProRes 422 |
| Archive | ProRes | ProRes 4444 |

### Section 3: Resolution and Frame Rate (1:30-2:15)
**Screen**: Resolution settings
**Highlight Areas**:
- Resolution dropdown
- Custom resolution fields
- Frame rate selector
- Aspect ratio lock

**Narration**:
"Match your export to your intended platform:
- Keep original resolution when possible
- YouTube: 1080p or 4K for best quality
- Instagram: 1080x1080 (square) or 1080x1920 (stories)
- Always maintain aspect ratio unless specifically needed"

**Platform Recommendations**:
- YouTube: 1920x1080 or 3840x2160
- Instagram Feed: 1080x1080
- Instagram Stories: 1080x1920  
- Twitter: 1280x720
- TikTok: 1080x1920

### Section 4: Quality Settings (2:15-3:15)
**Screen**: Bitrate and quality controls
**Highlight Areas**:
- Quality slider/dropdown
- Bitrate settings (CBR/VBR)
- Two-pass encoding
- Audio settings

**Narration**:
"Quality settings balance file size with visual fidelity:
- **Bitrate**: Higher = better quality, larger files
- **VBR** (Variable): Efficient, adjusts to scene complexity
- **CBR** (Constant): Predictable size, streaming-friendly
- **Two-pass**: Best quality, takes longer"

**Recommended Bitrates**:
- 1080p @30fps: 8-12 Mbps
- 1080p @60fps: 12-18 Mbps
- 4K @30fps: 35-45 Mbps
- 4K @60fps: 50-80 Mbps

**Audio Settings**:
- Format: AAC for compatibility
- Bitrate: 320 kbps for high quality
- Sample Rate: 48 kHz standard

### Section 5: Advanced Export Options (3:15-4:00)
**Screen**: Advanced settings panel
**Highlight Areas**:
- Color space settings
- GPU acceleration toggle
- Metadata fields
- Chapter markers
- Burn-in options

**Narration**:
"Advanced options for specific needs:
- **Color Space**: Rec.709 for HD, Rec.2020 for HDR
- **GPU Acceleration**: Faster exports with compatible GPUs
- **Metadata**: Add title, artist, copyright info
- **Burn-ins**: Timecode, watermarks (permanent)"

**Pro Tips**:
- Enable GPU encoding when available
- Use hardware encoders (NVENC, QuickSync)
- Add metadata for professional delivery
- Export markers as chapters for long videos

### Section 6: Export Queue and Batch Export (4:00-4:30)
**Screen**: Export queue interface
**Highlight Areas**:
- Add to queue button
- Queue management
- Batch settings
- Background export

**Narration**:
"The export queue lets you:
- Queue multiple exports with different settings
- Export various formats simultaneously  
- Continue editing while exporting
- Set up overnight batch exports"

**Queue Workflow**:
1. Set up first export (YouTube version)
2. Add to queue
3. Change settings (Instagram version)
4. Add to queue
5. Start batch export

### Section 7: Monitoring Export Progress (4:30-4:45)
**Screen**: Export progress window
**Highlight Areas**:
- Progress bar
- Time remaining
- Current frame preview
- Cancel/pause options

**Narration**:
"During export, monitor:
- Overall progress percentage
- Estimated time remaining
- Current frame being processed
- You can pause or cancel if needed"

### Closing (4:45-5:00)
**Screen**: Exported file in file browser
**Narration**:
"Perfect! You now know how to export for any platform or purpose. Remember to always preview your export before sharing. In our next tutorial, we'll explore advanced features like motion tracking and compositing. Happy exporting!"

## Recording Tips
- Have a complete project ready to export
- Show actual rendering progress
- Compare different quality settings
- Display file sizes for each option
- Test playback of exported file

## Export Preset Examples
```yaml
YouTube 1080p:
  Format: MP4
  Codec: H.264
  Resolution: 1920x1080
  Framerate: Original
  Bitrate: 10 Mbps (VBR)
  Audio: AAC 320kbps

Instagram Square:
  Format: MP4
  Codec: H.264  
  Resolution: 1080x1080
  Framerate: 30fps
  Bitrate: 8 Mbps (VBR)
  Audio: AAC 256kbps

Professional Archive:
  Format: MOV
  Codec: ProRes 422 HQ
  Resolution: Original
  Framerate: Original
  Audio: PCM 48kHz
```

## Common Export Issues to Address
1. **File too large**: Lower bitrate or resolution
2. **Choppy playback**: Codec not supported, try H.264
3. **Color shifts**: Check color space settings
4. **No audio**: Verify audio codec compatibility
5. **Slow exports**: Enable GPU acceleration

## Platform-Specific Requirements
**YouTube**:
- Container: MP4
- Video: H.264 (Main Profile)
- Audio: AAC-LC
- Max file: 128GB

**Vimeo**:
- Accepts most formats
- Recommends H.264 or H.265
- Higher bitrates supported

**Social Media**:
- Usually requires MP4 H.264
- Specific aspect ratios
- File size limits vary