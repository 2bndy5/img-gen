use super::{Border, ColorKind};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::{Deserialize, Serialize};

/// An enum to represent the possible options in specifying which [`Rectangle::corners`] to render rounded.
#[cfg_attr(
    feature = "pyo3",
    pyclass(eq, eq_int, module = "img_gen", from_py_object)
)]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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
    /// All rectangle corners in display order.
    pub const ALL: [Self; 4] = [
        Corners::TopLeft,
        Corners::TopRight,
        Corners::BottomLeft,
        Corners::BottomRight,
    ];
}

/// An attribute to represent a rectangle rendered in the layer.
///
/// The size of the rectangle is specified by the layer's size.
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Rectangle {
    /// The [`Border`] (if specified) to render around the rectangle.
    pub border: Option<Border>,

    /// The color used to fill the rectangle.
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

    /// The radius of the rendered [`Rectangle::corners`].
    ///
    /// The renderer shall limit this value if it is
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
