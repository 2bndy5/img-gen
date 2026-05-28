#[derive(Debug, thiserror::Error)]
pub enum ImgGenSpecError {
    // Validation.
    #[error("Failed to parse Corner identifier from string: {value}")]
    InvalidCornerIdentifier { value: String },
    #[error("Layer height cannot be zero")]
    InvalidLayerHeight,
    #[error("Value '{value}' is not a valid CSS color ({reason})")]
    InvalidCssColor { value: String, reason: String },
    #[error("Failed to build color gradient: {reason}")]
    InvalidGradientSpec { reason: String },
}

pub type Result<T> = std::result::Result<T, ImgGenSpecError>;
