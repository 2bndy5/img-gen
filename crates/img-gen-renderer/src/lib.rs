#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo
)]

pub use img_gen_spec::*;

pub mod error;
pub use error::{ImgGenRendererError, Result};

mod generator;
pub use generator::{Generator, Image};

#[cfg(feature = "pyo3")]
mod python_binding;
