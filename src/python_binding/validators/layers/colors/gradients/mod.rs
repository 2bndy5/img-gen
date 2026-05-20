use crate::{ColorGradient, Presets};
use pyo3::{exceptions::PyValueError, prelude::*};
mod types;

#[pymethods]
impl ColorGradient {
    #[new]
    #[pyo3(
        text_signature = "(spec: Optional[List[Tuple[float, str]]] = None, preset: Optional[Preset] = Presets.MonoChrome) -> ColorGradient",
        signature = (spec = None, preset = None)
    )]
    pub fn new_py(spec: Option<Vec<(f32, String)>>, preset: Option<Presets>) -> PyResult<Self> {
        let mut color_spec = vec![];
        if let Some(ref v) = spec {
            for (p, s) in v {
                color_spec.push((*p, s.as_str()));
            }
        }
        Self::new(spec.as_ref().map(|_| color_spec), preset)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
