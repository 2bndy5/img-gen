use directories::ProjectDirs;
use fontsource_downloader::FontSourceClient;
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use image::RgbaImage;
use resvg::usvg::{Options, fontdb};
use sha2::{Digest, Sha256};
use std::{
    borrow::Cow,
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    ImgGenRendererError, Layout, Result,
    validators::{HEIGHT, WIDTH},
};
mod renderer;
use renderer::Renderer;

/// A class to represent an image generator.
///
/// The given `Layout` describes how to generate the `Image`.
///
/// This struct caches the font database and font source client to avoid
/// re-initializing them on every `render()` call.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Clone)]
pub struct Generator {
    /// Search paths used to resolve input images and SVG assets.
    pub image_search_paths: Vec<PathBuf>,
    fontdb: Arc<fontdb::Database>,
    fontsource_client: FontSourceClient,
    /// The root directory used for renderer cache data such as downloaded fonts.
    pub cache_root: PathBuf,
}

impl Generator {
    /// Create a new `Generator` with the given image search paths.
    ///
    /// This initializes the shared font database and font source client once.
    /// Both are cached in-memory and reused across all subsequent renders.
    pub fn new(image_search_paths: Vec<PathBuf>, cache_root: Option<PathBuf>) -> Result<Self> {
        let fontdb = fontdb::Database::new();

        let cache_root = cache_root
            .or_else(|| {
                // Keep font downloads under img-gen's app cache by default.
                ProjectDirs::from("", "2bndy5", "img-gen")
                    .map(|dirs| dirs.cache_dir().to_path_buf())
            })
            .unwrap_or_else(|| PathBuf::from(".img-gen-cache"));

        let fontsource_client = FontSourceClient::with_cache_root(&cache_root)?;

        Ok(Generator {
            image_search_paths,
            fontdb: Arc::new(fontdb),
            fontsource_client,
            cache_root,
        })
    }

    /// Render the `Image` described by the `Generator`'s `Layout`.
    pub async fn render(&self, layout: Layout) -> Result<Image> {
        let mut canvas = RgbaImage::new(
            layout.size.width.unwrap_or(WIDTH).get(),
            layout.size.height.unwrap_or(HEIGHT).get(),
        );

        // Create Options from the cached fontdb. This is cheap since we're just
        // cloning Arcs and reusing the pre-loaded font database.
        let opt = Options {
            fontdb: self.fontdb.clone(),
            ..Default::default()
        };

        let mut renderer = Renderer::new(opt, &self.fontsource_client, &self.image_search_paths);
        for layer in &layout.layers {
            renderer.render_layer(layer, &mut canvas).await?;
        }
        renderer.render_debug(&layout, &mut canvas).await?;
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
    /// The raw RGBA pixel buffer for the rendered image.
    pub data: RgbaImage,
}

impl Image {
    /// Save the image to a file.
    ///
    /// Does not support SVG output.
    /// The file format is inferred from the file extension in the given `name`.
    pub fn save<P: AsRef<Path>>(&self, name: P) -> Result<()> {
        let name = name.as_ref();
        self.data
            .save(name)
            .map_err(|source| ImgGenRendererError::SaveImageFailed {
                path: name.to_string_lossy().into_owned(),
                source,
            })
    }

    /// Get a byte array of the image data.
    pub fn get_bytes(&'_ self) -> Result<Cow<'_, [u8]>> {
        let bytes: Vec<u8> = self
            .data
            .bytes()
            .collect::<std::io::Result<Vec<u8>>>()
            .map_err(|source| ImgGenRendererError::CollectImageBytesFailed { source })?;
        Ok(Cow::Owned(bytes))
    }

    /// Get the SHA256 hash of the image data as a hex string.
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
