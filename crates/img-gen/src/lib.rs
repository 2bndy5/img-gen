#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo
)]

pub use img_gen_renderer::{Generator, Image, ImgGenRendererError, Result};
pub use img_gen_spec::{
    Arc, Background, Border, ColorGradient, ColorKind, ConicalGradient, Corners, Debug, Ellipse,
    Font, HEIGHT, Icon, IrregularPolygonSides, Layer, LayerOffset, Layout, Line, LineHeight,
    LinearGradient, Mask, Polygon, PolygonSides, PreserveAspect, Presets, RadialGradient,
    Rectangle, RegularPolygonSides, Size, SolidColor, Spread, TRANSPARENT, Typography,
    TypographyAlign, WIDTH, Weight,
};

#[cfg(feature = "pyo3")]
mod python_binding;
