use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{
    ColorGradient, ColorKind, ConicalGradient, LayerOffset, LinearGradient, Presets,
    RadialGradient, SolidColor, Spread,
};

mod gradients;
mod solid;

#[pymethods]
impl ColorKind {
    /// Create a solid color from RGBA values.
    #[staticmethod]
    pub fn solid_color(r: i32, g: i32, b: i32, a: i32) -> Self {
        SolidColor::new(
            r.clamp(0, 255) as u8,
            g.clamp(0, 255) as u8,
            b.clamp(0, 255) as u8,
            a.clamp(0, 255) as u8,
        )
        .into()
    }

    /// Create a solid color from a CSS string.
    #[staticmethod]
    pub fn solid_color_from_str(color: &str) -> PyResult<Self> {
        match SolidColor::from_string(color) {
            Err(e) => Err(PyValueError::new_err(format!("{e:?}"))),
            Ok(c) => Ok(c.into()),
        }
    }

    /// Create a linear gradient from a list of color stops or a preset,
    /// along with a start and end offset.
    #[staticmethod]
    #[pyo3(
        text_signature = "(start: Offset, end: Offset, colors: list[tuple[float, str]] | None = None, preset: Presets | None = None, spread: Spread | None = None) -> ColorKind",
        signature = (start, end, colors=None, preset=None, spread=None)
    )]
    pub fn linear_gradient(
        start: LayerOffset,
        end: LayerOffset,
        colors: Option<Vec<(f32, String)>>,
        preset: Option<Presets>,
        spread: Option<Spread>,
    ) -> PyResult<Self> {
        ColorGradient::new_py(colors, preset)
            .map(|gradient| LinearGradient::new_py(gradient, start, end, spread).into())
    }

    /// Create radial gradient from a list of color stops or a preset,
    /// along with the center and radius of the gradient.
    #[staticmethod]
    #[pyo3(
        text_signature = "(center: Offset, radius: float, colors: list[tuple[float, str]] | None = None, preset: Presets | None = None, spread: Spread | None = None, focal_point: Offset | None = None, focal_radius: float | None = None) -> ColorKind",
        signature = (center, radius, colors=None, preset=None, spread=None, focal_point=None, focal_radius=None)
    )]
    pub fn radial_gradient(
        center: LayerOffset,
        radius: f32,
        colors: Option<Vec<(f32, String)>>,
        preset: Option<Presets>,
        spread: Option<Spread>,
        focal_point: Option<LayerOffset>,
        focal_radius: Option<f32>,
    ) -> PyResult<Self> {
        ColorGradient::new_py(colors, preset).map(|gradient| {
            RadialGradient::new(gradient, center, radius, focal_point, focal_radius, spread).into()
        })
    }

    /// Create a conical gradient from a list of color stops or a preset,
    /// along with the center and angle of the gradient.
    #[staticmethod]
    #[pyo3(
        text_signature = "(center: Offset, angle: float | None = None, colors: list[tuple[float, str]] | None = None, preset: Presets | None = None) -> ColorKind",
        signature = (center, angle=None, colors=None, preset=None)
    )]
    pub fn conical_gradient(
        center: LayerOffset,
        angle: Option<f32>,
        colors: Option<Vec<(f32, String)>>,
        preset: Option<Presets>,
    ) -> PyResult<Self> {
        ColorGradient::new_py(colors, preset)
            .map(|gradient| ConicalGradient::new(gradient, center, angle).into())
    }
}
