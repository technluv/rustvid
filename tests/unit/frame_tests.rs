//! Unit tests for frame handling

use video_editor_core::{Frame, PixelFormat};
use std::time::Duration;

#[test]
fn test_frame_creation() {
    let frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(0));
    
    assert!(frame.is_ok());
    let frame = frame.unwrap();
    
    assert_eq!(frame.width(), 1920);
    assert_eq!(frame.height(), 1080);
    assert_eq!(frame.format(), PixelFormat::RGB24);
    assert_eq!(frame.timestamp(), Duration::from_millis(0));
}

#[test]
fn test_frame_validation() {
    let frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    assert!(frame.validate());
    
    // Test with corrupted frame (would need to implement corruption)
    // let mut corrupted = frame.clone();
    // corrupted.corrupt_data();
    // assert!(!corrupted.validate());
}

#[test]
fn test_frame_conversion() {
    let rgb_frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    
    // Test conversion to YUV420P
    let yuv_frame = rgb_frame.convert_to(PixelFormat::YUV420P);
    assert!(yuv_frame.is_ok());
    
    let yuv_frame = yuv_frame.unwrap();
    assert_eq!(yuv_frame.format(), PixelFormat::YUV420P);
    assert_eq!(yuv_frame.width(), 640);
    assert_eq!(yuv_frame.height(), 480);
}

#[test]
fn test_frame_memory_size() {
    let frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    
    // RGB24 = 3 bytes per pixel
    let expected_size = 1920 * 1080 * 3;
    assert_eq!(frame.data_size(), expected_size);
}

#[test]
fn test_frame_clone() {
    let original = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_millis(100)).unwrap();
    let cloned = original.clone();
    
    assert_eq!(original.width(), cloned.width());
    assert_eq!(original.height(), cloned.height());
    assert_eq!(original.format(), cloned.format());
    assert_eq!(original.timestamp(), cloned.timestamp());
    
    // Ensure deep copy
    assert!(!std::ptr::eq(original.data_ptr(), cloned.data_ptr()));
}

#[test]
fn test_frame_pixel_access() {
    let mut frame = Frame::new(100, 100, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    
    // Test pixel setting and getting
    let pixel = [255u8, 128u8, 64u8];
    frame.set_pixel(50, 50, &pixel).unwrap();
    
    let retrieved = frame.get_pixel(50, 50).unwrap();
    assert_eq!(retrieved, pixel);
}

#[test]
fn test_frame_bounds_checking() {
    let frame = Frame::new(100, 100, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
    
    // Test out of bounds access
    assert!(frame.get_pixel(100, 100).is_err());
    assert!(frame.get_pixel(0, 100).is_err());
    assert!(frame.get_pixel(100, 0).is_err());
}

#[test]
fn test_different_pixel_formats() {
    let formats = [
        PixelFormat::RGB24,
        PixelFormat::RGBA32,
        PixelFormat::YUV420P,
        PixelFormat::YUV422P,
        PixelFormat::GRAY8,
    ];
    
    for format in &formats {
        let frame = Frame::new(640, 480, *format, Duration::from_millis(0));
        assert!(frame.is_ok(), "Failed to create frame with format {:?}", format);
        
        let frame = frame.unwrap();
        assert_eq!(frame.format(), *format);
        
        // Check correct data size
        let expected_size = match format {
            PixelFormat::RGB24 => 640 * 480 * 3,
            PixelFormat::RGBA32 => 640 * 480 * 4,
            PixelFormat::YUV420P => (640 * 480 * 3) / 2,
            PixelFormat::YUV422P => 640 * 480 * 2,
            PixelFormat::GRAY8 => 640 * 480,
        };
        
        assert_eq!(frame.data_size(), expected_size);
    }
}

#[test]
#[should_panic]
fn test_invalid_frame_dimensions() {
    // Should panic with 0 width
    Frame::new(0, 480, PixelFormat::RGB24, Duration::from_millis(0)).unwrap();
}

#[test]
fn test_frame_metadata() {
    let mut frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(42)).unwrap();
    
    // Test metadata operations
    frame.set_metadata("camera", "Canon EOS R5");
    frame.set_metadata("iso", "800");
    
    assert_eq!(frame.get_metadata("camera"), Some("Canon EOS R5"));
    assert_eq!(frame.get_metadata("iso"), Some("800"));
    assert_eq!(frame.get_metadata("nonexistent"), None);
}