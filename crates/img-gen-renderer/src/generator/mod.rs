use fontsource_downloader::FontSourceClient;
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use image::RgbaImage;
use resvg::usvg::Options;
use sha2::{Digest, Sha256};
use std::{borrow::Cow, io::Read, path::PathBuf};

use crate::{
    ImgGenRendererError, Layout, Result,
    validators::{HEIGHT, WIDTH},
};
mod renderer;
use renderer::Renderer;

/// A class to represent an image generator.
///
/// The given `Layout` describes how to generate the `Image`.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Clone)]
pub struct Generator {
    pub layout: Layout,
}

impl Generator {
    /// Render the `Image` described by the `Generator`'s `Layout`.
    pub async fn render(&self) -> Result<Image> {
        self.render_with_cache_root(None).await
    }

    /// Render while overriding the font cache root directory for this call.
    pub async fn render_with_cache_root(&self, cache_root: Option<PathBuf>) -> Result<Image> {
        let mut canvas = RgbaImage::new(
            self.layout.size.width.unwrap_or(WIDTH).get(),
            self.layout.size.height.unwrap_or(HEIGHT).get(),
        );
        let mut opt = Options {
            // Get file's absolute directory.
            // resources_dir: std::fs::canonicalize(&path)
            //     .ok()
            //     .and_then(|p| p.parent().map(|p| p.to_path_buf())),
            ..Default::default()
        };

        opt.fontdb_mut().load_system_fonts();
        opt.fontdb_mut().load_fonts_dir(".");

        let fontsource_downloader = if let Some(root) = cache_root {
            FontSourceClient::with_cache_root(root)
        } else {
            FontSourceClient::new()
        }?;

        let mut renderer = Renderer::new(opt, fontsource_downloader);
        for layer in &self.layout.layers {
            renderer.render_layer(layer, &mut canvas).await?;
        }
        renderer.render_debug(&self.layout, &mut canvas).await?;
        Ok(Image { data: canvas })
    }
}

/// A class to represent an Image object.
///
/// This class cannot be constructed from python, rather it is returned from
/// `Generator.render()`
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Clone)]
pub struct Image {
    pub data: RgbaImage,
}

impl Image {
    pub fn save(&self, name: &str) -> Result<()> {
        self.data
            .save(name)
            .map_err(|source| ImgGenRendererError::SaveImageFailed {
                path: name.to_string(),
                source,
            })
    }

    pub fn get_bytes(&'_ self) -> Result<Cow<'_, [u8]>> {
        let bytes: Vec<u8> = self
            .data
            .bytes()
            .collect::<std::io::Result<Vec<u8>>>()
            .map_err(|source| ImgGenRendererError::CollectImageBytesFailed { source })?;
        Ok(Cow::Owned(bytes))
    }

    pub fn get_sha256(&self) -> Result<String> {
        let mut hash_gen = Sha256::new();
        hash_gen.update(self.get_bytes()?);
        let digest = hash_gen.finalize();
        Ok(digest
            .as_slice()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::{Image, RgbaImage};

    #[test]
    fn hash() {
        let buffer = RgbaImage::new(50, 50);
        let img = Image { data: buffer };
        let sha256 = img.get_sha256().unwrap();
        assert_eq!(
            sha256,
            "95b532cc4381affdff0d956e12520a04129ed49d37e154228368fe5621f0b9a2"
        );
    }
}
