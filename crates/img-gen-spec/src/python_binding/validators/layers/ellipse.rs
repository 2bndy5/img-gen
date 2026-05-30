#[cfg(feature = "pyo3")]
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
}
