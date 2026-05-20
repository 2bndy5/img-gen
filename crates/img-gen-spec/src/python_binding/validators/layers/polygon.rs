use crate::{Border, ColorKind, Polygon, PolygonSides};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pymethods]
impl Polygon {
    #[new]
    #[pyo3(
        text_signature = "(color: ColorKind, border: Border | None = None, sides: int = 3, rotation: float = 0.0) -> Polygon",
        signature = (color, border = None, sides = 3u32, rotation = 0.0f32)
    )]
    pub fn new(
        color: ColorKind,
        border: Option<Border>,
        sides: Option<u32>,
        rotation: Option<f32>,
    ) -> PyResult<Self> {
        Ok(Self {
            color,
            border,
            sides: PolygonSides::new(sides.unwrap_or(3)).ok_or(PyValueError::new_err(
                "PolygonSides cannot be less than 3".to_string(),
            ))?,
            rotation: rotation.unwrap_or_default(),
        })
    }

    /// The number of sides that constitutes the polygon
    ///
    /// Note, this cannot be less than 3.
    #[getter(sides)]
    pub fn get_sides_py(&self) -> u32 {
        self.sides.get()
    }

    #[setter(sides)]
    pub fn set_sides_py(&mut self, val: u32) -> PyResult<()> {
        self.sides = PolygonSides::new(val).ok_or(PyValueError::new_err(
            "PolygonSides cannot be less than 3".to_string(),
        ))?;
        Ok(())
    }
}
