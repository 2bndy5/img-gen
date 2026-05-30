use crate::{Border, ColorKind, Corners, Rectangle};
use pyo3::prelude::*;

#[pymethods]
impl Corners {
    /// Returns a list of all corner enum values.
    #[staticmethod]
    pub fn all() -> Vec<Self> {
        Self::ALL.to_vec()
    }
}

#[pymethods]
impl Rectangle {
    /// Instantiate a `Rectangle` object.
    ///
    /// The `corners` parameter defaults to all `Corners` if `None` is specified.
    #[new]
    #[pyo3(
        text_signature = "(color: ColorKind, radius: float = 0, corners: list[Corners] | None = None, border: Border | None = None) -> Rectangle",
        signature = (color, radius = 0f32, corners = None, border = None)
    )]
    pub fn new(
        color: ColorKind,
        radius: f32,
        corners: Option<Vec<Corners>>,
        border: Option<Border>,
    ) -> Self {
        Self {
            color,
            radius,
            corners: match corners {
                None => Corners::ALL.to_vec(),
                Some(v) => {
                    if v.is_empty() {
                        vec![]
                    } else {
                        v
                    }
                }
            },
            border,
        }
    }
}
