#[cfg(feature = "pyo3")]
use pyo3::{exceptions::PyValueError, prelude::*};
use serde_saphyr::options::DuplicateKeyPolicy;

use crate::{ColorGradient, ConicalGradient, LayerOffset, SolidColor};

#[pymethods]
impl ConicalGradient {
    /// Creates a conical gradient from ``colors``, ``center``, and optional ``angle``.
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

    /// Sets the starting gradient angle from ``val``.
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

    /// Deserialize a `ConicalGradient` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        serde_saphyr::from_str_with_options(
            &yaml_str,
            serde_saphyr::options! {
                duplicate_keys: DuplicateKeyPolicy::LastWins,
            },
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize a `ConicalGradient` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `ConicalGradient` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `ConicalGradient` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
