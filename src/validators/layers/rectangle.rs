use std::fmt;

use super::{Border, ColorKind};
use crate::{ImgGenError, Result};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

/// An enum to represent the possible options in specifying which [`Rectangle::corners`] to render rounded.
#[cfg_attr(
    feature = "pyo3",
    pyclass(eq, eq_int, module = "img_gen", from_py_object)
)]
#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum Corners {
    /// The ``"top left"`` corner of the `Rectangle`.
    #[serde(alias = "top left")]
    TopLeft,

    /// The ``"top right"`` corner of the `Rectangle`.
    #[serde(alias = "top right")]
    TopRight,

    /// The ``"bottom left"`` corner of the `Rectangle`.
    #[serde(alias = "bottom left")]
    BottomLeft,

    /// The ``"bottom right"`` corner of the `Rectangle`.
    #[serde(alias = "bottom right")]
    BottomRight,
}

impl Corners {
    pub fn from_string(val: &str) -> Result<Self> {
        match val.to_lowercase().as_str() {
            "top left" => Ok(Corners::TopLeft),
            "top right" => Ok(Corners::TopRight),
            "bottom left" => Ok(Corners::BottomLeft),
            "bottom right" => Ok(Corners::BottomRight),
            _ => Err(ImgGenError::InvalidCornerIdentifier {
                value: val.to_string(),
            }),
        }
    }

    pub const ALL: [Self; 4] = [
        Corners::TopLeft,
        Corners::TopRight,
        Corners::BottomLeft,
        Corners::BottomRight,
    ];
}

impl fmt::Display for Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Corners::TopLeft => write!(f, "top left"),
            Corners::TopRight => write!(f, "top right"),
            Corners::BottomLeft => write!(f, "bottom left"),
            Corners::BottomRight => write!(f, "bottom right"),
        }
    }
}

/// An attribute to represent a rectangle rendered in the layer.
///
/// The size of the rectangle is specified by the layer's size.
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Rectangle {
    /// The [`Border`] (if specified) to render around the rectangle.
    pub border: Option<Border>,

    /// The color used to fill the rectangle.
    #[serde(default)]
    pub color: ColorKind,

    /// The radius of the rendered [`Rectangle::corners`].
    ///
    /// The [`Generator`](struct@crate::Generator) will limit this value if it is
    /// greater than half the minimum of the rectangle's width or height
    /// (see [`Layer::size`](value@crate::Layer::size)).
    #[serde(default)]
    pub radius: f32,

    /// A list of the [`Corners`] in which the `radius` is applied.
    ///
    /// Any [`Corners`] not in this list will not be rounded.
    #[serde(default)]
    pub corners: Vec<Corners>,
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use super::Corners;

    #[test]
    fn test_str() {
        let str_values = ["top left", "top right", "bottom right", "bottom left"];
        for str_val in str_values {
            let val = Corners::from_string(str_val);
            assert!(val.is_ok());
            assert_eq!(val.unwrap().to_string(), str_val);
        }
        assert!(Corners::from_string("val").is_err());
    }
}
