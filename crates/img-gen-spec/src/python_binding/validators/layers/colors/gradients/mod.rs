use crate::{ColorGradient, Presets};
use pyo3::prelude::*;
mod types;

#[pymethods]
impl ColorGradient {
    /// Creates a color gradient from ``spec`` or a named ``preset``.
    ///
    /// When ``spec`` is omitted, the preset is used instead.
    #[new]
    #[pyo3(
        text_signature = "(spec: Optional[List[Tuple[float, str]]] = None, preset: Optional[Preset] = Presets.MonoChrome) -> ColorGradient",
        signature = (spec = None, preset = None)
    )]
    pub fn new_py(spec: Option<Vec<(f32, String)>>, preset: Option<Presets>) -> PyResult<Self> {
        let mut color_spec = vec![];
        if let Some(ref v) = spec {
            for (p, s) in v {
                color_spec.push((*p, s.as_str()));
            }
        }
        Self::new(spec.as_ref().map(|_| color_spec), preset)
            .map_err(crate::python_binding::map_to_value_err)
    }

    /// Deserialize a `ColorGradient` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize a `ColorGradient` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `ColorGradient` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `ColorGradient` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}
