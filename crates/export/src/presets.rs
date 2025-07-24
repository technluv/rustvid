//! Export presets for various platforms and use cases

use crate::{ExportSettings, ExportFormat, Quality};
use std::path::PathBuf;

/// Export preset for common platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportPreset {
    // Video platforms
    YouTube1080p,
    YouTube4K,
    YouTubeShorts,
    Vimeo,
    Twitter,
    Instagram,
    TikTok,
    
    // Professional formats
    ProRes422,
    ProRes4444,
    DNxHD,
    
    // Web optimized
    WebMP4,
    WebM,
    
    // Mobile optimized
    MobileHigh,
    MobileLow,
    
    // Custom
    Custom,
}

impl ExportPreset {
    /// Get the display name for the preset
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::YouTube1080p => "YouTube 1080p",
            Self::YouTube4K => "YouTube 4K",
            Self::YouTubeShorts => "YouTube Shorts",
            Self::Vimeo => "Vimeo",
            Self::Twitter => "Twitter/X",
            Self::Instagram => "Instagram",
            Self::TikTok => "TikTok",
            Self::ProRes422 => "ProRes 422",
            Self::ProRes4444 => "ProRes 4444",
            Self::DNxHD => "DNxHD",
            Self::WebMP4 => "Web MP4",
            Self::WebM => "WebM",
            Self::MobileHigh => "Mobile (High Quality)",
            Self::MobileLow => "Mobile (Low Quality)",
            Self::Custom => "Custom",
        }
    }
    
    /// Get the description for the preset
    pub fn description(&self) -> &'static str {
        match self {
            Self::YouTube1080p => "Optimized for YouTube 1080p uploads",
            Self::YouTube4K => "Optimized for YouTube 4K uploads",
            Self::YouTubeShorts => "Vertical format for YouTube Shorts (9:16)",
            Self::Vimeo => "High quality for Vimeo uploads",
            Self::Twitter => "Optimized for Twitter/X video posts",
            Self::Instagram => "Square format for Instagram posts",
            Self::TikTok => "Vertical format for TikTok",
            Self::ProRes422 => "Apple ProRes 422 for professional editing",
            Self::ProRes4444 => "Apple ProRes 4444 with alpha channel",
            Self::DNxHD => "Avid DNxHD for broadcast",
            Self::WebMP4 => "H.264 MP4 optimized for web streaming",
            Self::WebM => "VP9 WebM for modern browsers",
            Self::MobileHigh => "High quality for mobile devices",
            Self::MobileLow => "Low bandwidth for mobile networks",
            Self::Custom => "Custom export settings",
        }
    }
    
    /// Convert preset to export settings
    pub fn to_settings(&self, output_path: PathBuf) -> ExportSettings {
        match self {
            Self::YouTube1080p => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h264".to_string(),
                audio_codec: "aac".to_string(),
                width: 1920,
                height: 1080,
                fps: 30.0,
                bitrate: 8_000_000, // 8 Mbps
                quality: Quality::High,
            },
            
            Self::YouTube4K => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h265".to_string(), // H.265 for 4K
                audio_codec: "aac".to_string(),
                width: 3840,
                height: 2160,
                fps: 30.0,
                bitrate: 40_000_000, // 40 Mbps
                quality: Quality::Ultra,
            },
            
            Self::YouTubeShorts => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h264".to_string(),
                audio_codec: "aac".to_string(),
                width: 1080,
                height: 1920, // 9:16 vertical
                fps: 30.0,
                bitrate: 10_000_000, // 10 Mbps
                quality: Quality::High,
            },
            
            Self::Vimeo => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h264".to_string(),
                audio_codec: "aac".to_string(),
                width: 1920,
                height: 1080,
                fps: 30.0,
                bitrate: 10_000_000, // 10 Mbps (Vimeo recommends higher quality)
                quality: Quality::Ultra,
            },
            
            Self::Twitter => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h264".to_string(),
                audio_codec: "aac".to_string(),
                width: 1280,
                height: 720,
                fps: 30.0,
                bitrate: 5_000_000, // 5 Mbps
                quality: Quality::High,
            },
            
            Self::Instagram => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h264".to_string(),
                audio_codec: "aac".to_string(),
                width: 1080,
                height: 1080, // Square format
                fps: 30.0,
                bitrate: 8_000_000, // 8 Mbps
                quality: Quality::High,
            },
            
            Self::TikTok => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h264".to_string(),
                audio_codec: "aac".to_string(),
                width: 1080,
                height: 1920, // 9:16 vertical
                fps: 30.0,
                bitrate: 8_000_000, // 8 Mbps
                quality: Quality::High,
            },
            
            Self::ProRes422 => ExportSettings {
                output_path,
                format: ExportFormat::Mov,
                video_codec: "prores_ks".to_string(),
                audio_codec: "pcm_s16le".to_string(),
                width: 1920,
                height: 1080,
                fps: 30.0,
                bitrate: 147_000_000, // ~147 Mbps for ProRes 422
                quality: Quality::Ultra,
            },
            
            Self::ProRes4444 => ExportSettings {
                output_path,
                format: ExportFormat::Mov,
                video_codec: "prores_ks".to_string(),
                audio_codec: "pcm_s16le".to_string(),
                width: 1920,
                height: 1080,
                fps: 30.0,
                bitrate: 330_000_000, // ~330 Mbps for ProRes 4444
                quality: Quality::Ultra,
            },
            
            Self::DNxHD => ExportSettings {
                output_path,
                format: ExportFormat::Mov,
                video_codec: "dnxhd".to_string(),
                audio_codec: "pcm_s16le".to_string(),
                width: 1920,
                height: 1080,
                fps: 30.0,
                bitrate: 145_000_000, // DNxHD 145
                quality: Quality::Ultra,
            },
            
            Self::WebMP4 => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h264".to_string(),
                audio_codec: "aac".to_string(),
                width: 1280,
                height: 720,
                fps: 30.0,
                bitrate: 2_500_000, // 2.5 Mbps
                quality: Quality::Medium,
            },
            
            Self::WebM => ExportSettings {
                output_path,
                format: ExportFormat::Webm,
                video_codec: "vp9".to_string(),
                audio_codec: "opus".to_string(),
                width: 1280,
                height: 720,
                fps: 30.0,
                bitrate: 2_000_000, // 2 Mbps
                quality: Quality::Medium,
            },
            
            Self::MobileHigh => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h264".to_string(),
                audio_codec: "aac".to_string(),
                width: 1280,
                height: 720,
                fps: 30.0,
                bitrate: 3_000_000, // 3 Mbps
                quality: Quality::High,
            },
            
            Self::MobileLow => ExportSettings {
                output_path,
                format: ExportFormat::Mp4,
                video_codec: "h264".to_string(),
                audio_codec: "aac".to_string(),
                width: 854,
                height: 480,
                fps: 30.0,
                bitrate: 1_000_000, // 1 Mbps
                quality: Quality::Low,
            },
            
            Self::Custom => ExportSettings::default(),
        }
    }
    
    /// Get recommended file extension for the preset
    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::ProRes422 | Self::ProRes4444 | Self::DNxHD => "mov",
            Self::WebM => "webm",
            _ => "mp4",
        }
    }
    
    /// Get all available presets
    pub fn all() -> Vec<Self> {
        vec![
            Self::YouTube1080p,
            Self::YouTube4K,
            Self::YouTubeShorts,
            Self::Vimeo,
            Self::Twitter,
            Self::Instagram,
            Self::TikTok,
            Self::ProRes422,
            Self::ProRes4444,
            Self::DNxHD,
            Self::WebMP4,
            Self::WebM,
            Self::MobileHigh,
            Self::MobileLow,
            Self::Custom,
        ]
    }
    
    /// Get presets grouped by category
    pub fn by_category() -> Vec<(&'static str, Vec<Self>)> {
        vec![
            ("Video Platforms", vec![
                Self::YouTube1080p,
                Self::YouTube4K,
                Self::YouTubeShorts,
                Self::Vimeo,
            ]),
            ("Social Media", vec![
                Self::Twitter,
                Self::Instagram,
                Self::TikTok,
            ]),
            ("Professional", vec![
                Self::ProRes422,
                Self::ProRes4444,
                Self::DNxHD,
            ]),
            ("Web & Mobile", vec![
                Self::WebMP4,
                Self::WebM,
                Self::MobileHigh,
                Self::MobileLow,
            ]),
        ]
    }
}

