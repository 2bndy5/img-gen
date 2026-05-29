use fontsource_downloader::error::FontSourceError;

#[derive(Debug, thiserror::Error)]
pub enum ImgGenRendererError {
    #[error(transparent)]
    SpecError(#[from] img_gen_spec::ImgGenSpecError),

    // Rendering and geometry.
    #[error("{shape} bounds are too large; width and height must be less than i32::MAX / 4")]
    BoundsTooLarge { shape: &'static str },
    #[error("{shape} path has invalid bounds")]
    InvalidPathBounds { shape: &'static str },
    #[error("Failed to allocate pixmap for {shape} ({width}x{height})")]
    PixmapAllocationFailed {
        shape: &'static str,
        width: u32,
        height: u32,
    },
    #[error("Failed to convert raster buffer to RGBA for {shape} ({width}x{height})")]
    RasterBufferConversionFailed {
        shape: &'static str,
        width: u32,
        height: u32,
    },

    // Image IO.
    #[error("Failed to save image to '{path}'")]
    SaveImageFailed {
        path: String,
        #[source]
        source: image::ImageError,
    },
    #[error("Failed to collect image bytes from pixel buffer")]
    CollectImageBytesFailed {
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to open image '{path}'")]
    OpenImageFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to decode image '{path}'")]
    DecodeImageFailed {
        path: String,
        #[source]
        source: image::ImageError,
    },
    #[error("Failed to read SVG file '{path}'")]
    ReadSvgFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to parse SVG file '{path}'")]
    ParseSvgFailed {
        path: String,
        #[source]
        source: resvg::usvg::Error,
    },
    #[error("Failed to parse SVG XML in '{path}'")]
    ParseSvgXmlFailed {
        path: String,
        #[source]
        source: resvg::usvg::roxmltree::Error,
    },
    #[error("SVG '{path}' scaled to zero size")]
    SvgScaledToZeroSize { path: String },
    #[error("Image not found: '{name}'")]
    ImageNotFound { name: String },

    // Typography and font loading.
    #[error("Failed to read font file '{path}'")]
    ReadFontFileFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to create font reference for glyph run")]
    InvalidGlyphRunFontReference,
    #[error(transparent)]
    FontSourceError(#[from] FontSourceError),
}

pub type Result<T> = std::result::Result<T, ImgGenRendererError>;
