use super::{ConcreteSize, Renderer};
use crate::{ImgGenError, Layer, Result};
use image::RgbaImage;
use resvg::{tiny_skia::PathBuilder, usvg::Rect};

impl Renderer<'_> {
    pub fn render_ellipse(
        &self,
        layer: &Layer,
        size: ConcreteSize,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        if let Some(l) = layer.ellipse.as_ref() {
            let border_width = l.border.as_ref().map_or(0, |b| b.width.get());
            let inset = border_width as f32 / 2.0;
            let path = PathBuilder::from_oval(
                Rect::from_xywh(
                    inset,
                    inset,
                    size.width as f32 - border_width as f32,
                    size.height as f32 - border_width as f32,
                )
                .ok_or(ImgGenError::BoundsTooLarge { shape: "ellipse" })?,
            )
            .ok_or(ImgGenError::InvalidPathBounds { shape: "ellipse" })?;
            Self::render_shape(path, &l.color, size, &layer.offset, &l.border, canvas)?;
        }
        Ok(())
    }
}
