#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use crate::{ColorGradient, ConicalGradient, LayerOffset, SolidColor};

#[pymethods]
impl ConicalGradient {
    #[new]
    #[pyo3(
            text_signature = "(colors: ColorGradient, center: Offset, angle: float = 0) -> ConicalGradient",
            signature = (colors, center, angle = None)
    )]
    pub fn new_py(colors: ColorGradient, center: LayerOffset, angle: Option<f32>) -> Self {
        Self::new(colors, center, angle)
    }

    /// The starting angle (in degrees) of the gradient.
    ///
    /// A ``0`` degree angle is at 3 o'clock. This angle increases counter-clockwise.
    #[getter(angle)]
    pub fn get_angle_py(&self) -> f32 {
        self.get_angle()
    }

    #[setter(angle)]
    pub fn set_angle_py(&mut self, val: f32) {
        self.set_angle(val);
    }

    /// A helper function to `Generator.render()` behavior.
    ///
    /// The given ``x`` and ``y`` values (unsigned) are the coordinate of the pixel in the `Layer`
    /// (relative to `Layer.offset`).
    #[pyo3(text_signature = "(x: int, y: int) -> SolidColor", signature = (x, y), name = "get_color_at")]
    pub fn get_color_at_py(&self, x: u32, y: u32) -> SolidColor {
        self.get_color_at(x, y)
    }
}
