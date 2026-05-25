use crate::{
    Border, ColorKind, IrregularPolygonSides, LayerOffset, Polygon, PolygonSides,
    RegularPolygonSides,
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pymethods]
impl PolygonSides {
    #[staticmethod]
    pub fn regular(sides: u32) -> PyResult<Self> {
        RegularPolygonSides::new(sides)
            .map(Into::into)
            .ok_or(PyValueError::new_err(
                "PolygonSides cannot be less than 3".to_string(),
            ))
    }

    #[staticmethod]
    pub fn irregular(offsets: Vec<crate::LayerOffset>) -> PyResult<Self> {
        IrregularPolygonSides::new(offsets)
            .map(Into::into)
            .ok_or(PyValueError::new_err(
                "PolygonSides irregular offsets cannot have less than 3 unique points".to_string(),
            ))
    }
}

#[pymethods]
impl RegularPolygonSides {
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
}

#[pymethods]
impl IrregularPolygonSides {
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
}

#[pymethods]
impl Polygon {
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
}
