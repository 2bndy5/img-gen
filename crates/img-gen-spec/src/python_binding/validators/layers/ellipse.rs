use pyo3::prelude::*;

use crate::{Arc, Border, ColorKind, Ellipse};

#[pymethods]
impl Arc {
    /// Instantiate an `Arc` object.
    #[new]
    #[pyo3(text_signature = "(start: float, end: float) -> Arc", signature = (start, end))]
    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }

    /// Deserialize an `Arc` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize an `Arc` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Arc` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Arc` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}

#[pymethods]
impl Ellipse {
    /// Instantiate an `Ellipse` object.
    #[new]
    #[pyo3(
        text_signature = "(color: ColorKind, border: Border | None = None, arc: Arc | None = None, border_to_origin: bool = False) -> Ellipse",
        signature = (color, border = None, arc = None, border_to_origin = false)
    )]
    pub fn new(
        color: ColorKind,
        border: Option<Border>,
        arc: Option<Arc>,
        border_to_origin: bool,
    ) -> Self {
        Self {
            color,
            border,
            arc,
            border_to_origin,
        }
    }

    /// Deserialize an `Ellipse` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize an `Ellipse` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Ellipse` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Ellipse` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}
