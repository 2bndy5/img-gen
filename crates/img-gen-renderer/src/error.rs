use fontsource_downloader::error::FontSourceError;

/// Errors produced while rendering images and loading renderer resources.
#[derive(Debug, thiserror::Error)]
pub enum ImgGenRendererError {
    /// Wraps a specification validation error.
    #[error(transparent)]
    SpecError(#[from] img_gen_spec::ImgGenSpecError),

    // --- Rendering and geometry. ---
    /// Returned when a rendered shape exceeds safe raster bounds.
    #[error("{shape} bounds are too large; width and height must be less than i32::MAX / 4")]
    BoundsTooLarge {
        /// The shape whose bounds were invalid.
        shape: &'static str,
    },

    /// Returned when a shape path reports invalid bounds.
    #[error("{shape} path has invalid bounds")]
    InvalidPathBounds {
        /// The shape whose path bounds were invalid.
        shape: &'static str,
    },

    /// Returned when pixmap allocation fails for a rendered shape.
    #[error("Failed to allocate pixmap for {shape} ({width}x{height})")]
    PixmapAllocationFailed {
        /// The shape being rasterized.
        shape: &'static str,
        /// The requested pixmap width.
        width: u32,
        /// The requested pixmap height.
        height: u32,
    },

    /// Returned when an intermediate raster buffer cannot be converted to RGBA pixels.
    #[error("Failed to convert raster buffer to RGBA for {shape} ({width}x{height})")]
    RasterBufferConversionFailed {
        /// The shape being rasterized.
        shape: &'static str,
        /// The raster buffer width.
        width: u32,
        /// The raster buffer height.
        height: u32,
    },

    // --- Image IO. ---
    /// Returned when an image cannot be written to disk.
    #[error("Failed to save image to '{path}'")]
    SaveImageFailed {
        /// The output path that failed.
        path: String,
        #[source]
        /// The underlying image-encoding error.
        source: image::ImageError,
    },

    /// Returned when image bytes cannot be collected from the pixel buffer.
    #[error("Failed to collect image bytes from pixel buffer")]
    CollectImageBytesFailed {
        #[source]
        /// The underlying I/O error.
        source: std::io::Error,
    },

    /// Returned when an input image file cannot be opened.
    #[error("Failed to open image '{path}'")]
    OpenImageFailed {
        /// The image path that failed.
        path: String,
        #[source]
        /// The underlying I/O error.
        source: std::io::Error,
    },

    /// Returned when an input image file cannot be decoded.
    #[error("Failed to decode image '{path}'")]
    DecodeImageFailed {
        /// The image path that failed.
        path: String,
        #[source]
        /// The underlying decoder error.
        source: image::ImageError,
    },

    /// Returned when an SVG file cannot be read from disk.
    #[error("Failed to read SVG file '{path}'")]
    ReadSvgFailed {
        /// The SVG path/name that failed.
        path: String,
        #[source]
        /// The underlying I/O error.
        source: std::io::Error,
    },

    /// Returned when SVG parsing fails after the file is read.
    #[error("Failed to parse SVG file '{path}'")]
    ParseSvgFailed {
        /// The SVG path/name that failed.
        path: String,
        #[source]
        /// The underlying SVG parser error.
        source: resvg::usvg::Error,
    },

    /// Returned when SVG XML parsing fails.
    #[error("Failed to parse SVG XML in '{path}'")]
    ParseSvgXmlFailed {
        /// The SVG path/name that failed.
        path: String,
        #[source]
        /// The underlying XML parser error.
        source: resvg::usvg::roxmltree::Error,
    },

    /// Returned when an SVG scales down to zero pixels.
    #[error("SVG '{path}' scaled to zero size")]
    SvgScaledToZeroSize {
        /// The SVG path/name that failed.
        path: String,
    },

    /// Returned when a referenced image cannot be located.
    #[error("Image not found: '{name}'")]
    ImageNotFound {
        /// The unresolved image name.
        name: String,
    },

    // --- Typography and font loading. ---
    /// Returned when a font file cannot be read.
    #[error("Failed to read font file '{path}'")]
    ReadFontFileFailed {
        /// The font path that failed.
        path: String,
        #[source]
        /// The underlying I/O error.
        source: std::io::Error,
    },

    /// Returned when glyph shaping cannot obtain a font reference.
    #[error("Failed to create font reference for glyph run")]
    InvalidGlyphRunFontReference,

    /// Wraps an error returned by the fontsource client.
    #[error(transparent)]
    FontSourceError(#[from] FontSourceError),
}

/// A specialized result type for renderer operations.
pub type Result<T> = std::result::Result<T, ImgGenRendererError>;
