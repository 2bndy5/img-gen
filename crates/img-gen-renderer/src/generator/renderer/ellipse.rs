use super::{ConcreteSize, Renderer};
use crate::{ImgGenRendererError, Layer, Result};
use img_gen_spec::{Arc, Ellipse, LayerOffset};

use image::{RgbaImage, imageops::overlay};
use resvg::{
    tiny_skia::{FillRule, Mask, MaskType, Paint, Path, PathBuilder, Pixmap, PixmapRef, Stroke},
    usvg::{Rect, Transform},
};

fn clamp_to_360(angle: f32) -> f32 {
    let mut a = angle % 360.0;
    if a < 0.0 {
        a += 360.0;
    }
    a
}

pub(super) fn arc_path(arc: &Arc, center: (f32, f32), radiuses: (f32, f32), pb: &mut PathBuilder) {
    let start = clamp_to_360(arc.start);
    let end = clamp_to_360(arc.end);
    let delta = clamp_to_360(end - start);

    // Number of segments to approximate the arc. ~1 segment per 5 degrees.
    let segments = (delta / 5.0).ceil().max(2.0) as usize;

    let (cx, cy) = center;
    let (rx, ry) = radiuses;
    // compute first point separately to initialize path
    let angle0 = (360.0 - start).to_radians();
    let x0 = cx + rx * angle0.cos();
    let y0 = cy + ry * angle0.sin();

    if pb.is_empty() {
        pb.move_to(x0, y0);
    } else {
        pb.line_to(x0, y0);
    }

    for i in 1..=segments {
        let t = i as f32 / segments as f32;
        let angle = (360.0 - (start + delta * t)).to_radians();
        let x = cx + rx * angle.cos();
        let y = cy + ry * angle.sin();
        pb.line_to(x, y);
    }
}

fn draw_arc(arc: &Arc, size: ConcreteSize, inset: f32) -> Result<Path> {
    // Common float helpers
    let fw = size.width as f32;
    let fh = size.height as f32;
    let cx = fw / 2.0;
    let cy = fh / 2.0;
    let rx = fw / 2.0 - inset;
    let ry = fh / 2.0 - inset;
    // Build the sector path (center -> arc -> center) used for filling the wedge
    let mut sector_pb = PathBuilder::new();
    sector_pb.move_to(cx, cy);
    arc_path(arc, (cx, cy), (rx, ry), &mut sector_pb);
    sector_pb.line_to(cx, cy);
    sector_pb.close();

    sector_pb
        .finish()
        .ok_or(ImgGenRendererError::InvalidPathBounds {
            shape: "ellipse arc",
        })
}

