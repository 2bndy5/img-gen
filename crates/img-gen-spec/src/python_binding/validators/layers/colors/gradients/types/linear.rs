use pyo3::prelude::*;

use crate::{ColorGradient, LayerOffset, LinearGradient, SolidColor, Spread};

#[pymethods]
impl LinearGradient {
    /// Creates a linear gradient from ``colors``, ``start``, ``end``, and optional ``spread``.
    #[new]
    #[pyo3(
        text_signature = "(colors: ColorGradient, start: Offset, end: Offset, spread: Spread = Spread.Pad) -> LinearGradient",
        signature = (colors, start, end, spread=Some(Spread::Pad))
    )]
    pub fn new_py(
        colors: ColorGradient,
        start: LayerOffset,
        end: LayerOffset,
        spread: Option<Spread>,
    ) -> Self {
        Self::new(colors, start, end, spread)
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
