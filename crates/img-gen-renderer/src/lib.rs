//! Rendering primitives and runtime errors for image generation.
//!
//! This crate turns validated image-generation specifications into images.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    missing_docs
)]

pub use img_gen_spec::*;

/// Error types produced while rendering images or loading renderer resources.
pub mod error;
pub use error::{ImgGenRendererError, Result};

mod generator;
pub use generator::{Generator, Image};

#[cfg(feature = "pyo3")]
mod python_binding;
