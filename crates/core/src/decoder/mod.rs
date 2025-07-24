//! Video decoder implementations

pub mod ffmpeg;

pub use ffmpeg::FFmpegDecoder;

#[cfg(test)]
mod tests;