/// Builder for custom export settings
pub struct ExportSettingsBuilder {
    settings: ExportSettings,
}

impl ExportSettingsBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self {
            settings: ExportSettings::default(),
        }
    }
    
    /// Create a builder from a preset
    pub fn from_preset(preset: ExportPreset) -> Self {
        Self {
            settings: preset.to_settings(PathBuf::from("output.mp4")),
        }
    }
    
    pub fn output_path(mut self, path: PathBuf) -> Self {
        self.settings.output_path = path;
        self
    }
    
    pub fn format(mut self, format: ExportFormat) -> Self {
        self.settings.format = format;
        self
    }
    
    pub fn video_codec(mut self, codec: impl Into<String>) -> Self {
        self.settings.video_codec = codec.into();
        self
    }
    
    pub fn audio_codec(mut self, codec: impl Into<String>) -> Self {
        self.settings.audio_codec = codec.into();
        self
    }
    
    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        self.settings.width = width;
        self.settings.height = height;
        self
    }
    
    pub fn fps(mut self, fps: f32) -> Self {
        self.settings.fps = fps;
        self
    }
    
    pub fn bitrate(mut self, bitrate: u32) -> Self {
        self.settings.bitrate = bitrate;
        self
    }
    
    pub fn quality(mut self, quality: Quality) -> Self {
        self.settings.quality = quality;
        self
    }
    
    pub fn build(self) -> ExportSettings {
        self.settings
    }
}

