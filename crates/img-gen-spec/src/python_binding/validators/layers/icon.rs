use crate::{ColorKind, Icon, PreserveAspect};
use pyo3::{exceptions::PyValueError, prelude::*};
use serde_saphyr::options::DuplicateKeyPolicy;

#[pymethods]
impl Icon {
    /// Instantiate an `Icon` object.
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
        serde_saphyr::from_str_with_options(
            &yaml_str,
            serde_saphyr::options! {
                duplicate_keys: DuplicateKeyPolicy::LastWins,
            },
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize an `Icon` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Icon` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Icon` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
