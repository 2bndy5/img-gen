#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

mod deserializing;
mod solid;

pub use solid::SolidColor;
mod gradients;
pub use gradients::{
    ColorGradient, ConicalGradient, LinearGradient, Presets, RadialGradient, Spread,
};

/// An enum to describe the possible kinds of colors used when rendering various
/// [`Layer`](struct@crate::Layer) attributes/properties.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Debug, Clone)]
pub enum ColorKind {
    /// A kind of [`LinearGradient`].
    LinearGradient(LinearGradient),

    /// A kind of [`RadialGradient`].
    RadialGradient(RadialGradient),

    /// A kind of [`ConicalGradient`].
    ConicalGradient(ConicalGradient),

    /// A kind of [`SolidColor`].
    SolidColor(SolidColor),
}

impl From<SolidColor> for ColorKind {
    fn from(val: SolidColor) -> Self {
        ColorKind::SolidColor(val)
    }
}
impl From<LinearGradient> for ColorKind {
    fn from(val: LinearGradient) -> Self {
        ColorKind::LinearGradient(val)
    }
}
impl From<RadialGradient> for ColorKind {
    fn from(val: RadialGradient) -> Self {
        ColorKind::RadialGradient(val)
    }
}
impl From<ConicalGradient> for ColorKind {
    fn from(val: ConicalGradient) -> Self {
        ColorKind::ConicalGradient(val)
    }
}

impl Default for ColorKind {
    fn default() -> Self {
        Self::SolidColor(SolidColor::default())
    }
}

impl ColorKind {
    pub fn get_color_tuple_at(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        match self {
            ColorKind::SolidColor(solid) => solid.to_tuple(),
            ColorKind::LinearGradient(gradient) => gradient.get_color_at(x, y).to_tuple(),
            ColorKind::RadialGradient(gradient) => gradient.get_color_at(x, y).to_tuple(),
            ColorKind::ConicalGradient(gradient) => gradient.get_color_at(x, y).to_tuple(),
        }
    }
}
