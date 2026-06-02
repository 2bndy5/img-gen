use std::num::NonZeroU32;

use crate::{Border, ColorKind, PreserveAspect, validators::layers::BORDER_WIDTH};
use pyo3::{exceptions::PyValueError, prelude::*};

mod background;
mod colors;
mod ellipse;
mod icon;
mod polygon;
mod rectangle;
mod size_offset;
mod typography;

#[pymethods]
impl Border {
    /// Instantiate a `Border` object.
    #[new]
    #[pyo3(
        text_signature = "(color: ColorKind, width: int = 1) -> None",
        signature = (color, width = 1)
    )]
    pub fn new(color: ColorKind, width: Option<u32>) -> PyResult<Self> {
        let width = match width {
            Some(w) => NonZeroU32::new(w).ok_or(PyValueError::new_err(
                "Border.width must be greater than 0".to_string(),
            ))?,
            None => BORDER_WIDTH,
        };
        Ok(Self { color, width })
    }

    /// Returns the border width.
    #[getter]
    pub fn width(&self) -> u32 {
        self.width.get()
    }

    /// Sets the border width from ``width``.
    #[setter]
    pub fn set_width(&mut self, width: u32) -> PyResult<()> {
        self.width = NonZeroU32::new(width).ok_or(PyValueError::new_err(format!(
            "Border.width must be greater than 0; got {width}"
        )))?;
        Ok(())
    }

    /// Deserialize a `Border` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize a `Border` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Border` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `Border` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}

#[pymethods]
impl PreserveAspect {
    /// Translate a given string value into a `PreserveAspect` enumeration.
    ///
    /// Any unsupported value (see signature) defaults to `PreserveAspect.On`.
    #[staticmethod]
    #[pyo3(
        name = "from_string",
        text_signature = "(Literal['false', 'width', 'height', 'true']) -> PreserveAspect"
    )]
    pub fn from_string_py(value: String) -> PreserveAspect {
        Self::from_string(&value)
    }

    /// Deserialize a `PreserveAspect` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize a `PreserveAspect` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `PreserveAspect` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `PreserveAspect` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}
