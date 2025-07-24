//! Comprehensive tests for video effects

use effects::*;
use std::any::Any;

#[derive(Debug, Clone)]
struct TestFrame {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl TestFrame {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![128; (width * height * 3) as usize], // Gray pixels
        }
    }
    
    fn average_brightness(&self) -> f32 {
        let sum: u32 = self.data.iter().map(|&b| b as u32).sum();
        sum as f32 / self.data.len() as f32 / 255.0
    }
}

#[cfg(test)]
mod basic_effect_tests {
    use super::*;
    
    #[test]
    fn test_brightness_effect_creation() {
        let effect = BrightnessEffect::new();
        assert_eq!(effect.name(), "Brightness");
        
        let params = effect.get_parameters();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].name, "brightness");
    }
    
    #[test]
    fn test_brightness_parameter_range() {
        let mut effect = BrightnessEffect::new();
        
        // Test valid range
        assert!(effect.set_parameter("brightness", ParameterValue::Float(0.0)).is_ok());
        assert!(effect.set_parameter("brightness", ParameterValue::Float(1.0)).is_ok());
        assert!(effect.set_parameter("brightness", ParameterValue::Float(2.0)).is_ok());
        
        // Test invalid parameter name
        assert!(effect.set_parameter("invalid", ParameterValue::Float(1.0)).is_err());
        
        // Test wrong parameter type
        assert!(effect.set_parameter("brightness", ParameterValue::Integer(1)).is_err());
    }
    
    #[test]
    fn test_effect_processing() {
        let mut effect = BrightnessEffect::new();
        let mut frame = TestFrame::new(100, 100);
        
        // Apply brightness increase
        effect.set_parameter("brightness", ParameterValue::Float(1.5)).unwrap();
        
        // In a real implementation, this would modify the frame data
        let result = effect.process(&mut frame as &mut dyn Any);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod advanced_effect_tests {
    use super::*;
    
    // Test color correction effect
    struct ColorCorrectionEffect {
        hue_shift: f32,
        saturation: f32,
        contrast: f32,
    }
    
    impl ColorCorrectionEffect {
        fn new() -> Self {
            Self {
                hue_shift: 0.0,
                saturation: 1.0,
                contrast: 1.0,
            }
        }
    }
    
    impl Effect for ColorCorrectionEffect {
        fn name(&self) -> &str {
            "Color Correction"
        }
        
        fn process(&mut self, input: &mut dyn Any) -> Result<()> {
            // Implementation would apply color transformations
            Ok(())
        }
        
        fn get_parameters(&self) -> Vec<Parameter> {
            vec![
                Parameter {
                    name: "hue_shift".to_string(),
                    value: ParameterValue::Float(self.hue_shift),
                    min: Some(ParameterValue::Float(-180.0)),
                    max: Some(ParameterValue::Float(180.0)),
                },
                Parameter {
                    name: "saturation".to_string(),
                    value: ParameterValue::Float(self.saturation),
                    min: Some(ParameterValue::Float(0.0)),
                    max: Some(ParameterValue::Float(2.0)),
                },
                Parameter {
                    name: "contrast".to_string(),
                    value: ParameterValue::Float(self.contrast),
                    min: Some(ParameterValue::Float(0.0)),
                    max: Some(ParameterValue::Float(2.0)),
                },
            ]
        }
        
        fn set_parameter(&mut self, name: &str, value: ParameterValue) -> Result<()> {
            match (name, value) {
                ("hue_shift", ParameterValue::Float(v)) => {
                    if v >= -180.0 && v <= 180.0 {
                        self.hue_shift = v;
                        Ok(())
                    } else {
                        Err(EffectError::InvalidParameter("Hue shift out of range".into()))
                    }
                }
                ("saturation", ParameterValue::Float(v)) => {
                    if v >= 0.0 && v <= 2.0 {
                        self.saturation = v;
                        Ok(())
                    } else {
                        Err(EffectError::InvalidParameter("Saturation out of range".into()))
                    }
                }
                ("contrast", ParameterValue::Float(v)) => {
                    if v >= 0.0 && v <= 2.0 {
                        self.contrast = v;
                        Ok(())
                    } else {
                        Err(EffectError::InvalidParameter("Contrast out of range".into()))
                    }
                }
                _ => Err(EffectError::InvalidParameter(format!("Unknown parameter: {}", name))),
            }
        }
    }
    
    #[test]
    fn test_color_correction_effect() {
        let mut effect = ColorCorrectionEffect::new();
        
        // Test parameter validation
        assert!(effect.set_parameter("hue_shift", ParameterValue::Float(90.0)).is_ok());
        assert!(effect.set_parameter("saturation", ParameterValue::Float(1.5)).is_ok());
        assert!(effect.set_parameter("contrast", ParameterValue::Float(1.2)).is_ok());
        
        // Test out of range
        assert!(effect.set_parameter("hue_shift", ParameterValue::Float(200.0)).is_err());
        assert!(effect.set_parameter("saturation", ParameterValue::Float(-1.0)).is_err());
        assert!(effect.set_parameter("contrast", ParameterValue::Float(3.0)).is_err());
    }
}

#[cfg(test)]
mod effect_chain_tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    
    struct EffectChain {
        effects: Vec<Box<dyn Effect>>,
    }
    
    impl EffectChain {
        fn new() -> Self {
            Self {
                effects: Vec::new(),
            }
        }
        
        fn add_effect(&mut self, effect: Box<dyn Effect>) {
            self.effects.push(effect);
        }
        
        fn process(&mut self, frame: &mut dyn Any) -> Result<()> {
            for effect in &mut self.effects {
                effect.process(frame)?;
            }
            Ok(())
        }
    }
    
    #[test]
    fn test_effect_chain() {
        let mut chain = EffectChain::new();
        
        // Add multiple effects
        chain.add_effect(Box::new(BrightnessEffect::new()));
        chain.add_effect(Box::new(ColorCorrectionEffect::new()));
        
        let mut frame = TestFrame::new(640, 480);
        let result = chain.process(&mut frame as &mut dyn Any);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_effect_performance() {
        let mut effect = BrightnessEffect::new();
        let mut frames: Vec<TestFrame> = (0..100)
            .map(|_| TestFrame::new(1920, 1080))
            .collect();
        
        let start = Instant::now();
        for frame in &mut frames {
            effect.process(frame as &mut dyn Any).unwrap();
        }
        let elapsed = start.elapsed();
        
        println!("Processed 100 HD frames in {:?}", elapsed);
        // Should process 100 frames in under 1 second (placeholder implementation)
        assert!(elapsed.as_secs() < 1);
    }
    
    #[test]
    fn test_parameter_update_performance() {
        let mut effect = BrightnessEffect::new();
        
        let start = Instant::now();
        for i in 0..10000 {
            let value = ((i % 200) as f32) / 100.0; // 0.0 to 2.0
            effect.set_parameter("brightness", ParameterValue::Float(value)).unwrap();
        }
        let elapsed = start.elapsed();
        
        println!("Updated parameter 10,000 times in {:?}", elapsed);
        assert!(elapsed.as_millis() < 100); // Should be very fast
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;
    
    #[test]
    fn test_parameter_serialization() {
        let param = Parameter {
            name: "test_param".to_string(),
            value: ParameterValue::Float(1.5),
            min: Some(ParameterValue::Float(0.0)),
            max: Some(ParameterValue::Float(2.0)),
        };
        
        let json = serde_json::to_string(&param).unwrap();
        let deserialized: Parameter = serde_json::from_str(&json).unwrap();
        
        assert_eq!(param.name, deserialized.name);
        match (&param.value, &deserialized.value) {
            (ParameterValue::Float(a), ParameterValue::Float(b)) => assert_eq!(a, b),
            _ => panic!("Parameter value mismatch"),
        }
    }
    
    #[test]
    fn test_parameter_value_types() {
        let values = vec![
            ParameterValue::Float(1.5),
            ParameterValue::Integer(42),
            ParameterValue::Boolean(true),
            ParameterValue::String("test".to_string()),
            ParameterValue::Color(0xFF00FF),
        ];
        
        for value in values {
            let json = serde_json::to_string(&value).unwrap();
            let deserialized: ParameterValue = serde_json::from_str(&json).unwrap();
            
            match (&value, &deserialized) {
                (ParameterValue::Float(a), ParameterValue::Float(b)) => assert_eq!(a, b),
                (ParameterValue::Integer(a), ParameterValue::Integer(b)) => assert_eq!(a, b),
                (ParameterValue::Boolean(a), ParameterValue::Boolean(b)) => assert_eq!(a, b),
                (ParameterValue::String(a), ParameterValue::String(b)) => assert_eq!(a, b),
                (ParameterValue::Color(a), ParameterValue::Color(b)) => assert_eq!(a, b),
                _ => panic!("Parameter value type mismatch"),
            }
        }
    }
}

// GPU effect tests (if GPU is available)
#[cfg(all(test, feature = "gpu"))]
mod gpu_effect_tests {
    use super::*;
    
    #[test]
    fn test_gpu_effect_initialization() {
        // Test GPU effect initialization
        // This would require actual GPU context setup
    }
    
    #[test]
    fn test_shader_compilation() {
        // Test shader compilation and validation
    }
}