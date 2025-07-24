#!/usr/bin/env python3
"""
Script to create test video fixtures for the Rust Video Editor test suite.
Requires FFmpeg to be installed.
"""

import subprocess
import os
import sys
from pathlib import Path

def run_ffmpeg(args, output_file):
    """Run FFmpeg command and check for success."""
    cmd = ['ffmpeg', '-y'] + args
    print(f"Creating {output_file}...")
    print(f"Command: {' '.join(cmd)}")
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        print(f"✓ Successfully created {output_file}")
        return True
    except subprocess.CalledProcessError as e:
        print(f"✗ Failed to create {output_file}")
        print(f"Error: {e.stderr}")
        return False
    except FileNotFoundError:
        print("✗ FFmpeg not found. Please install FFmpeg.")
        return False

def create_test_videos():
    """Create various test video files for testing."""
    
    # Create fixtures directory
    fixtures_dir = Path(__file__).parent
    fixtures_dir.mkdir(exist_ok=True)
    
    os.chdir(fixtures_dir)
    
    test_videos = [
        # Basic test video - 5 seconds, 720p, 30fps
        {
            'name': 'test_720p_30fps.mp4',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=duration=5:size=1280x720:rate=30',
                '-c:v', 'libx264',
                '-preset', 'ultrafast',
                '-pix_fmt', 'yuv420p',
                'test_720p_30fps.mp4'
            ]
        },
        
        # Short video for quick tests - 2 seconds, 480p
        {
            'name': 'test_480p_quick.mp4',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=duration=2:size=640x480:rate=30',
                '-c:v', 'libx264',
                '-preset', 'ultrafast',
                '-pix_fmt', 'yuv420p',
                'test_480p_quick.mp4'
            ]
        },
        
        # HD test video - 3 seconds, 1080p
        {
            'name': 'test_1080p_hd.mp4',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=duration=3:size=1920x1080:rate=30',
                '-c:v', 'libx264',
                '-preset', 'ultrafast',
                '-pix_fmt', 'yuv420p',
                'test_1080p_hd.mp4'
            ]
        },
        
        # Color test video with different patterns
        {
            'name': 'test_color_patterns.mp4',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc2=duration=4:size=640x480:rate=25',
                '-c:v', 'libx264',
                '-preset', 'ultrafast',
                '-pix_fmt', 'yuv420p',
                'test_color_patterns.mp4'
            ]
        },
        
        # Different frame rate - 60fps
        {
            'name': 'test_60fps.mp4',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=duration=2:size=640x480:rate=60',
                '-c:v', 'libx264',
                '-preset', 'ultrafast',
                '-pix_fmt', 'yuv420p',
                'test_60fps.mp4'
            ]
        },
        
        # Video with audio track
        {
            'name': 'test_with_audio.mp4',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=duration=3:size=640x480:rate=30',
                '-f', 'lavfi',
                '-i', 'sine=frequency=440:duration=3',
                '-c:v', 'libx264',
                '-c:a', 'aac',
                '-preset', 'ultrafast',
                '-pix_fmt', 'yuv420p',
                'test_with_audio.mp4'
            ]
        },
        
        # Different codec - H.265/HEVC (if available)
        {
            'name': 'test_hevc.mp4',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=duration=2:size=640x480:rate=30',
                '-c:v', 'libx265',
                '-preset', 'ultrafast',
                '-pix_fmt', 'yuv420p',
                'test_hevc.mp4'
            ]
        },
        
        # WebM format
        {
            'name': 'test_webm.webm',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=duration=2:size=640x480:rate=30',
                '-c:v', 'libvpx-vp9',
                '-b:v', '1M',
                'test_webm.webm'
            ]
        },
        
        # MOV format
        {
            'name': 'test_mov.mov',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=duration=2:size=640x480:rate=30',
                '-c:v', 'libx264',
                '-preset', 'ultrafast',
                '-pix_fmt', 'yuv420p',
                'test_mov.mov'
            ]
        },
        
        # Very short video for edge case testing
        {
            'name': 'test_single_frame.mp4',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=duration=0.033:size=320x240:rate=30',
                '-c:v', 'libx264',
                '-preset', 'ultrafast',
                '-pix_fmt', 'yuv420p',
                'test_single_frame.mp4'
            ]
        }
    ]
    
    successful = 0
    failed = 0
    
    for video in test_videos:
        if run_ffmpeg(video['args'], video['name']):
            successful += 1
        else:
            failed += 1
    
    print(f"\n=== Test Video Creation Summary ===")
    print(f"Successfully created: {successful} videos")
    print(f"Failed to create: {failed} videos")
    
    if failed > 0:
        print(f"\nSome videos failed to create. This might be due to:")
        print(f"- Missing codecs (e.g., libx265, libvpx-vp9)")
        print(f"- FFmpeg version compatibility")
        print(f"- System limitations")
        print(f"\nThe tests will skip videos that are not available.")
    
    return failed == 0

