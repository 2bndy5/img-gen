use image::{Rgba, RgbaImage};
use parley::{Alignment, TextWrapMode};

use super::{
    ConcreteSize, Renderer,
    typography::{RenderTextParams, TextMeasureParams},
};
use crate::{Border, Font, Layer, LayerOffset, Layout, Line, Rectangle, Result, SolidColor};

impl Renderer<'_> {
    const DEBUG_LABEL_PADDING: u32 = 1;
    const DEBUG_LABEL_TEXT_Y_SHIFT: i32 = 1;

    pub async fn render_debug(&mut self, layout: &Layout, canvas: &mut RgbaImage) -> Result<()> {
        if let Some(debug) = &layout.debug {
            if !debug.enable {
                return Ok(());
            }
            let bg = debug.color.clone();
            let fg = debug.get_foreground_color();
            let fg_color = fg.clone().into();
            let layout_cs = ConcreteSize::from(&layout.size);
            if debug.grid {
                for y in (debug.grid_step..layout_cs.height).step_by(debug.grid_step as usize) {
                    for x in (debug.grid_step..layout_cs.width).step_by(debug.grid_step as usize) {
                        // draw 3x3 cross centered on (x, y)
                        let bg_tuple = bg.to_tuple();
                        let pixel = Rgba::from([bg_tuple.0, bg_tuple.1, bg_tuple.2, bg_tuple.3]);
                        canvas.put_pixel(x, y, pixel);
                        canvas.put_pixel(x - 1, y, pixel);
                        canvas.put_pixel(x, y - 1, pixel);
                        if x + 1 < canvas.width() {
                            canvas.put_pixel(x + 1, y, pixel);
                        }
                        if y + 1 < canvas.height() {
                            canvas.put_pixel(x, y + 1, pixel);
                        }
                    }
                }
            }
            let outline_layer = Layer {
                rectangle: Some(Rectangle {
                    color: SolidColor::new(0, 0, 0, 0).into(),
                    border: Some(Border {
                        color: bg.clone().into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            };
            self.render_rectangle(&outline_layer, layout_cs, canvas)?;
            let font_size = 12.0;
            let font = Font::default();
            let line = Line::default();

            let (layout_w, layout_h) = (layout_cs.width, layout_cs.height);
            let layout_top_label = " 0 - 0, 0 ";
            let layout_top_offset = LayerOffset { x: 1, y: 1 };
            let layout_top_size = self
                .measure_debug_label(layout_top_label, font_size, layout_w as f32, &font, &line)
                .await?;
            let layout_top_bg_size = Self::debug_label_background_size(layout_top_size);
            let layout_top_bg = Layer {
                offset: layout_top_offset,
                rectangle: Some(Rectangle {
                    color: bg.clone().into(),
                    border: Some(Border {
                        color: fg.clone().into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            };
            self.render_rectangle(&layout_top_bg, layout_top_bg_size, canvas)?;
            let text_offset = LayerOffset {
                x: layout_top_offset.x + Self::DEBUG_LABEL_PADDING as i32,
                y: layout_top_offset.y
                    + Self::DEBUG_LABEL_PADDING as i32
                    + Self::DEBUG_LABEL_TEXT_Y_SHIFT,
            };
            self.render_text(
                canvas,
                layout_top_label,
                RenderTextParams {
                    color: &fg_color,
                    font_size,
                    max_width: layout_w as f32,
                    max_height: font_size.ceil() as u32,
                    layer_offset: &text_offset,
                    font: &font,
                    line: &line,
                    alignment: Alignment::Start,
                    wrap_mode: TextWrapMode::NoWrap,
                    border: None,
                },
            )
            .await?;

            let layout_bottom_label = format!(" 0 - {layout_w}, {layout_h} ");
            let layout_bottom_size = self
                .measure_debug_label(
                    &layout_bottom_label,
                    font_size,
                    layout_w as f32,
                    &font,
                    &line,
                )
                .await?;
            let layout_bottom_bg_size = Self::debug_label_background_size(layout_bottom_size);
            let layout_bottom_offset = LayerOffset {
                x: (layout_w as i32 - layout_bottom_bg_size.width as i32).max(0),
                y: (layout_h as i32 - layout_bottom_bg_size.height as i32).max(0),
            };
            let layout_bottom_bg = Layer {
                offset: layout_bottom_offset,
                rectangle: Some(Rectangle {
                    color: bg.clone().into(),
                    border: Some(Border {
                        color: fg.clone().into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            };
            self.render_rectangle(&layout_bottom_bg, layout_bottom_bg_size, canvas)?;
            let text_offset = LayerOffset {
                x: layout_bottom_offset.x + Self::DEBUG_LABEL_PADDING as i32,
                y: layout_bottom_offset.y
                    + Self::DEBUG_LABEL_PADDING as i32
                    + Self::DEBUG_LABEL_TEXT_Y_SHIFT,
            };
            self.render_text(
                canvas,
                &layout_bottom_label,
                RenderTextParams {
                    color: &fg_color,
                    font_size,
                    max_width: layout_w as f32,
                    max_height: font_size.ceil() as u32,
                    layer_offset: &text_offset,
                    font: &font,
                    line: &line,
                    alignment: Alignment::Start,
                    wrap_mode: TextWrapMode::NoWrap,
                    border: None,
                },
            )
            .await?;

            for (index, layer) in layout.layers.iter().enumerate() {
                let layer_cs = ConcreteSize::for_layer(layer.size, (&layout.size).into());
                // add 1px to each dimension for the outline (zero-based border), clamped to canvas
                let outline_cs = ConcreteSize {
                    width: (layer_cs.width + 1).min(canvas.width()),
                    height: (layer_cs.height + 1).min(canvas.height()),
                };
                let layer_outline = Layer {
                    offset: layer.offset,
                    rectangle: Some(Rectangle {
                        color: SolidColor::new(0, 0, 0, 0).into(),
                        border: Some(Border {
                            color: bg.clone().into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                };
                self.render_rectangle(&layer_outline, outline_cs, canvas)?;
                // Top-left label: show index and layer offset
                let top_label = format!(" {} - {}, {} ", index + 1, layer.offset.x, layer.offset.y);
                let top_offset = LayerOffset {
                    x: layer.offset.x.max(0) + 1,
                    y: layer.offset.y.max(0) + 1,
                };
                let top_size = self
                    .measure_debug_label(&top_label, font_size, layer_cs.width as f32, &font, &line)
                    .await?;
                let top_bg_size = Self::debug_label_background_size(top_size);
                let top_bg = Layer {
                    offset: top_offset,
                    rectangle: Some(Rectangle {
                        color: bg.clone().into(),
                        border: Some(Border {
                            color: fg.clone().into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                };
                self.render_rectangle(&top_bg, top_bg_size, canvas)?;
                let text_offset = LayerOffset {
                    x: top_offset.x + Self::DEBUG_LABEL_PADDING as i32,
                    y: top_offset.y
                        + Self::DEBUG_LABEL_PADDING as i32
                        + Self::DEBUG_LABEL_TEXT_Y_SHIFT,
                };
                self.render_text(
                    canvas,
                    &top_label,
                    RenderTextParams {
                        color: &fg_color,
                        font_size,
                        max_width: layer_cs.width as f32,
                        max_height: top_size.height,
                        layer_offset: &text_offset,
                        font: &font,
                        line: &line,
                        alignment: Alignment::Start,
                        wrap_mode: TextWrapMode::NoWrap,
                        border: None,
                    },
                )
                .await?;

                // Bottom-right label: show index and layer size
                let bottom_label =
                    format!(" {} - {}, {} ", index + 1, layer_cs.width, layer_cs.height,);
                let bottom_size = self
                    .measure_debug_label(
                        &bottom_label,
                        font_size,
                        layer_cs.width as f32,
                        &font,
                        &line,
                    )
                    .await?;
                if bottom_size.width > 0 && bottom_size.height > 0 {
                    let bottom_bg_size = Self::debug_label_background_size(bottom_size);
                    // place label near bottom-right corner inside the layer bounds
                    let bx = (layer.offset.x + layer_cs.width as i32 - bottom_bg_size.width as i32)
                        .max(0);
                    let by = (layer.offset.y + layer_cs.height as i32
                        - bottom_bg_size.height as i32)
                        .max(0);
                    let bottom_offset = LayerOffset { x: bx, y: by };
                    let bottom_bg = Layer {
                        offset: bottom_offset,
                        rectangle: Some(Rectangle {
                            color: bg.clone().into(),
                            border: Some(Border {
                                color: fg.clone().into(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    };
                    self.render_rectangle(&bottom_bg, bottom_bg_size, canvas)?;
                    let text_offset = LayerOffset {
                        x: bottom_offset.x + Self::DEBUG_LABEL_PADDING as i32,
                        y: bottom_offset.y
                            + Self::DEBUG_LABEL_PADDING as i32
                            + Self::DEBUG_LABEL_TEXT_Y_SHIFT,
                    };
                    self.render_text(
                        canvas,
                        &bottom_label,
                        RenderTextParams {
                            color: &fg_color,
                            font_size,
                            max_width: layer_cs.width as f32,
                            max_height: bottom_size.height,
                            layer_offset: &text_offset,
                            font: &font,
                            line: &line,
                            alignment: Alignment::Start,
                            wrap_mode: TextWrapMode::NoWrap,
                            border: None,
                        },
                    )
                    .await?;
                }
            }
        }
        Ok(())
    }

    async fn measure_debug_label(
        &mut self,
        text: &str,
        font_size: f32,
        max_width: f32,
        font: &Font,
        line: &Line,
    ) -> Result<ConcreteSize> {
        let measure = TextMeasureParams {
            max_width: Some(max_width),
            font: Some(font),
            line: Some(line),
            alignment: Alignment::Start,
            wrap_mode: TextWrapMode::NoWrap,
            border_width: 0,
        };
        self.measure_text(text, font_size, measure).await
    }

    fn debug_label_background_size(label_size: ConcreteSize) -> ConcreteSize {
        let pad = Self::DEBUG_LABEL_PADDING * 3;
        ConcreteSize {
            width: label_size.width + pad,
            height: label_size.height + pad,
        }
    }
}
