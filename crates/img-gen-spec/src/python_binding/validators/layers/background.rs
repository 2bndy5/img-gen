use crate::{Background, ColorKind, PreserveAspect};
use pyo3::prelude::*;

#[pymethods]
impl Background {
    /// Instantiate a `Background` object.
    #[new]
    #[pyo3(
        text_signature = "(image: str = None, color: ColorKind = None, preserve_aspect: PreserveAspect = None) -> Background",
        signature = (image = None, color = None, preserve_aspect = None)
    )]
    pub fn new(
        image: Option<String>,
        color: Option<ColorKind>,
        preserve_aspect: Option<PreserveAspect>,
    ) -> Self {
        Self {
            image,
            color,
            preserve_aspect: preserve_aspect.unwrap_or_default(),
        }
    }
}
