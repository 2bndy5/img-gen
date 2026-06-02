use std::num::NonZeroU32;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::{Deserialize, Serialize};

/// A layer attribute to describe the layer's offset (from top left corner).
#[cfg_attr(
    feature = "pyo3",
    pyclass(name = "Offset", module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct LayerOffset {
    /// The offset in pixels along the x-axis.
    #[serde(default)]
    pub x: i32,

    /// The offset in pixels along the y-axis.
    #[serde(default)]
    pub y: i32,
}

/// The default value used for a [`Size`]'s `width`.
pub const WIDTH: NonZeroU32 = NonZeroU32::new(1200).unwrap();

/// The default value used for a [`Size`]'s `height`.
pub const HEIGHT: NonZeroU32 = NonZeroU32::new(630).unwrap();

/// A layer attribute to describe the layer's size.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
    /// The size in pixels along the x-axis.
    pub width: Option<NonZeroU32>,
    /// The size in pixels along the y-axis.
    pub height: Option<NonZeroU32>,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: Some(WIDTH),
            height: Some(HEIGHT),
        }
    }
}
