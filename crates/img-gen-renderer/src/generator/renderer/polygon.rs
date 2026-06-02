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
                        let degrees = 360.0 / sides as f32;
                        let inset = border_width as f32;
                        let radius = size.width.min(size.height) as f32 / 2.0 - inset;
                        let center_x = size.width as f32 / 2.0;
                        let center_y = size.height as f32 / 2.0;
                        let mut angle = (270.0 - 0.5 * degrees) + l.rotation;

                        let radians = (360.0 - angle).to_radians();
                        let (sin_angle, cos_angle) = radians.sin_cos();
                        pb.move_to(cos_angle * radius + center_x, sin_angle * radius + center_y);

                        for _ in 1..sides {
                            angle += degrees;
                            if angle > 360.0 {
                                angle -= 360.0;
                            }

                            let radians = (360.0 - angle).to_radians();
                            let (sin_angle, cos_angle) = radians.sin_cos();
                            pb.line_to(
                                cos_angle * radius + center_x,
                                sin_angle * radius + center_y,
                            );
                        }
                    }
                    PolygonSides::Irregular(offsets) => {
                        let x_max = size.width.saturating_sub(border_width) as i32;
                        let y_max = size.height.saturating_sub(border_width) as i32;

                        if let Some((first, remaining)) = offsets.as_slice().split_first() {
                            let x = first.x.clamp(border_width as i32, x_max) as f32;
                            let y = first.y.clamp(border_width as i32, y_max) as f32;
                            pb.move_to(x, y);

                            for offset in remaining {
                                let x = offset.x.clamp(border_width as i32, x_max) as f32;
                                let y = offset.y.clamp(border_width as i32, y_max) as f32;
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
