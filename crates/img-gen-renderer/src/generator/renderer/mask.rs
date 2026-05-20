use image::RgbaImage;

use super::{ConcreteSize, Renderer};
use crate::{Layer, LayerOffset, Mask, Result};

impl Renderer<'_> {
    pub(super) async fn apply_layer_mask(
        &mut self,
        layer: &Layer,
        mask: &Mask,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        let mut mask_canvas = RgbaImage::new(canvas.width(), canvas.height());
        let mask_size = mask.size.or(layer.size);
        let mask_layer = Layer {
            size: mask_size,
            offset: LayerOffset {
                x: layer.offset.x.saturating_add(mask.offset.x),
                y: layer.offset.y.saturating_add(mask.offset.y),
            },
            background: mask.background.clone(),
            rectangle: mask.rectangle.clone(),
            ellipse: mask.ellipse.clone(),
            polygon: mask.polygon.clone(),
            icon: mask.icon.clone(),
            typography: mask.typography.clone(),
            mask: None,
        };
        let mask_size = ConcreteSize::for_layer(mask_layer.size, ConcreteSize::for_canvas(canvas));
        self.render_layer_unmasked(&mask_layer, mask_size, &mut mask_canvas)
            .await?;

        let layer_raw = canvas.as_mut();
        let mask_raw = mask_canvas.as_raw();

        for (layer_px, mask_px) in layer_raw.chunks_exact_mut(4).zip(mask_raw.chunks_exact(4)) {
            let raw_mask_alpha = mask_px[3];
            let mask_alpha = if mask.invert {
                255u8.saturating_sub(raw_mask_alpha)
            } else {
                raw_mask_alpha
            };

            if mask_alpha == 255 {
                continue;
            }
            if mask_alpha == 0 {
                layer_px[3] = 0;
                continue;
            }

            let layer_alpha = layer_px[3] as u16;
            layer_px[3] = ((layer_alpha * mask_alpha as u16) / 255) as u8;
        }

        Ok(())
    }
}
