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

    /// Deserialize a `LinearGradient` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize a `LinearGradient` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `LinearGradient` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `LinearGradient` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}
