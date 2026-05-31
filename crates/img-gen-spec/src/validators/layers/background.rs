use super::{ColorKind, PreserveAspect};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::{Deserialize, Serialize};

/// An attribute to describe a [`Layer`](struct@crate::Layer)'s
/// ``background``.
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Background {
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
    /// ``external_resource_paths`` provided to the ``Generator`` (in `img_gen_renderer` crate),
    /// which defaults to the current working directory if unspecified or an empty list.
    pub image: Option<String>,
    /// A color overlaid on top of the `image`.
    /// If no image is specified, then the layer is simple filled with this color.
    pub color: Option<ColorKind>,
    /// This controls how the original image is rendered into the layer.
    /// Default is to preserve the original image's width and height.
    #[serde(default)]
    pub preserve_aspect: PreserveAspect,
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::Background;

    #[test]
    fn duplicate_color_last_wins() {
        let yaml = "color: red\ncolor: blue\n";
        let opts = serde_saphyr::options! {
            duplicate_keys: serde_saphyr::options::DuplicateKeyPolicy::LastWins,
        };
        // Parse into an intermediate `serde_json::Value` with LastWins, then deserialize
        let value: serde_json::Value = serde_saphyr::from_str_with_options(yaml, opts).unwrap();
        let bg: Background = serde_json::from_value(value).unwrap();
        assert!(bg.color.is_some());
        match bg.color.unwrap() {
            super::ColorKind::SolidColor(sc) => assert_eq!(sc.get_b(), 255u8),
            other => panic!("unexpected color kind: {:?}", other),
        }
    }
}
