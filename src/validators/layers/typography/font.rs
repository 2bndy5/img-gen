use std::path::PathBuf;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use parley::{FontFamily, FontStyle, FontWeight};
use serde::Deserialize;

/// An enumeration of the possible font weights.
#[cfg_attr(
    feature = "pyo3",
    pyclass(eq, eq_int, module = "img_gen", from_py_object)
)]
#[derive(Debug, PartialEq, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Weight {
    #[serde(alias = "100")]
    Thin = 100,
    #[serde(alias = "300")]
    Light = 300,
    #[serde(alias = "400")]
    #[default]
    Regular = 400,
    #[serde(alias = "500")]
    Medium = 500,
    #[serde(alias = "700")]
    Bold = 700,
    #[serde(alias = "900")]
    Black = 900,
}

impl Weight {
    pub(crate) const fn value(self) -> u16 {
        self as u16
    }
}

/// A property to implicitly describe the font used in a
/// [`Typography`](struct@super::Typography) attribute.
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Deserialize)]
pub struct Font {
    /// The font family's name.
    #[serde(default = "Font::default_font_family")]
    pub family: String,
    /// The font family's style.
    #[serde(default = "Font::default_font_style")]
    pub style: String,
    /// The font's `Weight`.
    #[serde(default)]
    pub weight: Weight,
    /// The font family's lingual subset.
    ///
    /// The valid options for this can vary depending on the chosen font `family`.
    #[serde(default = "Font::default_font_subset")]
    pub subset: String,
    /// An optional path to the font's ``.ttf`` file.
    pub path: Option<String>,
}

impl Font {
    pub(crate) fn font_family(&self) -> FontFamily<'_> {
        FontFamily::named(&self.family)
    }

    pub(crate) fn font_style(&self) -> FontStyle {
        FontStyle::parse_css(self.style.trim()).unwrap_or(FontStyle::Normal)
    }

    pub(crate) fn font_weight(&self) -> FontWeight {
        FontWeight::new(self.weight.value() as f32)
    }

    pub(crate) fn path_buf(&self) -> Option<PathBuf> {
        self.path.as_ref().map(PathBuf::from)
    }

    pub(crate) fn default_font_family() -> String {
        "Roboto".to_string()
    }

    pub(crate) fn default_font_style() -> String {
        "normal".to_string()
    }

    pub(crate) fn default_font_subset() -> String {
        "latin".to_string()
    }
}

impl Default for Font {
    fn default() -> Self {
        Font {
            family: Self::default_font_family(),
            style: Self::default_font_style(),
            weight: Weight::default(),
            subset: Self::default_font_subset(),
            path: None,
        }
    }
}
