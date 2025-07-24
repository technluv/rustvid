//! Comprehensive tests for frame operations

use super::*;
use pretty_assertions::assert_eq;
use proptest::prelude::*;
use rstest::*;
use test_case::test_case;

#[cfg(test)]
mod frame_creation {
    use super::*;
    
    #[test]
    fn test_frame_new_valid() {
        let frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_secs(0));
        assert!(frame.is_ok());
        
        let frame = frame.unwrap();
        assert_eq!(frame.width, 1920);
        assert_eq!(frame.height, 1080);
        assert_eq!(frame.format, PixelFormat::RGB24);
        assert_eq!(frame.timestamp, Duration::from_secs(0));
    }
    
    #[test_case(0, 1080, PixelFormat::RGB24; "zero width")]
    #[test_case(1920, 0, PixelFormat::RGB24; "zero height")]
    #[test_case(100_000, 100_000, PixelFormat::RGB24; "excessive dimensions")]
    fn test_frame_new_invalid(width: u32, height: u32, format: PixelFormat) {
        let frame = Frame::new(width, height, format, Duration::from_secs(0));
        assert!(frame.is_err());
    }
    
    #[rstest]
    #[case(PixelFormat::RGB24, 3)]
    #[case(PixelFormat::RGBA, 4)]
    #[case(PixelFormat::YUV420P, 3)] // Planar format
    #[case(PixelFormat::NV12, 3)]
    fn test_bytes_per_pixel(#[case] format: PixelFormat, #[case] expected: usize) {
        assert_eq!(format.bytes_per_pixel(), expected);
    }
}

#[cfg(test)]
mod frame_validation {
    use super::*;
    
    #[test]
    fn test_frame_validate_success() {
        let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_secs(0)).unwrap();
        assert!(frame.validate());
    }
    
    #[test]
    fn test_frame_validate_corrupted_data() {
        let mut frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_secs(0)).unwrap();
        // Corrupt the data by truncating it
        frame.data.truncate(100);
        assert!(!frame.validate());
    }
    
    #[test]
    fn test_frame_validate_zero_dimensions() {
        // This should not be possible to create, but we test the validation logic
        let mut frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_secs(0)).unwrap();
        frame.width = 0;
        assert!(!frame.validate());
    }
}

#[cfg(test)]
mod frame_cloning {
    use super::*;
    
    #[test]
    fn test_frame_clone() {
        let original = Frame::new(320, 240, PixelFormat::RGB24, Duration::from_millis(42)).unwrap();
        let cloned = original.clone();
        
        assert_eq!(original.width, cloned.width);
        assert_eq!(original.height, cloned.height);
        assert_eq!(original.format, cloned.format);
        assert_eq!(original.timestamp, cloned.timestamp);
        assert_eq!(original.data, cloned.data);
        
        // Ensure they have different data buffers
        assert!(!std::ptr::eq(original.data.as_ptr(), cloned.data.as_ptr()));
    }
}

#[cfg(test)]
mod frame_metadata {
    use super::*;
    
    #[test]
    fn test_frame_metadata_default() {
        let frame = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_secs(1)).unwrap();
        
        assert_eq!(frame.metadata.pts, None);
        assert_eq!(frame.metadata.dts, None);
        assert_eq!(frame.metadata.duration, None);
        assert_eq!(frame.metadata.key_frame, false);
        assert_eq!(frame.metadata.color_space, ColorSpace::BT709);
        assert_eq!(frame.metadata.hdr_metadata, None);
    }
    
    #[test]
    fn test_frame_with_metadata() {
        let mut frame = Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_secs(0)).unwrap();
        
        frame.metadata.pts = Some(1000);
        frame.metadata.dts = Some(900);
        frame.metadata.duration = Some(Duration::from_millis(33));
        frame.metadata.key_frame = true;
        frame.metadata.color_space = ColorSpace::BT2020;
        frame.metadata.hdr_metadata = Some(HdrMetadata {
            max_display_mastering_luminance: 1000.0,
            min_display_mastering_luminance: 0.001,
            max_content_light_level: 1000,
            max_frame_average_light_level: 400,
        });
        
        assert_eq!(frame.metadata.pts, Some(1000));
        assert_eq!(frame.metadata.key_frame, true);
        assert!(frame.metadata.hdr_metadata.is_some());
    }
}

#[cfg(test)]
mod frame_builder {
    use super::*;
    
