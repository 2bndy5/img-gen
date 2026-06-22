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

    /// Deserialize a `ColorKind` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize a `ColorKind` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `ColorKind` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `ColorKind` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}

#[pymethods]
impl Presets {
    /// Factory method to create a `Presets` object from a preset name string.
    ///
    /// The preset name must exactly match the enum variant name (case-sensitive).
    #[staticmethod]
    #[pyo3(name = "from_str", text_signature = "(preset_name: str) -> Presets")]
    #[allow(
        clippy::should_implement_trait,
        reason = "Its a python bound method, generics aren't allowed for pyo3 pyclass"
    )]
    pub fn from_str(preset_name: &str) -> PyResult<Self> {
        Self::try_from_str(preset_name)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid preset name: {preset_name}")))
    }

    /// Factory method to create a `Presets` object from its declaration-order index.
    ///
    /// The preset index must be a valid integer corresponding to a preset (0-based).
    #[staticmethod]
    #[pyo3(name = "from_index", text_signature = "(preset_index: int) -> Presets")]
    pub fn from_index(preset_index: i32) -> PyResult<Self> {
        Self::try_from_index(preset_index.clamp(0, u8::MAX as i32) as u8)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid preset index: {preset_index}")))
    }
}
