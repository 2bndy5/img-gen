use std::num::NonZeroU32;

use crate::{LayerOffset, Size};
#[cfg(feature = "pyo3")]
use pyo3::{exceptions::PyValueError, prelude::*};
use serde_saphyr::options::DuplicateKeyPolicy;

#[pymethods]
impl LayerOffset {
    /// Instantiate a `LayerOffset` object.
    #[new]
    #[pyo3(
        text_signature = "(x: int = 0, y: int = 0) -> Offset",
        signature = (x = 0, y = 0)
    )]
    pub fn new(x: Option<i32>, y: Option<i32>) -> Self {
        Self {
            x: x.unwrap_or_default(),
            y: y.unwrap_or_default(),
        }
    }

    /// Deserialize a `LayerOffset` object from a YAML string.
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

    /// Deserialize a `LayerOffset` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `LayerOffset` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `LayerOffset` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl Size {
    /// Instantiate a [`Size`] object.
    #[new]
    #[pyo3(
        text_signature = "(width: int = None, height: int = None) -> Size",
        signature = (width = None, height = None)
    )]
    pub fn new(width: Option<u32>, height: Option<u32>) -> PyResult<Self> {
        let width = match width {
            Some(w) => Some(
                NonZeroU32::new(w)
                    .ok_or_else(|| PyValueError::new_err("Size.width cannot be zero"))?,
            ),
            None => None,
        };
        let height = match height {
            Some(h) => Some(
                NonZeroU32::new(h)
                    .ok_or_else(|| PyValueError::new_err("Size.height cannot be zero"))?,
            ),
            None => None,
        };

        Ok(Self { width, height })
    }

    /// The current width.
    #[setter(width)]
    pub fn set_width_py(&mut self, width: Option<u32>) -> PyResult<()> {
        let width = match width {
            Some(w) => Some(
                NonZeroU32::new(w)
                    .ok_or_else(|| PyValueError::new_err("Size.width cannot be zero"))?,
            ),
            None => None,
        };
        self.width = width;
        Ok(())
    }

    /// The current height.
    #[setter(height)]
    pub fn set_height_py(&mut self, height: Option<u32>) -> PyResult<()> {
        let height = match height {
            Some(h) => Some(
                NonZeroU32::new(h)
                    .ok_or_else(|| PyValueError::new_err("Size.height cannot be zero"))?,
            ),
            None => None,
        };
        self.height = height;
        Ok(())
    }

    /// Returns the current width, if set.
    #[getter(width)]
    pub fn get_width_py(&self) -> Option<u32> {
        self.width.map(|v| v.get())
    }

    /// Returns the current height, if set.
    #[getter(height)]
    pub fn get_height_py(&self) -> Option<u32> {
        self.height.map(|v| v.get())
    }

    /// Deserialize a `Size` object from a YAML string.
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

    /// Deserialize a `Size` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Size` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Size` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
