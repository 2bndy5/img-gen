mod validators;

use pyo3::{exceptions::PyValueError, prelude::*};
use serde::de::DeserializeOwned;

/// Parse YAML with duplicate-key policy `LastWins` into type `T`.
/// Returns a Python `ValueError` on failure.
pub(crate) fn parse_yaml_last_wins<T>(s: &str) -> PyResult<T>
where
    T: DeserializeOwned,
{
    let opts = serde_saphyr::options! {
        duplicate_keys: serde_saphyr::options::DuplicateKeyPolicy::LastWins,
    };
    serde_saphyr::from_str_with_options(s, opts).map_err(crate::python_binding::map_to_value_err)
}

pub(crate) fn map_to_value_err<E: std::fmt::Display>(e: E) -> PyErr {
    PyValueError::new_err(e.to_string())
}
