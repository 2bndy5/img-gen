mod debug;
mod ellipse;
mod images;
mod mask;
mod polygon;
mod rectangle;
mod typography;
use std::{collections::HashSet, path::PathBuf};

use fontsource_downloader::FontSourceClient;
use image::{ImageBuffer, Pixel, RgbaImage, imageops::overlay};
use parley::{FontContext, LayoutContext};
use resvg::{
    tiny_skia::{FillRule, Paint, Path, Pixmap, Stroke, Transform},
    usvg::Options,
};
use swash::scale::ScaleContext;

use crate::{
    Border, ColorKind, ImgGenRendererError, Layer, LayerOffset, Result, Size,
    validators::{HEIGHT, WIDTH},
};

pub struct Renderer<'a> {
    svg_options: Options<'a>,
    font_cx: FontContext,
    layout_cx: LayoutContext<typography::TextBrush>,
    scale_cx: ScaleContext,
    loaded_font_paths: HashSet<PathBuf>,
    fontsource_client: &'a FontSourceClient,
    image_search_paths: &'a [PathBuf],
}

impl<'a> Renderer<'a> {
    pub fn new(
        options: Options<'a>,
        fontsource_client: &'a FontSourceClient,
        image_search_paths: &'a [PathBuf],
    ) -> Self {
        Renderer {
            svg_options: options,
            font_cx: FontContext::new(),
            layout_cx: LayoutContext::new(),
            scale_cx: ScaleContext::new(),
            loaded_font_paths: HashSet::new(),
            fontsource_client,
            image_search_paths,
        }
    }

    /// Fill a given image buffer (`img`) with a specified `color`
    fn colorize_masked(
        color: &ColorKind,
        img: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        mask: Option<&[u8]>,
        only_visible: bool,
    ) {
        let (w, _h) = (img.width(), img.height());
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let old_color = pixel.0;
            if only_visible && old_color[3] == 0 {
                continue;
            }
            let idx = (y * w + x) as usize;
            if let Some(mask) = mask {
                let mask_alpha = mask[idx];
                if mask_alpha == 0 {
                    continue;
                }
                let (r, g, b, a) = color.get_color_tuple_at(x, y);
                let final_alpha = ((mask_alpha as u16 * a as u16) / 255) as u8;
                pixel.blend(&image::Rgba([r, g, b, final_alpha]));
            } else {
                let tup = color.get_color_tuple_at(x, y);
                pixel.0 = [tup.0, tup.1, tup.2, tup.3];
            }
        }
    }

    fn colorize(
        color: &ColorKind,
        img: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        only_visible: bool,
    ) {
        Self::colorize_masked(color, img, None, only_visible)
    }

    fn render_shape(
        path: Path,
        layer_color: &ColorKind,
        shape_size: ConcreteSize,
        layer_offset: &LayerOffset,
        border: &Option<Border>,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        let mut pixmap = Pixmap::new(shape_size.width, shape_size.height).ok_or(
            ImgGenRendererError::PixmapAllocationFailed {
                shape: "shape",
                width: shape_size.width,
                height: shape_size.height,
            },
        )?;
        let mut paint = Paint::default();
        paint.set_color_rgba8(255, 255, 255, 255);
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::EvenOdd,
            Transform::identity(),
            None,
        );
        let mut body = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())
            .ok_or(ImgGenRendererError::RasterBufferConversionFailed {
                shape: "shape",
                width: pixmap.width(),
                height: pixmap.height(),
            })?;
        Self::colorize(layer_color, &mut body, true);
        overlay(canvas, &body, layer_offset.x.into(), layer_offset.y.into());
        if let Some(b) = border {
            let stroke = Stroke {
                width: b.width.get() as f32,
                ..Default::default()
            };
            pixmap = Pixmap::new(shape_size.width, shape_size.height).ok_or(
                ImgGenRendererError::PixmapAllocationFailed {
                    shape: "shape border",
                    width: shape_size.width,
                    height: shape_size.height,
                },
            )?;
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            let mut border =
                RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())
                    .ok_or(ImgGenRendererError::RasterBufferConversionFailed {
                        shape: "shape border",
                        width: pixmap.width(),
                        height: pixmap.height(),
                    })?;
            Self::colorize(&b.color, &mut border, true);
            overlay(
                canvas,
                &border,
                layer_offset.x.into(),
                layer_offset.y.into(),
            );
        }
        Ok(())
    }

    pub async fn render_layer(&mut self, layer: &Layer, canvas: &mut RgbaImage) -> Result<()> {
        let layout_size = ConcreteSize::for_canvas(canvas);
        let size = ConcreteSize::for_layer(layer.size, layout_size);
        if let Some(layer_mask) = layer.mask.as_ref() {
            let mut layer_canvas = RgbaImage::new(canvas.width(), canvas.height());
            self.render_layer_unmasked(layer, size, &mut layer_canvas)
                .await?;
            self.apply_layer_mask(layer, layer_mask, &mut layer_canvas)
                .await?;
            overlay(canvas, &layer_canvas, 0, 0);
            Ok(())
        } else {
            // Fast path: render directly when no mask is defined for this layer.
            self.render_layer_unmasked(layer, size, canvas).await
        }
    }

    pub(super) async fn render_layer_unmasked(
        &mut self,
        layer: &Layer,
        size: ConcreteSize,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        // Render layer attributes in order of priority.
        self.render_background(layer, size, canvas)?;
        self.render_rectangle(layer, size, canvas)?;
        self.render_ellipse(layer, size, canvas)?;
        self.render_polygon(layer, size, canvas)?;
        self.render_icon(layer, size, canvas)?;
        self.render_typography(layer, size, canvas).await?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct ConcreteSize {
    width: u32,
    height: u32,
}

impl ConcreteSize {
    /// Resolve a layer's optional size against the layout's size, falling back
    /// to the layout dimensions when any layer dimension is `None`.
    pub(self) fn for_layer(layer_size: Option<Size>, layout_size: Self) -> Self {
        match layer_size {
            None => layout_size,
            Some(s) => Self {
                width: s.width.map_or(layout_size.width, |value| value.get()),
                height: s.height.map_or(layout_size.height, |value| value.get()),
            },
        }
    }

    pub(self) fn for_canvas(canvas: &RgbaImage) -> Self {
        Self {
            width: canvas.width().max(1),
            height: canvas.height().max(1),
        }
    }
}

impl Default for ConcreteSize {
    fn default() -> Self {
        Self {
            width: WIDTH.get(),
            height: HEIGHT.get(),
        }
    }
}

impl From<&Size> for ConcreteSize {
    fn from(value: &Size) -> Self {
        Self {
            width: value.width.unwrap_or(WIDTH).get(),
            height: value.height.unwrap_or(HEIGHT).get(),
        }
    }
}
