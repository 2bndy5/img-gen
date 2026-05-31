use crate::{ColorGradient, Presets};
use pyo3::{exceptions::PyValueError, prelude::*};
use serde_saphyr::options::DuplicateKeyPolicy;
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
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize a `ColorGradient` object from a YAML string.
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

    /// Deserialize a `ColorGradient` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `ColorGradient` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `ColorGradient` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
