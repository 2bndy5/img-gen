use super::{ColorKind, PreserveAspect};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

/// An attribute to describe a [`Layer`](struct@crate::Layer)'s
/// [`Background`].
///
/// See [`LayerAttrKind::Background`](type@crate::LayerAttrKind::Background)
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Background {
    /// A path to an image file.
    ///
    /// If the given image path does not exist or has no file extension, then
    /// it will be treated as an SVG image.
    ///
    /// This also supports built-in SVG icons from the following icon packs:
    ///
    /// - Material Design Icons (``material/{icon_slug}``)
    /// - Simple Icons (``simple/{icon_slug}``)
    /// - Octicons (``octicons/{icon_slug}``)
    /// - Font Awesome Free (``fontawesome/{icon_slug}``)
    pub image: Option<String>,
    /// A color overlaid on top of the `image`.
    /// If no image is specified, then the layer is simple filled with this color.
    pub color: Option<ColorKind>,
    /// This controls how the original image is rendered into the layer.
    /// Default is to preserve the original image's width and height.
    #[serde(default)]
    pub preserve_aspect: PreserveAspect,
}
