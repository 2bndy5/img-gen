use std::num::NonZeroI32;

use crate::{
    Border, ColorKind, Font, Line, Typography, TypographyAlign, Weight, validators::LineHeight,
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pymethods]
impl Line {
    /// Creates a typography's line descriptor from ``amount`` and ``height``.
    ///
    /// ``amount`` must be greater than zero, and ``height`` must not be zero.
    #[new]
    #[pyo3(
        text_signature = "(amount: int = 1, height: float = 1.0) -> Line",
        signature = (amount = 1, height = 1.0f32)
    )]
    pub fn new(amount: i32, height: f32) -> PyResult<Self> {
        let amount = NonZeroI32::new(amount).ok_or(PyValueError::new_err(
            "Line.amount must be greater than zero.",
        ))?;
        let height = LineHeight::new(height).ok_or(PyValueError::new_err(
            "Line.height must be greater than zero.",
        ))?;
        Ok(Self { amount, height })
    }

    /// Returns the current line height.
    #[getter]
    pub fn get_height(&self) -> f32 {
        self.height.get()
    }

    /// Returns the current line amount.
    #[getter]
    pub fn get_amount(&self) -> i32 {
        self.amount.get()
    }

    /// Sets the line height from ``height``.
    #[setter]
    pub fn set_height(&mut self, height: f32) -> PyResult<()> {
        self.height = LineHeight::new(height).ok_or(PyValueError::new_err(
            "Line.height must be greater than zero.",
        ))?;
        Ok(())
    }

    /// Sets the line amount from ``amount``.
    #[setter]
    pub fn set_amount(&mut self, amount: i32) -> PyResult<()> {
        self.amount = NonZeroI32::new(amount).ok_or(PyValueError::new_err(
            "Line.amount must be greater than zero.",
        ))?;
        Ok(())
    }

    /// Deserialize a `Line` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        serde_saphyr::from_str(&yaml_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize a `Line` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Line` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Line` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl Font {
    /// Creates a font from ``family`` and optional
    /// ``style``, ``weight``, ``subset``, and ``path``.
    #[new]
    #[pyo3(
        text_signature = "(family: str = \"Roboto\", style: str | None = None, weight: Weight | None = None, subset: str | None = None, path: str | None = None) -> Font",
        signature = (family = "Roboto".to_string(), style = None, weight = None, subset = None, path = None)
    )]
    pub fn new(
        family: String,
        style: Option<String>,
        weight: Option<Weight>,
        subset: Option<String>,
        path: Option<String>,
    ) -> Self {
        Font::from_parts(family, style, weight, subset, path)
    }

    /// Deserialize a `Font` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        serde_saphyr::from_str(&yaml_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize a `Font` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Font` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Font` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl Typography {
    /// Instantiate a `Typography` object.
    ///
    /// If color is not given, then solid black is used by default.
    /// If align is not given, then `TypographyAlign.StartTop` is used by default.
    /// If font is not given, then Roboto (400 weight) is used by default.
    /// See `Typography.overflow` for behavioral details.
    #[new]
    #[pyo3(
        text_signature = "(content: str, align: TypographyAlign | None = None, color: ColorKind | None = None, line: Line | None = None, overflow: bool = False, font: Font | None = None, border: Border | None = None) -> Typography",
        signature = (content, align = None, color = None, line = None, overflow = false, font = None, border = None)
    )]
    pub fn new(
        content: String,
        align: Option<TypographyAlign>,
        color: Option<ColorKind>,
        line: Option<Line>,
        overflow: bool,
        font: Option<Font>,
        border: Option<Border>,
    ) -> Self {
        Self {
            content,
            align: align.unwrap_or_default(),
            color: color.unwrap_or_default(),
            line: line.unwrap_or_default(),
            overflow,
            font: font.unwrap_or_default(),
            border,
        }
    }

    /// Deserialize a `Typography` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        serde_saphyr::from_str(&yaml_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize a `Typography` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Typography` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Typography` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
