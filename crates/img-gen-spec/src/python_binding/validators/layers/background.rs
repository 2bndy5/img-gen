use crate::{Background, ColorKind, PreserveAspect};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pymethods]
impl Background {
    /// Instantiate a `Background` object.
    #[new]
    #[pyo3(
        text_signature = "(image: str = None, color: ColorKind = None, preserve_aspect: PreserveAspect = None) -> Background",
        signature = (image = None, color = None, preserve_aspect = None)
    )]
    pub fn new(
        image: Option<String>,
        color: Option<ColorKind>,
        preserve_aspect: Option<PreserveAspect>,
    ) -> Self {
        Self {
            image,
            color,
            preserve_aspect: preserve_aspect.unwrap_or_default(),
        }
    }

    /// Deserialize a `Background` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str).map_err(PyValueError::new_err)
    }

    /// Deserialize a `Background` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Background` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Background` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