impl Renderer<'_> {
    /// Render an arc with border.
    ///
    /// Needed a dedicated function because drawing a border for an arc is tricky.
    fn render_arc_with_border(
        &self,
        layer: &Ellipse,
        size: ConcreteSize,
        layer_offset: &LayerOffset,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        let (Some(arc), Some(border)) = (layer.arc.as_ref(), layer.border.as_ref()) else {
            return Ok(()); // unreachable since caller checks for arc presence.
        };
        let border_width = border.width.get() as f32;
        let inset = border_width / 2.0;

        let mut paint = Paint::default();
        paint.set_color_rgba8(255, 255, 255, 255);

        // only stroke the arc with a border.
        // To get a border whose ends align with the arc region's edges,
        // we must draw an entire ellipse with a border and
        // mask out the region beyond the arc bounds.

        // create our mask of the arc. what results from masking is what gets colored.
        let mask_path = draw_arc(arc, size, 0.0)?;
        let mut pixmap_mask = Pixmap::new(size.width, size.height).ok_or(
            ImgGenRendererError::PixmapAllocationFailed {
                shape: "shape border mask",
                width: size.width,
                height: size.height,
            },
        )?;
        pixmap_mask.fill_path(
            &mask_path,
            &paint,
            FillRule::EvenOdd,
            Transform::identity(),
            None,
        );
        let mask = Mask::from_pixmap(
            PixmapRef::from_bytes(pixmap_mask.data(), size.width, size.height).ok_or(
                ImgGenRendererError::RasterBufferConversionFailed {
                    shape: "arc border mask",
                    width: size.width,
                    height: size.height,
                },
            )?,
            MaskType::Alpha,
        );

        // create the ellipse path used for both arc and border.
        let ellipse_path = PathBuilder::from_oval(
            // spell-checker: disable-next-line
            Rect::from_xywh(
                inset,
                inset,
                size.width as f32 - border_width,
                size.height as f32 - border_width,
            )
            .ok_or(ImgGenRendererError::BoundsTooLarge {
                shape: "ellipse arc border",
            })?,
        )
        .ok_or(ImgGenRendererError::InvalidPathBounds {
            shape: "ellipse arc border",
        })?;

        let mut pixmap = Pixmap::new(size.width, size.height).ok_or(
            ImgGenRendererError::PixmapAllocationFailed {
                shape: "ellipse arc border",
                width: size.width,
                height: size.height,
            },
        )?;

        if !layer.color.is_transparent() {
            // first fill the ellipse path and mask it to the arc region.
            pixmap.fill_path(
                &ellipse_path,
                &paint,
                FillRule::EvenOdd,
                Transform::identity(),
                Some(&mask),
            );
            // color the fill using a temp canvas
            let mut tmp_canvas =
                RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())
                    .ok_or(ImgGenRendererError::RasterBufferConversionFailed {
                        shape: "ellipse arc",
                        width: pixmap.width(),
                        height: pixmap.height(),
                    })?;
            Self::colorize(&layer.color, &mut tmp_canvas, true);
            overlay(
                canvas,
                &tmp_canvas,
                layer_offset.x.into(),
                layer_offset.y.into(),
            );
        }

        if !border.color.is_transparent() {
            // draw the border with same path
            let mut border_pixmap = Pixmap::new(size.width, size.height).ok_or(
                ImgGenRendererError::PixmapAllocationFailed {
                    shape: "ellipse arc border",
                    width: size.width,
                    height: size.height,
                },
            )?;
            let stroke = Stroke {
                width: border_width,
                ..Default::default()
            };
            border_pixmap.stroke_path(
                &ellipse_path,
                &paint,
                &stroke,
                Transform::identity(),
                Some(&mask),
            );
            // color the border using a temp canvas
            let mut border_canvas = RgbaImage::from_raw(
                border_pixmap.width(),
                border_pixmap.height(),
                border_pixmap.data().to_vec(),
            )
            .ok_or(ImgGenRendererError::RasterBufferConversionFailed {
                shape: "ellipse arc border",
                width: border_pixmap.width(),
                height: border_pixmap.height(),
            })?;
            Self::colorize(&border.color, &mut border_canvas, true);
            overlay(
                canvas,
                &border_canvas,
                layer_offset.x.into(),
                layer_offset.y.into(),
            );
        }

        Ok(())
    }

    pub fn render_ellipse(
        &self,
        layer: &Layer,
        size: ConcreteSize,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        if let Some(l) = layer.ellipse.as_ref() {
            if l.arc.is_some() && l.border.is_some() && !l.border_to_origin {
                self.render_arc_with_border(l, size, &layer.offset, canvas)?;
            } else {
                let border_width = l.border.as_ref().map_or(0, |b| b.width.get());
                let inset = border_width as f32 / 2.0;
                let path = if let Some(arc) = &l.arc {
                    draw_arc(arc, size, inset)?
                } else {
                    PathBuilder::from_oval(
                        // spell-checker: disable-next-line
                        Rect::from_xywh(
                            inset,
                            inset,
                            size.width as f32 - border_width as f32,
                            size.height as f32 - border_width as f32,
                        )
                        .ok_or(ImgGenRendererError::BoundsTooLarge { shape: "ellipse" })?,
                    )
                    .ok_or(ImgGenRendererError::InvalidPathBounds { shape: "ellipse" })?
                };
                Self::render_shape(path, &l.color, size, &layer.offset, &l.border, canvas)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use std::num::NonZeroU32;

    use fontsource_downloader::FontSourceClient;
    use img_gen_spec::{Border, SolidColor};

    use super::*;

    #[test]
    fn non_op_render_arc() {
        let ellipse = Ellipse::default();
        let fontsource_client = FontSourceClient::new().unwrap();
        let renderer = Renderer::new(resvg::usvg::Options::default(), &fontsource_client, &[]);
        let mut canvas = RgbaImage::new(100, 100);
        renderer
            .render_arc_with_border(
                &ellipse,
                ConcreteSize::default(),
                &LayerOffset::default(),
                &mut canvas,
            )
            .unwrap();
        // rendering an arc with no arc or border should be a no-op and not modify the canvas.
        assert!(canvas.pixels().all(|p| *p == image::Rgba([0, 0, 0, 0])));
    }

    #[test]
    fn render_transparent_arc() {
        let transparent_color = SolidColor::new(0, 0, 0, 0);
        let ellipse = Ellipse {
            arc: Some(Arc {
                start: 0.0,
                end: 90.0,
            }),
            color: transparent_color.clone().into(),
            border: Some(Border {
                width: NonZeroU32::new(10).unwrap(),
                color: transparent_color.into(),
            }),
            ..Default::default()
        };
        let fontsource_client = FontSourceClient::new().unwrap();
        let renderer = Renderer::new(resvg::usvg::Options::default(), &fontsource_client, &[]);
        let mut canvas = RgbaImage::new(100, 100);
        renderer
            .render_arc_with_border(
                &ellipse,
                ConcreteSize::default(),
                &LayerOffset::default(),
                &mut canvas,
            )
            .unwrap();
        // rendering an arc with a transparent border and fill color should modify the canvas.
        assert!(canvas.pixels().all(|p| *p == image::Rgba([0, 0, 0, 0])));
    }
}
