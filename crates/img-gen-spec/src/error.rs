/// Errors produced while validating or parsing image generation specifications.
#[derive(Debug, thiserror::Error)]
pub enum ImgGenSpecError {
    /// Returned when a layer height would evaluate to zero.
    #[error("Layer height cannot be zero")]
    InvalidLayerHeight,

    /// Returned when a CSS color string cannot be parsed.
    #[error("Value '{value}' is not a valid CSS color ({reason})")]
    InvalidCssColor {
        /// The input value that failed to parse.
        value: String,
        /// The parser-provided failure reason.
        reason: String,
    },

    /// Returned when a gradient specification cannot be converted into a gradient.
    #[error("Failed to build color gradient: {reason}")]
    InvalidGradientSpec {
        /// The gradient builder failure reason.
        reason: String,
    },
}

/// A specialized result type for specification parsing and validation.
pub type Result<T> = std::result::Result<T, ImgGenSpecError>;
