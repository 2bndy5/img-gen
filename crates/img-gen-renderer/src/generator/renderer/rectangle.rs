use super::{ConcreteSize, Renderer, ellipse::arc_path};
use crate::{Arc, Corners, ImgGenRendererError, Layer, Result};
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
                if max_radius > 0.0
                    && (l.corners.is_empty() || l.corners.contains(&Corners::TopLeft))
                {
                    let cx = max_radius + inset;
                    let cy = max_radius + inset;
                    arc_path(
                        &Arc {
                            start: 90.0,
                            end: 180.0,
                        },
                        (cx, cy),
                        (max_radius, max_radius),
                        &mut pb,
                    );
                } else {
                    pb.move_to(inset, inset);
                }
                if max_radius > 0.0
                    && (l.corners.is_empty() || l.corners.contains(&Corners::BottomLeft))
                {
                    let cx = max_radius + inset;
                    let cy = size.height as f32 - inset - max_radius;
                    arc_path(
                        &Arc {
                            start: 180.0,
                            end: 270.0,
                        },
                        (cx, cy),
                        (max_radius, max_radius),
                        &mut pb,
                    );
                } else {
                    pb.line_to(inset, size.height as f32 - inset);
                }
                if max_radius > 0.0
                    && (l.corners.is_empty() || l.corners.contains(&Corners::BottomRight))
                {
                    let cx = size.width as f32 - max_radius - inset;
                    let cy = size.height as f32 - max_radius - inset;
                    arc_path(
                        &Arc {
                            start: 270.0,
                            end: 0.0,
                        },
                        (cx, cy),
                        (max_radius, max_radius),
                        &mut pb,
                    );
                } else {
                    pb.line_to(size.width as f32 - inset, size.height as f32 - inset);
                }
                if max_radius > 0.0
                    && (l.corners.is_empty() || l.corners.contains(&Corners::TopRight))
                {
                    let cx = size.width as f32 - max_radius - inset;
                    let cy = max_radius + inset;
                    arc_path(
                        &Arc {
                            start: 0.0,
                            end: 90.0,
                        },
                        (cx, cy),
                        (max_radius, max_radius),
                        &mut pb,
                    );
                } else {
                    pb.line_to(size.width as f32 - inset, inset);
                }
                pb.close();
                pb.finish()
                    .ok_or(ImgGenRendererError::InvalidPathBounds { shape: "rectangle" })?
            };
            Self::render_shape(path, &l.color, size, &layer.offset, &l.border, canvas)?;
        }
        Ok(())
    }
}