impl Default for ExportSettingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_youtube_preset() {
        let preset = ExportPreset::YouTube1080p;
        let settings = preset.to_settings(PathBuf::from("test.mp4"));
        
        assert_eq!(settings.width, 1920);
        assert_eq!(settings.height, 1080);
        assert_eq!(settings.video_codec, "h264");
        assert_eq!(settings.bitrate, 8_000_000);
    }
    
    #[test]
    fn test_vertical_presets() {
        let shorts = ExportPreset::YouTubeShorts.to_settings(PathBuf::from("shorts.mp4"));
        assert_eq!(shorts.width, 1080);
        assert_eq!(shorts.height, 1920);
        
        let tiktok = ExportPreset::TikTok.to_settings(PathBuf::from("tiktok.mp4"));
        assert_eq!(tiktok.width, 1080);
        assert_eq!(tiktok.height, 1920);
    }
    
    #[test]
    fn test_professional_presets() {
        let prores = ExportPreset::ProRes422.to_settings(PathBuf::from("prores.mov"));
        assert_eq!(prores.video_codec, "prores_ks");
        assert_eq!(prores.audio_codec, "pcm_s16le");
        assert!(prores.bitrate > 100_000_000);
    }
    
    #[test]
    fn test_settings_builder() {
        let settings = ExportSettingsBuilder::new()
            .resolution(1280, 720)
            .fps(60.0)
            .bitrate(5_000_000)
            .quality(Quality::High)
            .video_codec("h265")
            .build();
        
        assert_eq!(settings.width, 1280);
        assert_eq!(settings.height, 720);
        assert_eq!(settings.fps, 60.0);
        assert_eq!(settings.bitrate, 5_000_000);
        assert_eq!(settings.video_codec, "h265");
    }
    
    #[test]
    fn test_preset_categories() {
        let categories = ExportPreset::by_category();
        assert!(!categories.is_empty());
        
        // Check that all presets are categorized
        let all_presets = ExportPreset::all();
        let categorized_count: usize = categories.iter()
            .map(|(_, presets)| presets.len())
            .sum();
        
        // Subtract 1 for Custom preset which might not be in categories
        assert!(categorized_count >= all_presets.len() - 1);
    }
}