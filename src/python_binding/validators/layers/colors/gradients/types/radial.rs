use pyo3::prelude::*;

use crate::{ColorGradient, RadialGradient, SolidColor, Spread, validators::LayerOffset};

#[pymethods]
impl RadialGradient {
    #[new]
    #[pyo3(
        text_signature = "(colors: ColorGradient, center: Offset, radius: float, focal_point: Offset | None = None, focal_radius: float | None = None, spread: Spread = Spread.Pad) -> RadialGradient",
        signature = (colors, center, radius, focal_point = None, focal_radius = None, spread=Some(Spread::Pad))
    )]
    pub fn new_py(
        colors: ColorGradient,
        center: LayerOffset,
        radius: f32,
        focal_point: Option<LayerOffset>,
        focal_radius: Option<f32>,
        spread: Option<Spread>,
    ) -> Self {
        Self::new(colors, center, radius, focal_point, focal_radius, spread)
    }

    /// The center (`Offset`) of the gradient (relative to the `Layer.offset`).
    #[getter(center)]
    pub fn get_center_py(&self) -> LayerOffset {
        self.get_center()
    }

    #[setter(center)]
    pub fn set_center_py(&mut self, val: LayerOffset) {
        self.set_center(val);
    }

    /// The radius of the gradient.
    #[getter(radius)]
    pub fn get_radius_py(&self) -> f32 {
        self.get_radius()
    }

    #[setter(radius)]
    pub fn set_radius_py(&mut self, val: f32) {
        self.set_radius(val);
    }

    /// The focal point (`Offset`)  of the gradient (relative to the `Layer.offset`).
    #[getter(focal_point)]
    pub fn get_focal_point_py(&self) -> LayerOffset {
        self.get_focal_point()
    }

    #[setter(focal_point)]
    pub fn set_focal_point_py(&mut self, val: LayerOffset) {
        self.set_focal_point(val);
    }

    #[getter(focal_radius)]
    pub fn get_focal_radius_py(&self) -> f32 {
        self.get_focal_radius()
    }

    #[setter(focal_radius)]
    pub fn set_focal_radius_py(&mut self, val: f32) {
        self.set_focal_radius(val);
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