def create_image_fixtures():
    """Create test images for frame processing tests."""
    
    print("\n=== Creating Image Fixtures ===")
    
    image_fixtures = [
        # Test patterns
        {
            'name': 'test_pattern_640x480.png',
            'args': [
                '-f', 'lavfi',
                '-i', 'testsrc=size=640x480:duration=1:rate=1',
                '-frames:v', '1',
                'test_pattern_640x480.png'
            ]
        },
        
        # Solid colors
        {
            'name': 'solid_red_1920x1080.png',
            'args': [
                '-f', 'lavfi',
                '-i', 'color=red:size=1920x1080:duration=1:rate=1',
                '-frames:v', '1',
                'solid_red_1920x1080.png'
            ]
        },
        
        {
            'name': 'solid_green_640x480.png',
            'args': [
                '-f', 'lavfi',
                '-i', 'color=green:size=640x480:duration=1:rate=1',
                '-frames:v', '1',
                'solid_green_640x480.png'
            ]
        },
        
        # Gradient
        {
            'name': 'gradient_1280x720.png',
            'args': [
                '-f', 'lavfi',
                '-i', 'gradients=size=1280x720:duration=1:rate=1',
                '-frames:v', '1',
                'gradient_1280x720.png'
            ]
        }
    ]
    
    successful = 0
    failed = 0
    
    for image in image_fixtures:
        if run_ffmpeg(image['args'], image['name']):
            successful += 1
        else:
            failed += 1
    
    print(f"\nImage fixtures - Successfully created: {successful}, Failed: {failed}")
    return failed == 0

def create_fixture_info():
    """Create a JSON file with information about all fixtures."""
    
    import json
    from datetime import datetime
    
    fixtures_info = {
        'created': datetime.now().isoformat(),
        'description': 'Test fixtures for Rust Video Editor',
        'videos': [
            {
                'filename': 'test_720p_30fps.mp4',
                'description': 'Basic 720p test video, 5 seconds, 30fps',
                'resolution': '1280x720',
                'duration': 5.0,
                'fps': 30,
                'codec': 'H.264'
            },
            {
                'filename': 'test_480p_quick.mp4',
                'description': 'Quick test video for fast tests',
                'resolution': '640x480',
                'duration': 2.0,
                'fps': 30,
                'codec': 'H.264'
            },
            {
                'filename': 'test_1080p_hd.mp4',
                'description': 'HD test video',
                'resolution': '1920x1080',
                'duration': 3.0,
                'fps': 30,
                'codec': 'H.264'
            },
            {
                'filename': 'test_color_patterns.mp4',
                'description': 'Color pattern test video',
                'resolution': '640x480',
                'duration': 4.0,
                'fps': 25,
                'codec': 'H.264'
            },
            {
                'filename': 'test_60fps.mp4',
                'description': 'High frame rate test',
                'resolution': '640x480',
                'duration': 2.0,
                'fps': 60,
                'codec': 'H.264'
            },
            {
                'filename': 'test_with_audio.mp4',
                'description': 'Video with audio track',
                'resolution': '640x480',
                'duration': 3.0,
                'fps': 30,
                'codec': 'H.264',
                'audio': True,
                'audio_codec': 'AAC'
            }
        ],
        'images': [
            {
                'filename': 'test_pattern_640x480.png',
                'description': 'Test pattern image',
                'resolution': '640x480'
            },
            {
                'filename': 'solid_red_1920x1080.png',
                'description': 'Solid red image for testing',
                'resolution': '1920x1080'
            }
        ]
    }
    
    with open('fixtures_info.json', 'w') as f:
        json.dump(fixtures_info, f, indent=2)
    
    print("✓ Created fixtures_info.json")

def main():
    """Main function to create all test fixtures."""
    
    print("=== Rust Video Editor Test Fixture Generator ===")
    print("This script creates test videos and images for the test suite.")
    print("Requires FFmpeg to be installed and available in PATH.\n")
    
    video_success = create_test_videos()
    image_success = create_image_fixtures()
    
    if video_success and image_success:
        create_fixture_info()
        print(f"\n✅ All fixtures created successfully!")
        print(f"Fixtures are located in: {Path(__file__).parent}")
        return 0
    else:
        print(f"\n⚠️  Some fixtures failed to create, but tests can still run.")
        print(f"Missing fixtures will be skipped during testing.")
        return 1

if __name__ == '__main__':
    sys.exit(main())