    #[test]
    fn test_frame_builder_basic() {
        let frame = FrameBuilder::new()
            .width(1280)
            .height(720)
            .format(PixelFormat::YUV420P)
            .timestamp(Duration::from_millis(100))
            .build();
            
        assert!(frame.is_ok());
        let frame = frame.unwrap();
        assert_eq!(frame.width, 1280);
        assert_eq!(frame.height, 720);
        assert_eq!(frame.format, PixelFormat::YUV420P);
    }
    
    #[test]
    fn test_frame_builder_with_metadata() {
        let frame = FrameBuilder::new()
            .width(3840)
            .height(2160)
            .format(PixelFormat::RGB24)
            .timestamp(Duration::from_secs(0))
            .pts(Some(1000))
            .dts(Some(900))
            .key_frame(true)
            .color_space(ColorSpace::BT2020)
            .build();
            
        assert!(frame.is_ok());
        let frame = frame.unwrap();
        assert_eq!(frame.metadata.pts, Some(1000));
        assert_eq!(frame.metadata.key_frame, true);
        assert_eq!(frame.metadata.color_space, ColorSpace::BT2020);
    }
    
    #[test]
    fn test_frame_builder_invalid() {
        let result = FrameBuilder::new()
            .width(0)
            .height(1080)
            .format(PixelFormat::RGB24)
            .timestamp(Duration::from_secs(0))
            .build();
            
        assert!(result.is_err());
    }
}

// Property-based tests
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_frame_size_calculation(
            width in 1u32..4096,
            height in 1u32..2160,
        ) {
            let frame = Frame::new(width, height, PixelFormat::RGB24, Duration::from_secs(0)).unwrap();
            let expected_size = (width * height * 3) as usize;
            assert_eq!(frame.data.len(), expected_size);
        }
        
        #[test]
        fn test_frame_timestamp_ordering(
            ts1 in 0u64..1000000,
            ts2 in 0u64..1000000,
        ) {
            let frame1 = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_micros(ts1)).unwrap();
            let frame2 = Frame::new(640, 480, PixelFormat::RGB24, Duration::from_micros(ts2)).unwrap();
            
            if ts1 < ts2 {
                assert!(frame1.timestamp < frame2.timestamp);
            } else if ts1 > ts2 {
                assert!(frame1.timestamp > frame2.timestamp);
            } else {
                assert_eq!(frame1.timestamp, frame2.timestamp);
            }
        }
    }
}

// Performance tests (for use with criterion benchmarks)
#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_frame_allocation_speed() {
        use std::time::Instant;
        
        let start = Instant::now();
        let _frames: Vec<_> = (0..100)
            .map(|i| Frame::new(1920, 1080, PixelFormat::RGB24, Duration::from_millis(i as u64)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let elapsed = start.elapsed();
        
        // Should allocate 100 HD frames in under 1 second
        assert!(elapsed.as_secs() < 1, "Frame allocation too slow: {:?}", elapsed);
    }
    
    #[test]
    fn test_frame_clone_speed() {
        let original = Frame::new(3840, 2160, PixelFormat::RGBA, Duration::from_secs(0)).unwrap();
        
        use std::time::Instant;
        let start = Instant::now();
        let _clones: Vec<_> = (0..50).map(|_| original.clone()).collect();
        let elapsed = start.elapsed();
        
        // Should clone 50 4K frames in under 1 second
        assert!(elapsed.as_secs() < 1, "Frame cloning too slow: {:?}", elapsed);
    }
}

// Integration tests with actual pixel data
#[cfg(test)]
mod pixel_data_tests {
    use super::*;
    
    #[test]
    fn test_frame_pixel_access() {
        let mut frame = Frame::new(2, 2, PixelFormat::RGB24, Duration::from_secs(0)).unwrap();
        
        // Set pixel values
        frame.data[0] = 255; // R
        frame.data[1] = 0;   // G
        frame.data[2] = 0;   // B
        
        frame.data[3] = 0;   // R
        frame.data[4] = 255; // G
        frame.data[5] = 0;   // B
        
        // Verify pixel values
        assert_eq!(frame.data[0], 255);
        assert_eq!(frame.data[4], 255);
    }
    
    #[test]
    fn test_frame_fill_pattern() {
        let mut frame = Frame::new(100, 100, PixelFormat::RGB24, Duration::from_secs(0)).unwrap();
        
        // Fill with a pattern
        for (i, pixel) in frame.data.iter_mut().enumerate() {
            *pixel = (i % 256) as u8;
        }
        
        // Verify pattern
        for (i, &pixel) in frame.data.iter().enumerate() {
            assert_eq!(pixel, (i % 256) as u8);
        }
    }
}