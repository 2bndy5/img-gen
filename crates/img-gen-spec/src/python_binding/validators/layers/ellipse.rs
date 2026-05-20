#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use crate::{Border, ColorKind, Ellipse};

#[pymethods]
impl Ellipse {
    /// Instantiate an `Ellipse` object.
    #[new]
    #[pyo3(
        text_signature = "(color: ColorKind, border: Border | None = None) -> Ellipse",
        signature = (color, border = None)
    )]
    pub fn new(color: ColorKind, border: Option<Border>) -> Self {
        Self { color, border }
    }
}
