use super::{ConcreteSize, Renderer};
use crate::{ImgGenRendererError, Layer, Result};
use image::RgbaImage;
use resvg::tiny_skia::PathBuilder;

impl Renderer<'_> {
    pub fn render_polygon(
        &self,
        layer: &Layer,
        size: ConcreteSize,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        if let Some(l) = layer.polygon.as_ref() {
            let border_width = l.border.as_ref().map_or(0, |b| b.width.get());

            let path = {
                let mut pb = PathBuilder::new();
                // l.get_sides() is guaranteed to be 3 or more
                let radius = size.width.min(size.height) as f32 / 2.0 - border_width as f32;
                for i in 0..l.sides.get() {
                    let angle =
                        ((360.0 / l.sides.get() as f32 * i as f32) - l.rotation).to_radians();
                    let x = angle.cos() * radius + (size.width as f32 / 2.0);
                    let y = angle.sin() * radius + (size.height as f32 / 2.0);
                    if i == 0 {
                        pb.move_to(x, y);
                    } else {
                        pb.line_to(x, y);
                    }
                }
                pb.close();
                pb.finish()
                    .ok_or(ImgGenRendererError::InvalidPathBounds { shape: "polygon" })?
            };
            Self::render_shape(path, &l.color, size, &layer.offset, &l.border, canvas)?
        }
        Ok(())
    }
}
