use crate::{
    Border, ColorKind, IrregularPolygonSides, LayerOffset, Polygon, PolygonSides,
    RegularPolygonSides,
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pymethods]
impl PolygonSides {
    /// Creates regular polygon sides from ``sides``.
    ///
    /// The number of sides cannot be less than 3.
    #[staticmethod]
    pub fn regular(sides: u32) -> PyResult<Self> {
        RegularPolygonSides::new(sides)
            .map(Into::into)
            .ok_or(PyValueError::new_err(
                "PolygonSides cannot be less than 3".to_string(),
            ))
    }

    /// Creates irregular polygon sides from vertex ``offsets``.
    ///
    /// Each offset will be clamped to the bounds of the `Layer` dimensions,
    /// and duplicate offsets will be removed. The last offset will be
    /// connected to the first to close the polygon.
    ///
    /// The offsets list must contain at least 3 unique points.
    #[staticmethod]
    pub fn irregular(offsets: Vec<crate::LayerOffset>) -> PyResult<Self> {
        IrregularPolygonSides::new(offsets)
            .map(Into::into)
            .ok_or(PyValueError::new_err(
                "PolygonSides irregular offsets cannot have less than 3 unique points".to_string(),
            ))
    }

    /// Deserialize a `PolygonSides` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        serde_saphyr::from_str(&yaml_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize a `PolygonSides` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `PolygonSides` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `PolygonSides` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl RegularPolygonSides {
    /// Creates a regular polygon side-count wrapper from ``sides``.
    ///
    /// The number of sides cannot be less than 3.
    #[new]
    #[pyo3(
        text_signature = "(sides: int = 3) -> RegularPolygonSides",
        signature = (sides=3)
    )]
    pub fn new_py(sides: Option<u32>) -> PyResult<Self> {
        RegularPolygonSides::new(sides.unwrap_or(3)).ok_or(PyValueError::new_err(
            "PolygonSides cannot be less than 3".to_string(),
        ))
    }

    /// Get the number of sides of a regular polygon.
    #[pyo3(name = "get", text_signature = "() -> u32")]
    pub fn get_py(&self) -> u32 {
        self.get()
    }

    /// Deserialize a `RegularPolygonSides` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        serde_saphyr::from_str(&yaml_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize a `RegularPolygonSides` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `RegularPolygonSides` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `RegularPolygonSides` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl IrregularPolygonSides {
    /// Creates irregular polygon sides from the vertex ``offsets`` list.
    ///
    /// Each offset will be clamped to the bounds of the `Layer` dimensions,
    /// and duplicate offsets will be removed. The last offset will be
    /// connected to the first to close the polygon.
    ///
    /// The offsets list must contain at least 3 unique points.
    #[new]
    #[pyo3(text_signature = "(offsets: list[Offset]) -> IrregularPolygonSides")]
    pub fn new_py(offsets: Vec<LayerOffset>) -> PyResult<Self> {
        IrregularPolygonSides::new(offsets).ok_or_else(|| {
            PyValueError::new_err(
                "PolygonSides irregular offsets cannot have less than 3 unique points".to_string(),
            )
        })
    }

    /// Get the offsets that mark the vertices of an irregular polygon.
    #[pyo3(text_signature = "() -> list[Offset]")]
    pub fn get(&self) -> Vec<LayerOffset> {
        self.as_slice().to_owned()
    }

    /// Deserialize an `IrregularPolygonSides` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        serde_saphyr::from_str(&yaml_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize an `IrregularPolygonSides` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `IrregularPolygonSides` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `IrregularPolygonSides` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl Polygon {
    /// Creates a polygon from ``color`` and optional
    /// ``border``, ``sides``, and ``rotation``.
    #[new]
    #[pyo3(
        text_signature = "(color: ColorKind, border: Border | None = None, sides: PolygonSides = PolygonSides.regular(3), rotation: float = 0.0) -> Polygon",
        signature = (color, border = None, sides = None, rotation = 0.0f32)
    )]
    pub fn new(
        color: ColorKind,
        border: Option<Border>,
        sides: Option<PolygonSides>,
        rotation: Option<f32>,
    ) -> PyResult<Self> {
        Ok(Self {
            color,
            border,
            sides: sides.unwrap_or_default(),
            rotation: rotation.unwrap_or_default(),
        })
    }

    /// Deserialize a `Polygon` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        serde_saphyr::from_str(&yaml_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Deserialize a `Polygon` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Polygon` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize the `Polygon` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
