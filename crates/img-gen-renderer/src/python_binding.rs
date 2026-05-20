use crate::{
    Layout,
    generator::{Generator, Image},
};
use pyo3::{exceptions::PyOSError, prelude::*};
use std::{borrow::Cow, path::PathBuf};

#[pymethods]
impl Generator {
    /// Instantiate a `Generator` object for a given `layout`.
    #[new]
    #[pyo3(text_signature = "(layout: Layout) -> Generator")]
    pub fn new(layout: Layout) -> Self {
        Generator { layout }
    }

    /// Render the layout and return the `Image`.
    #[pyo3(
        name = "render",
        text_signature = "(cache_root: Path | None = None) -> Image",
        signature = (cache_root=None)
    )]
    pub fn render_py<'py>(
        &self,
        py: Python<'py>,
        cache_root: Option<PathBuf>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            this.render_with_cache_root(cache_root)
                .await
                .map_err(|e| PyOSError::new_err(format!("{e:?}")))
        })
    }
}

#[pymethods]
impl Image {
    /// The image data as a `bytes` object
    #[getter(bytes)]
    pub fn get_bytes_py(&'_ self) -> PyResult<Cow<'_, [u8]>> {
        self.get_bytes()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// Save the image data to a file.
    ///
    /// The image format is determined from the file extension in the given ``name``.
    #[pyo3(text_signature = "(name: str) -> None", name = "save")]
    pub fn save_py(&self, name: &str) -> PyResult<()> {
        self.save(name)
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }

    /// A hexadecimal string representing the SHA256 hash digest of the image data.
    #[getter(sha256)]
    pub fn get_sha256_py(&self) -> PyResult<String> {
        self.get_sha256()
            .map_err(|e| PyOSError::new_err(e.to_string()))
    }
}
