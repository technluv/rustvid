use crate::error::{EffectError, Result};
use glam::{Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a parameter value that can be animated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ParameterValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    Color(Vec4),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    String(String),
}

impl ParameterValue {
    pub fn as_float(&self) -> Result<f32> {
        match self {
            ParameterValue::Float(v) => Ok(*v),
            ParameterValue::Int(v) => Ok(*v as f32),
            _ => Err(EffectError::InvalidParameter(
                "Expected float parameter".to_string(),
            )),
        }
    }
    
    pub fn as_int(&self) -> Result<i32> {
        match self {
            ParameterValue::Int(v) => Ok(*v),
            ParameterValue::Float(v) => Ok(*v as i32),
            _ => Err(EffectError::InvalidParameter(
                "Expected int parameter".to_string(),
            )),
        }
    }
    
    pub fn as_bool(&self) -> Result<bool> {
        match self {
            ParameterValue::Bool(v) => Ok(*v),
            _ => Err(EffectError::InvalidParameter(
                "Expected bool parameter".to_string(),
            )),
        }
    }
    
    pub fn as_color(&self) -> Result<Vec4> {
        match self {
            ParameterValue::Color(v) => Ok(*v),
            ParameterValue::Vec4(v) => Ok(*v),
            _ => Err(EffectError::InvalidParameter(
                "Expected color parameter".to_string(),
            )),
        }
    }
    
    pub fn as_vec2(&self) -> Result<Vec2> {
        match self {
            ParameterValue::Vec2(v) => Ok(*v),
            _ => Err(EffectError::InvalidParameter(
                "Expected Vec2 parameter".to_string(),
            )),
        }
    }
    
    pub fn as_vec3(&self) -> Result<Vec3> {
        match self {
            ParameterValue::Vec3(v) => Ok(*v),
            _ => Err(EffectError::InvalidParameter(
                "Expected Vec3 parameter".to_string(),
            )),
        }
    }
    
    pub fn as_vec4(&self) -> Result<Vec4> {
        match self {
            ParameterValue::Vec4(v) => Ok(*v),
            ParameterValue::Color(v) => Ok(*v),
            _ => Err(EffectError::InvalidParameter(
                "Expected Vec4 parameter".to_string(),
            )),
        }
    }
    
    pub fn as_string(&self) -> Result<&str> {
        match self {
            ParameterValue::String(v) => Ok(v),
            _ => Err(EffectError::InvalidParameter(
                "Expected string parameter".to_string(),
            )),
        }
    }
}

/// Keyframe for parameter animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: f64,
    pub value: ParameterValue,
    pub interpolation: InterpolationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpolationType {
    Linear,
    Cubic,
    Hold,
    EaseIn,
    EaseOut,
    EaseInOut,
}

/// Manages keyframes for a single parameter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyframeTrack {
    keyframes: Vec<Keyframe>,
}

impl KeyframeTrack {
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
        }
    }
    
    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        // Insert keyframe in sorted order by time
        let pos = self
            .keyframes
            .binary_search_by(|k| k.time.partial_cmp(&keyframe.time).unwrap())
            .unwrap_or_else(|e| e);
        
        // Replace if keyframe at same time exists
        if pos < self.keyframes.len() && self.keyframes[pos].time == keyframe.time {
            self.keyframes[pos] = keyframe;
        } else {
            self.keyframes.insert(pos, keyframe);
        }
    }
    
    pub fn remove_keyframe(&mut self, time: f64) -> Result<()> {
        if let Ok(pos) = self
            .keyframes
            .binary_search_by(|k| k.time.partial_cmp(&time).unwrap())
        {
            self.keyframes.remove(pos);
            Ok(())
        } else {
            Err(EffectError::KeyframeError(
                "Keyframe not found at specified time".to_string(),
            ))
        }
    }
    
    pub fn interpolate(&self, time: f64) -> Option<ParameterValue> {
        if self.keyframes.is_empty() {
            return None;
        }
        
        // Find surrounding keyframes
        let mut prev_idx = None;
        let mut next_idx = None;
        
        for (i, keyframe) in self.keyframes.iter().enumerate() {
            if keyframe.time <= time {
                prev_idx = Some(i);
            }
            if keyframe.time >= time && next_idx.is_none() {
                next_idx = Some(i);
            }
        }
        
        match (prev_idx, next_idx) {
            (Some(i), Some(j)) if i == j => Some(self.keyframes[i].value.clone()),
            (Some(i), Some(j)) => {
                let prev = &self.keyframes[i];
                let next = &self.keyframes[j];
                let t = (time - prev.time) / (next.time - prev.time);
                
                Some(interpolate_values(
                    &prev.value,
                    &next.value,
                    t as f32,
                    prev.interpolation,
                ))
            }
            (Some(i), None) => Some(self.keyframes[i].value.clone()),
            (None, Some(j)) => Some(self.keyframes[j].value.clone()),
            _ => None,
        }
    }
    
    pub fn get_keyframes(&self) -> &[Keyframe] {
        &self.keyframes
    }
}

