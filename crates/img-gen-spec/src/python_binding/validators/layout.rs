use pyo3::{exceptions::PyValueError, prelude::*};
use serde_saphyr::options::DuplicateKeyPolicy;

use crate::{
    Background, Debug, Ellipse, Icon, Layer, LayerOffset, Layout, Mask, Polygon, Rectangle, Size,
    SolidColor, Typography,
};

#[pymethods]
impl Mask {
    /// Creates a mask layer from optional geometry and drawable attributes.
    #[new]
    #[pyo3(
        text_signature = "(size: Size | None = None, offset: Offset | None = None, invert: bool = False, background: Background | None = None, ellipse: Ellipse | None = None, rectangle: Rectangle | None = None, polygon: Polygon | None = None, icon: Icon | None = None, typography: Typography | None = None) -> Mask",
        signature = (size = None, offset = None, invert = false, background = None, ellipse = None, rectangle = None, polygon = None, icon = None, typography = None)
    )]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        size: Option<Size>,
        offset: Option<LayerOffset>,
        invert: Option<bool>,
        background: Option<Background>,
        ellipse: Option<Ellipse>,
        rectangle: Option<Rectangle>,
        polygon: Option<Polygon>,
        icon: Option<Icon>,
        typography: Option<Typography>,
    ) -> Self {
        Self {
            size,
            offset: offset.unwrap_or_default(),
            invert: invert.unwrap_or_default(),
            background,
            rectangle,
            ellipse,
            polygon,
            icon,
            typography,
        }
    }

    /// Deserialize a `Mask` object from a YAML string.
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

    /// Deserialize a `Mask` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Mask` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Mask` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl Layer {
    #[cfg(feature = "pyo3")]
    /// Creates a layer from optional geometry and drawable attributes.
    #[new]
    #[pyo3(
        text_signature = "(size: Size | None = None, offset: Offset | None = None, background: Background | None = None, ellipse: Ellipse | None = None, rectangle: Rectangle | None = None, polygon: Polygon | None = None, icon: Icon | None = None, typography: Typography | None = None, mask: Mask | None = None) -> Layer",
        signature = (size = None, offset = None, background = None, ellipse = None, rectangle = None, polygon = None, icon = None, typography = None, mask = None)
    )]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        size: Option<Size>,
        offset: Option<LayerOffset>,
        background: Option<Background>,
        ellipse: Option<Ellipse>,
        rectangle: Option<Rectangle>,
        polygon: Option<Polygon>,
        icon: Option<Icon>,
        typography: Option<Typography>,
        mask: Option<Mask>,
    ) -> Self {
        Self {
            size,
            offset: offset.unwrap_or_default(),
            background,
            rectangle,
            ellipse,
            polygon,
            icon,
            typography,
            mask,
        }
    }

    /// Deserialize a `Layer` object from a YAML string.
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

    /// Deserialize a `Layer` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Layer` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Layer` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl Debug {
    /// Creates debug rendering options from ``enable``, ``grid``,
    /// optional``grid_step``, and optional ``color``.
    #[new]
    #[pyo3(
        text_signature = "(enable: bool = False, grid: bool = True, grid_step: int = 16, color: Color | None = None) -> Debug",
        signature = (enable = false, grid = true, grid_step = 16, color = None)
    )]
    pub fn new(
        enable: bool,
        grid: bool,
        grid_step: Option<u32>,
        color: Option<SolidColor>,
    ) -> Self {
        Self {
            enable,
            grid,
            grid_step: grid_step.unwrap_or_else(Self::default_grid_step),
            color: color.unwrap_or_else(Self::default_color),
        }
    }

    /// Calculate a black or white foreground color using `Debug.color` as a background.
    #[pyo3(name = "get_foreground_color", text_signature = "() -> SolidColor")]
    pub fn get_foreground_color_py(&self) -> SolidColor {
        self.get_foreground_color()
    }

    /// Deserialize a `Debug` object from a YAML string.
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

    /// Deserialize a `Debug` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Debug` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Debug` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl Layout {
    /// Creates a layout from optional ``size``, a list of ``layers``,
    /// and optional ``debug`` settings.
    #[new]
    #[pyo3(
        text_signature = "(size: Size | None = None, layers: list[Layer] = [], debug: Optional[Debug]) -> Layout",
        signature = (size = None, layers = Vec::new(), debug = None)
    )]
    pub fn new(size: Option<Size>, layers: Vec<Layer>, debug: Option<Debug>) -> Self {
        Self {
            size: size.unwrap_or_default(),
            layers,
            debug,
        }
    }

    /// Append a single `Layer` to the layout's list of layers.
    pub fn append_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    /// Extend the layout's list of layers with an iterable of `Layer` objects.
    pub fn extend_layers(&mut self, layers: Vec<Layer>) {
        self.layers.extend(layers);
    }

    /// Deserialize a `Layout` object from a YAML string.
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

    /// Deserialize a `Layout` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(yaml_str: String) -> PyResult<Self> {
        serde_json::from_str(&yaml_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Layout` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Layout` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
