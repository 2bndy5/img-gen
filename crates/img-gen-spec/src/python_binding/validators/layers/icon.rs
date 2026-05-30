use crate::{ColorKind, Icon, PreserveAspect};
use pyo3::prelude::*;

#[pymethods]
impl Icon {
    /// Instantiate an `Icon` object.
    #[new]
    #[pyo3(
        text_signature = "(image: str, color: ColorKind = None, preserve_aspect: PreserveAspect | None = None) -> Icon",
        signature = (image, color = None, preserve_aspect = None)
    )]
    pub fn new(
        image: String,
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