/// Parameter manager for effects with keyframe support
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParameterManager {
    values: HashMap<String, ParameterValue>,
    tracks: HashMap<String, KeyframeTrack>,
}

impl ParameterManager {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            tracks: HashMap::new(),
        }
    }
    
    pub fn set_parameter(&mut self, name: &str, value: ParameterValue) {
        self.values.insert(name.to_string(), value);
    }
    
    pub fn get_parameter(&self, name: &str) -> Option<&ParameterValue> {
        self.values.get(name)
    }
    
    pub fn get_parameter_at_time(&self, name: &str, time: f64) -> Option<ParameterValue> {
        // Check if there's a keyframe track
        if let Some(track) = self.tracks.get(name) {
            if let Some(interpolated) = track.interpolate(time) {
                return Some(interpolated);
            }
        }
        
        // Fall back to static value
        self.values.get(name).cloned()
    }
    
    pub fn add_keyframe(&mut self, name: &str, keyframe: Keyframe) {
        self.tracks
            .entry(name.to_string())
            .or_insert_with(KeyframeTrack::new)
            .add_keyframe(keyframe);
    }
    
    pub fn remove_keyframe(&mut self, name: &str, time: f64) -> Result<()> {
        if let Some(track) = self.tracks.get_mut(name) {
            track.remove_keyframe(time)
        } else {
            Err(EffectError::KeyframeError(
                "No keyframe track for parameter".to_string(),
            ))
        }
    }
    
    pub fn get_keyframes(&self, name: &str) -> Vec<(f64, ParameterValue)> {
        self.tracks
            .get(name)
            .map(|track| {
                track
                    .get_keyframes()
                    .iter()
                    .map(|kf| (kf.time, kf.value.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Interpolate between two parameter values
fn interpolate_values(
    from: &ParameterValue,
    to: &ParameterValue,
    t: f32,
    interpolation: InterpolationType,
) -> ParameterValue {
    let t = match interpolation {
        InterpolationType::Linear => t,
        InterpolationType::Cubic => t * t * (3.0 - 2.0 * t),
        InterpolationType::Hold => 0.0,
        InterpolationType::EaseIn => t * t,
        InterpolationType::EaseOut => t * (2.0 - t),
        InterpolationType::EaseInOut => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                -1.0 + (4.0 - 2.0 * t) * t
            }
        }
    };
    
    match (from, to) {
        (ParameterValue::Float(a), ParameterValue::Float(b)) => {
            ParameterValue::Float(a + (b - a) * t)
        }
        (ParameterValue::Int(a), ParameterValue::Int(b)) => {
            ParameterValue::Int((a + ((b - a) as f32 * t) as i32))
        }
        (ParameterValue::Vec2(a), ParameterValue::Vec2(b)) => {
            ParameterValue::Vec2(Vec2::lerp(*a, *b, t))
        }
        (ParameterValue::Vec3(a), ParameterValue::Vec3(b)) => {
            ParameterValue::Vec3(Vec3::lerp(*a, *b, t))
        }
        (ParameterValue::Vec4(a), ParameterValue::Vec4(b)) => {
            ParameterValue::Vec4(Vec4::lerp(*a, *b, t))
        }
        (ParameterValue::Color(a), ParameterValue::Color(b)) => {
            ParameterValue::Color(Vec4::lerp(*a, *b, t))
        }
        _ => {
            // For non-interpolatable types, use hold behavior
            if t < 0.5 {
                from.clone()
            } else {
                to.clone()
            }
        }
    }
}