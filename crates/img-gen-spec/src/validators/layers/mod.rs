mod background;
mod colors;
mod ellipse;
mod icon;
mod polygon;
mod rectangle;
mod size_offset;
mod typography;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::{Deserialize, Serialize};

use std::{fmt, num::NonZeroU32};

pub use background::Background;
pub use colors::{
    ColorGradient, ColorKind, ConicalGradient, LinearGradient, Presets, RadialGradient, SolidColor,
    Spread, TRANSPARENT,
};
pub use ellipse::{Arc, Ellipse};
pub use icon::Icon;
pub use polygon::{IrregularPolygonSides, Polygon, PolygonSides, RegularPolygonSides};
pub use rectangle::{Corners, Rectangle};
pub use size_offset::{HEIGHT, LayerOffset, Size, WIDTH};
pub use typography::{Font, Line, LineHeight, Typography, TypographyAlign, Weight};

/// A property to describe an attributes [`Border`].
///
/// See [`Polygon`], [`Rectangle`], [`Ellipse`], and [`Typography`].
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", set_all, get_all, from_py_object)
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
    /// The width (in pixels) used to draw the border.
    pub width: NonZeroU32,
    /// The color used to draw the border.
    #[serde(
        alias = "linear_gradient",
        alias = "radial_gradient",
        alias = "conical_gradient",
        alias = "linear-gradient",
        alias = "radial-gradient",
        alias = "conical-gradient"
    )]
    pub color: ColorKind,
}

/// The default border width in pixels.
pub const BORDER_WIDTH: NonZeroU32 = NonZeroU32::new(1).unwrap();

impl Default for Border {
    fn default() -> Self {
        Self {
            width: BORDER_WIDTH,
            color: ColorKind::default(),
        }
    }
}

/// An enum to describe the how to preserve an image's dimensions when portrayed in
/// an [`Icon`] or [`Background`] attribute.
#[cfg_attr(
    feature = "pyo3",
    pyclass(eq, eq_int, module = "img_gen", from_py_object)
)]
#[derive(Debug, PartialEq, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreserveAspect {
    /// Preserve the original image's aspect ratio with respect to both width and height.
    #[serde(alias = "true")]
    #[default]
    On,
    /// Ignore the original image's aspect ratio.
    /// Instead the image will be stretched to the layer's `Size`.
    #[serde(alias = "false")]
    Off,
    /// Preserve the original image's aspect ratio with respect to width.
    Width,
    /// Preserve the original image's aspect ratio with respect to height.
    Height,
}

impl PreserveAspect {
    /// Instantiate a [`PreserveAspect`] object from a string.
    ///
    /// Possible values include `"true"`, `"false"`, `"width"`, or `"height"`.
    /// If a given `value` does not match those values (case insensitive), then
    /// [`PreserveAspect::On`] is returned.
    pub fn from_string(value: &str) -> PreserveAspect {
        match value.to_lowercase().as_str() {
            "false" => PreserveAspect::Off,
            "width" => PreserveAspect::Width,
            "height" => PreserveAspect::Height,
            _ => PreserveAspect::On,
        }
    }
}

impl fmt::Display for PreserveAspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreserveAspect::On => write!(f, "true"),
            PreserveAspect::Off => write!(f, "false"),
            PreserveAspect::Width => write!(f, "width"),
            PreserveAspect::Height => write!(f, "height"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::PreserveAspect;

    #[test]
    fn test_str() {
        let str_values = ["true", "false", "width", "height"];
        for str_val in str_values {
            let val = PreserveAspect::from_string(str_val).to_string();
            assert_eq!(val, str_val);
        }
        assert_eq!(PreserveAspect::from_string("val").to_string(), "true");
    }
}
