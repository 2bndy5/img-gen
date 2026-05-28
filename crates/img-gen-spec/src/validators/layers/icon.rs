use super::{ColorKind, PreserveAspect};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

/// An attribute to describe a [`Layer`](struct@crate::Layer)'s [`Icon`].
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Deserialize)]
pub struct Icon {
    /// A path to an image file.
    ///
    /// If the given image path has no file extension, then
    /// it will be treated as an SVG image.
    ///
    /// This also supports built-in SVG icons from the following icon packs:
    ///
    /// - Material Design Icons (``material/{icon_slug}``)
    /// - Simple Icons (``simple/{icon_slug}``)
    /// - Octicons (``octicons/{icon_slug}``)
    /// - FontAwesome Free (``fontawesome/<brands|solid|regular>/{icon_slug}``)
    ///
    /// Otherwise, the image file's path is resolved via a search through the list of
    /// ``image_search_paths`` provided to the ``Generator`` (in `img_gen_renderer` crate),
    /// which defaults to the current working directory if unspecified or an empty list.
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
