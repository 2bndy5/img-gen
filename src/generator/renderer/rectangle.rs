use super::{ConcreteSize, Renderer};
use crate::{Corners, ImgGenError, Layer, Result};
use image::RgbaImage;
use resvg::tiny_skia::PathBuilder;

impl Renderer<'_> {
    pub fn render_rectangle(
        &self,
        layer: &Layer,
        size: ConcreteSize,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        if let Some(l) = layer.rectangle.as_ref() {
            let border_width = l.border.as_ref().map_or(0, |b| b.width.get());
            let max_radius = (size.width.min(size.height) as f32 / 2.0).min(l.radius);
            let path = {
                let mut pb = PathBuilder::new();
                let inset = border_width as f32 / 2.0;
                if l.corners.contains(&Corners::TopLeft) && max_radius > 0.0 {
                    pb.move_to(inset, max_radius);
                    pb.quad_to(inset, inset, max_radius + inset, inset);
                } else {
                    pb.move_to(inset, inset);
                }
                if l.corners.contains(&Corners::TopRight) && max_radius > 0.0 {
                    pb.line_to(size.width as f32 - inset - max_radius, inset);
                    pb.quad_to(
                        size.width as f32 - inset,
                        inset,
                        size.width as f32 - inset,
                        max_radius,
                    );
                } else {
                    pb.line_to(size.width as f32 - inset, inset);
                }
                if l.corners.contains(&Corners::BottomRight) && max_radius > 0.0 {
                    pb.line_to(
                        size.width as f32 - inset,
                        size.height as f32 - max_radius - inset,
                    );
                    pb.quad_to(
                        size.width as f32 - inset,
                        size.height as f32 - inset,
                        size.width as f32 - inset - max_radius,
                        size.height as f32 - inset,
                    );
                } else {
                    pb.line_to(size.width as f32 - inset, size.height as f32 - inset);
                }
                if l.corners.contains(&Corners::BottomLeft) && max_radius > 0.0 {
                    pb.line_to(inset + max_radius, size.height as f32 - inset);
                    pb.quad_to(
                        inset,
                        size.height as f32 - inset,
                        inset,
                        size.height as f32 - inset - max_radius,
                    );
                } else {
                    pb.line_to(inset, size.height as f32 - inset);
                }
                pb.close();
                pb.finish()
                    .ok_or(ImgGenError::InvalidPathBounds { shape: "rectangle" })?
            };
            Self::render_shape(path, &l.color, size, &layer.offset, &l.border, canvas)?;
        }
        Ok(())
    }
}
