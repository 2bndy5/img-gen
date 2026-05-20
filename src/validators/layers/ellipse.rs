#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

use super::{Border, ColorKind};

/// An attribute to represent an Ellipse rendered in the layer.
#[derive(Debug, Clone, Default, Deserialize)]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
pub struct Ellipse {
    /// The [`Border`] (if specified) ro render around the ellipse.
    pub border: Option<Border>,
    /// The color used to fill the ellipse.
    #[serde(default)]
    pub color: ColorKind,
}
