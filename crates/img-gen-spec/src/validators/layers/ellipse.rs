#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::{Deserialize, Serialize};

use super::{Border, ColorKind};

/// An attribute to represent an Arc rendered in the layer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
pub struct Arc {
    /// The starting angle of the arc in degrees.
    pub start: f32,
    /// The ending angle of the arc in degrees.
    pub end: f32,
}

/// An attribute to represent an Ellipse rendered in the layer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
pub struct Ellipse {
    /// The [`Border`] (if specified) ro render around the ellipse.
    pub border: Option<Border>,
    /// The color used to fill the ellipse.
    #[serde(
        default = "ColorKind::transparent_default",
        alias = "linear_gradient",
        alias = "radial_gradient",
        alias = "conical_gradient",
        alias = "linear-gradient",
        alias = "radial-gradient",
        alias = "conical-gradient"
    )]
    pub color: ColorKind,
    /// The Arc (if specified) to render instead of a full ellipse.
    pub arc: Option<Arc>,
    /// Extend the border of an arc to the center of the ellipse, creating a pie shape.
    ///
    /// Does not affect full ellipses, and is ignored if `arc` is not specified.
    #[serde(default, alias = "border-to-origin")]
    pub border_to_origin: bool,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn deserialize_default_ellipse() {
        let json = r#"{}"#;
        let ellipse: Ellipse = serde_json::from_str(json).unwrap();
        assert_eq!(ellipse.color.get_color_tuple_at(0, 0), (0, 0, 0, 0));
        assert!(ellipse.border.is_none());
        assert!(ellipse.arc.is_none());
        assert!(!ellipse.border_to_origin);
    }
}
