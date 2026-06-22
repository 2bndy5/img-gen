use crate::{ColorKind, Icon, PreserveAspect};
use pyo3::prelude::*;

#[pymethods]
impl Icon {
    /// Instantiate an `Icon` object.
    ///
    /// ``preserve_aspect`` defaults to `PreserveAspect.On` if not specified.
    #[new]
    #[pyo3(
        text_signature = "(image: str, color: ColorKind = None, preserve_aspect: PreserveAspect | None = None) -> Icon",
        signature = (image, color = None, preserve_aspect = None)
    )]
    pub fn new(
        image: String,
        color: Option<ColorKind>,
        preserve_aspect: Option<PreserveAspect>,
    ) -> Self {
        Self {
            image,
            color,
            preserve_aspect: preserve_aspect.unwrap_or_default(),
        }
    }

    /// Deserialize an `Icon` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize an `Icon` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Icon` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Icon` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}
