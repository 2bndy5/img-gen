#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo
)]

pub mod validators;
pub use validators::{
    Background, Border, ColorGradient, ColorKind, ConicalGradient, Corners, Debug, Ellipse, Font,
    HEIGHT, Icon, IrregularPolygonSides, Layer, LayerOffset, Layout, Line, LineHeight,
    LinearGradient, Mask, Polygon, PolygonSides, PreserveAspect, Presets, RadialGradient,
    Rectangle, RegularPolygonSides, Size, SolidColor, Spread, Typography, TypographyAlign, WIDTH,
    Weight,
};

pub mod error;
pub use error::{ImgGenSpecError, Result};

#[cfg(feature = "pyo3")]
mod python_binding;
