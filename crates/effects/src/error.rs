use thiserror::Error;

#[derive(Error, Debug)]
pub enum EffectError {
    #[error("GPU error: {0}")]
    GpuError(String),
    
    #[error("Shader compilation error: {0}")]
    ShaderCompilationError(String),
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    #[error("Effect not found: {id}")]
    EffectNotFound { id: uuid::Uuid },
    
    #[error("Pipeline error: {0}")]
    PipelineError(String),
    
    #[error("Resource error: {0}")]
    ResourceError(String),
    
    #[error("Keyframe error: {0}")]
    KeyframeError(String),
    
    #[error("Transition error: {0}")]
    TransitionError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
    
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, EffectError>;