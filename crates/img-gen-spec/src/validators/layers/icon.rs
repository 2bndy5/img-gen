use super::{ColorKind, PreserveAspect};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

/// An attribute to describe a [`Layer`](struct@crate::Layer)'s [`Icon`].
///
/// See [`LayerAttrKind::Icon`](type@crate::LayerAttrKind::Icon).
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Deserialize)]
pub struct Icon {
    /// A path to an image file.
    pub image: String,
    /// A color used to replace the [`Icon::image`]'s original coloring.
    pub color: Option<ColorKind>,
    /// This controls how the original image is rendered into the layer.
    ///
    /// The default is to preserve the original image's width and height
    /// ([`PreserveAspect::On`]).
    #[serde(default)]
    pub preserve_aspect: PreserveAspect,
}
