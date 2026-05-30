//! Validation types and error handling for image generation specifications.
//!
//! This crate defines the public data model shared by the renderer and higher-level API.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    missing_docs
)]

/// Public validation types used to describe layouts, layers, and styling.
pub mod validators;
pub use validators::{
    Arc, Background, Border, ColorGradient, ColorKind, ConicalGradient, Corners, Debug, Ellipse,
    Font, HEIGHT, Icon, IrregularPolygonSides, Layer, LayerOffset, Layout, Line, LineHeight,
    LinearGradient, Mask, Polygon, PolygonSides, PreserveAspect, Presets, RadialGradient,
    Rectangle, RegularPolygonSides, Size, SolidColor, Spread, TRANSPARENT, Typography,
    TypographyAlign, WIDTH, Weight,
};

/// Error types returned while parsing or validating image generation specifications.
pub mod error;
pub use error::{ImgGenSpecError, Result};

#[cfg(feature = "pyo3")]
mod python_binding;
