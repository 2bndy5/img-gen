use crate::{Border, ColorKind, Corners, Rectangle};
use pyo3::prelude::*;

#[pymethods]
impl Corners {
    /// Returns a list of all corner enum values.
    #[staticmethod]
    pub fn all() -> Vec<Self> {
        Self::ALL.to_vec()
    }

    /// Deserialize a `Corners` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize a `Corners` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Corners` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Corners` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}

#[pymethods]
impl Rectangle {
    /// Instantiate a `Rectangle` object.
    ///
    /// The `corners` parameter defaults to all `Corners` if `None` is specified.
    #[new]
    #[pyo3(
        text_signature = "(color: ColorKind, radius: float = 0, corners: list[Corners] | None = None, border: Border | None = None) -> Rectangle",
        signature = (color, radius = 0f32, corners = None, border = None)
    )]
    pub fn new(
        color: ColorKind,
        radius: f32,
        corners: Option<Vec<Corners>>,
        border: Option<Border>,
    ) -> Self {
        Self {
            color,
            radius,
            corners: match corners {
                None => Corners::ALL.to_vec(),
                Some(v) => {
                    if v.is_empty() {
                        vec![]
                    } else {
                        v
                    }
                }
            },
            border,
        }
    }

    /// Deserialize a `Rectangle` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize a `Rectangle` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Rectangle` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Rectangle` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}
