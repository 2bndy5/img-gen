#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo
)]

mod generator;
pub use generator::{Generator, Image};

mod external_resources;

pub const DEFAULT_FONT_ROBOTO_REGULAR_400: &[u8] =
    include_bytes!("../assets/fonts/roboto-latin-400-normal.ttf");

pub mod validators;
pub use validators::{
    Background, Border, ColorGradient, ColorKind, ConicalGradient, Corners, Debug, Ellipse, Font,
    Icon, Layer, LayerOffset, Layout, Line, LineHeight, LinearGradient, Mask, Polygon,
    PolygonSides, PreserveAspect, Presets, RadialGradient, Rectangle, Size, SolidColor, Spread,
    Typography, TypographyAlign, Weight,
};

pub mod error;
pub use error::{ImgGenError, Result};

#[cfg(feature = "pyo3")]
mod python_binding;
