use super::{ConcreteSize, Renderer};
use crate::{ImgGenRendererError, Layer, Result};
use img_gen_spec::TRANSPARENT;

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

            // Common float helpers
            let fw = size.width as f32;
            let fh = size.height as f32;
            let cx = fw / 2.0;
            let cy = fh / 2.0;
            let rx = fw / 2.0 - inset;
            let ry = fh / 2.0 - inset;

            if let Some(arc) = l.arc.as_ref() {
                let mut start = arc.start % 360.0;
                if start < 0.0 {
                    start += 360.0;
                }
                let mut end = arc.end % 360.0;
                if end < 0.0 {
                    end += 360.0;
                }
                let mut delta = end - start;
                if delta <= 0.0 {
                    delta += 360.0;
                }

                // Number of segments to approximate the arc. ~1 segment per 5 degrees.
                let segments = (delta / 5.0).ceil().max(2.0) as usize;

                // Build two paths:
                //  - sector_path: closed path (center -> arc -> center) used for filling the wedge
                //  - arc_path: open path following just the arc used for stroking when border_to_origin is false
                let mut sector_pb = PathBuilder::new();
                let mut arc_pb = PathBuilder::new();

                // compute first point separately to initialize paths
                let angle0 = (start).to_radians();
                let x0 = cx + rx * angle0.cos();
                let y0 = cy + ry * angle0.sin();

                sector_pb.move_to(cx, cy);
                sector_pb.line_to(x0, y0);

                arc_pb.move_to(x0, y0);

                for i in 1..=segments {
                    let t = i as f32 / segments as f32;
                    let angle = (start + delta * t).to_radians();
                    let x = cx + rx * angle.cos();
                    let y = cy + ry * angle.sin();
                    sector_pb.line_to(x, y);
                    arc_pb.line_to(x, y);
                }

                sector_pb.line_to(cx, cy);
                sector_pb.close();

                let sector_path =
                    sector_pb
                        .finish()
                        .ok_or(ImgGenRendererError::InvalidPathBounds {
                            shape: "ellipse arc",
                        })?;
                let arc_path = arc_pb
                    .finish()
                    .ok_or(ImgGenRendererError::InvalidPathBounds {
                        shape: "ellipse arc",
                    })?;

                // Fill the sector with the ellipse color (no stroke)
                Self::render_shape(
                    sector_path.clone(),
                    &l.color,
                    size,
                    &layer.offset,
                    &None,
                    canvas,
                )?;

                // Draw border if requested. If border_to_origin=false, stroke only the arc path;
                // if true, stroke the whole sector path (including radial edges).
                if l.border.is_some() {
                    let transparent = TRANSPARENT.into();
                    let border_ref = &l.border;
                    if l.border_to_origin {
                        Self::render_shape(
                            sector_path,
                            &transparent,
                            size,
                            &layer.offset,
                            border_ref,
                            canvas,
                        )?;
                    } else {
                        Self::render_shape(
                            arc_path,
                            &transparent,
                            size,
                            &layer.offset,
                            border_ref,
                            canvas,
                        )?;
                    }
                }
            } else {
                // Full ellipse as before.
                let path = PathBuilder::from_oval(
                    Rect::from_xywh(
                        inset,
                        inset,
                        size.width as f32 - border_width as f32,
                        size.height as f32 - border_width as f32,
                    )
                    .ok_or(ImgGenRendererError::BoundsTooLarge { shape: "ellipse" })?,
                )
                .ok_or(ImgGenRendererError::InvalidPathBounds { shape: "ellipse" })?;
                Self::render_shape(path, &l.color, size, &layer.offset, &l.border, canvas)?;
            }
        }
        Ok(())
    }
}
