#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

pub mod font;
pub mod line;

use super::{Border, ColorKind};
pub use font::{Font, Weight};
pub use line::{Line, LineHeight};

/// A enumeration of the possible alignment options for the text in a [`Typography`] attribute.
#[cfg_attr(
    feature = "pyo3",
    pyclass(eq, eq_int, module = "img_gen", from_py_object)
)]
#[derive(Debug, PartialEq, Clone, Default, Deserialize)]
pub enum TypographyAlign {
    #[serde(alias = "start top")]
    #[default]
    StartTop,
    #[serde(alias = "start center")]
    StartCenter,
    #[serde(alias = "start bottom")]
    StartBottom,
    #[serde(alias = "center top")]
    CenterTop,
    #[serde(alias = "center")]
    Center,
    #[serde(alias = "center center")]
    CenterCenter,
    #[serde(alias = "center bottom")]
    CenterBottom,
    #[serde(alias = "end top")]
    EndTop,
    #[serde(alias = "end center")]
    EndCenter,
    #[serde(alias = "end bottom")]
    EndBottom,
}

/// An attribute to represent a [`Layer`](struct@crate::Layer)'s rendered text.
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Typography {
    /// The text content.
    pub content: String,

    /// The alignment of the text.
    #[serde(default = "TypographyAlign::default")]
    pub align: TypographyAlign,

    /// The color used to fill the rendered text.
    #[serde(default)]
    pub color: ColorKind,

    /// The `Line` property used to describe the text's size.
    #[serde(default = "Line::default")]
    pub line: Line,

    /// Controls how text that doesn't fit within the layer's bounds is handled.
    ///
    /// - `true`: the font size is progressively reduced until all text fits within the layer.
    /// - `false`: text wraps within the layer's horizontal boundary; any text that still
    ///   overflows is replaced with a trailing ellipsis (`…`).
    #[serde(default)]
    pub overflow: bool,

    /// The `Font` property used to describe the font used to render the text.
    #[serde(default = "Font::default")]
    pub font: Font,

    /// The border (if specified) to draw around the rendered glyphs of the `content`.
    #[serde(default)]
    pub border: Option<Border>,
}
