use super::{ConcreteSize, Renderer};
use crate::{ImgGenRendererError, Layer, Result};
use image::RgbaImage;
use img_gen_spec::PolygonSides;
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
                match &l.sides {
                    PolygonSides::Regular(sides) => {
                        let sides = sides.get();
                        let radius = size.width.min(size.height) as f32 / 2.0 - border_width as f32;
                        for i in 0..sides {
                            let angle =
                                ((360.0 / sides as f32 * i as f32) - l.rotation).to_radians();
                            let x = angle.cos() * radius + (size.width as f32 / 2.0);
                            let y = angle.sin() * radius + (size.height as f32 / 2.0);
                            if i == 0 {
                                pb.move_to(x, y);
                            } else {
                                pb.line_to(x, y);
                            }
                        }
                    }
                    PolygonSides::Irregular(offsets) => {
                        let x_min = if border_width > 0 {
                            border_width as i32
                        } else {
                            0
                        };
                        let y_min = if border_width > 0 {
                            border_width as i32
                        } else {
                            0
                        };
                        let x_max = if border_width > 0 {
                            size.width.saturating_sub(border_width) as i32
                        } else {
                            size.width as i32
                        };
                        let y_max = if border_width > 0 {
                            size.height.saturating_sub(border_width) as i32
                        } else {
                            size.height as i32
                        };

                        for (i, offset) in offsets.as_slice().iter().enumerate() {
                            let x = offset.x.clamp(x_min, x_max) as f32;
                            let y = offset.y.clamp(y_min, y_max) as f32;
                            if i == 0 {
                                pb.move_to(x, y);
                            } else {
                                pb.line_to(x, y);
                            }
                        }
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
