use image::{
    ImageReader, RgbaImage,
    imageops::{FilterType, overlay},
};
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::Tree,
};

use crate::{ImgGenRendererError, Layer, PreserveAspect, Result};

use super::{ConcreteSize, Renderer};

impl Renderer<'_> {
    pub fn render_background(
        &self,
        layer: &Layer,
        size: ConcreteSize,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        if let Some(l) = layer.background.as_ref() {
            let mut img = None;

            // load image data
            if let Some(i) = &l.image {
                img = Some(if i.as_str().ends_with(".svg") {
                    self.load_svg(i.clone(), size, l.preserve_aspect)?
                } else {
                    Self::load_image(i.clone(), size, l.preserve_aspect)?
                });
            }

            // colorize
            if let Some(color) = &l.color {
                let mut over_layer = RgbaImage::new(size.width, size.height);
                Self::colorize(color, &mut over_layer, false);
                if let Some(ref mut pic) = img {
                    overlay(pic, &over_layer, 0, 0);
                } else {
                    img = Some(over_layer);
                }
            }

            if let Some(img) = img {
                overlay(canvas, &img, layer.offset.x.into(), layer.offset.y.into());
            }
        }
        Ok(())
    }

    pub fn render_icon(
        &self,
        layer: &Layer,
        size: ConcreteSize,
        canvas: &mut RgbaImage,
    ) -> Result<()> {
        if let Some(l) = layer.icon.as_ref() {
            // load image data
            let mut img = if l.image.as_str().ends_with(".svg") {
                self.load_svg(l.image.clone(), size, l.preserve_aspect)?
            } else {
                Self::load_image(l.image.clone(), size, l.preserve_aspect)?
            };

            // colorize
            if let Some(color) = &l.color {
                Self::colorize(color, &mut img, true);
            }

            overlay(canvas, &img, layer.offset.x.into(), layer.offset.y.into());
        }
        Ok(())
    }

    fn load_image(
        path: String,
        size: ConcreteSize,
        preserve_aspect: PreserveAspect,
    ) -> Result<RgbaImage> {
        let mut buf = ImageReader::open(path.clone())
            .map_err(|source| ImgGenRendererError::OpenImageFailed {
                path: path.clone(),
                source,
            })?
            .decode()
            .map_err(|source| ImgGenRendererError::DecodeImageFailed {
                path: path.clone(),
                source,
            })?;
        let width = size.width;
        let height = size.height;
        let og_width = buf.width();
        let og_height = buf.height();
        let (new_width, new_height) = match preserve_aspect {
            PreserveAspect::Off => (width, height),
            PreserveAspect::On => {
                if og_width > og_height {
                    (
                        width,
                        (height as f32 * (og_height as f32 / og_width as f32) + 0.5) as u32,
                    )
                } else {
                    (
                        (width as f32 * (og_width as f32 / og_height as f32) + 0.5) as u32,
                        height,
                    )
                }
            }
            PreserveAspect::Width => {
                let ratio = og_height as f32 / og_width as f32;
                (width, (width as f32 * ratio + 0.5) as u32)
            }
            PreserveAspect::Height => {
                let ratio = og_width as f32 / og_height as f32;
                ((height as f32 * ratio + 0.5) as u32, height)
            }
        };
        buf = buf.resize_exact(new_width, new_height, FilterType::CatmullRom);
        let mut img = RgbaImage::new(width, height);
        let offset_x = (width as i64 - buf.width() as i64) / 2;
        let offset_y = (height as i64 - buf.height() as i64) / 2;
        overlay(&mut img, &RgbaImage::from(buf), offset_x, offset_y);
        Ok(img)
    }

    fn load_svg(
        &self,
        path: String,
        size: ConcreteSize,
        preserve_aspect: PreserveAspect,
    ) -> Result<RgbaImage> {
        let tree = {
            let svg_data =
                std::fs::read(&path).map_err(|source| ImgGenRendererError::ReadSvgFailed {
                    path: path.clone(),
                    source,
                })?;
            Tree::from_data(&svg_data, &self.svg_options).map_err(|source| {
                ImgGenRendererError::ParseSvgFailed {
                    path: path.clone(),
                    source,
                }
            })?
        };
        let width = size.width;
        let height = size.height;
        let og_width = tree.size().width();
        let og_height = tree.size().height();
        let (scale_x, scale_y) = match preserve_aspect {
            PreserveAspect::Off => {
                let scale_x = width as f32 / og_width;
                let scale_y = height as f32 / og_height;
                (scale_x, scale_y)
            }
            PreserveAspect::On => {
                let ratio = if og_width > og_height {
                    og_width / width as f32
                } else {
                    og_height / height as f32
                };
                (1.0 / ratio, 1.0 / ratio)
            }
            PreserveAspect::Width => {
                let ratio = og_width / width as f32;
                (1.0 / ratio, 1.0 / ratio)
            }
            PreserveAspect::Height => {
                let ratio = og_height / height as f32;
                (1.0 / ratio, 1.0 / ratio)
            }
        };
        let mut pixmap = Pixmap::new((og_width * scale_x) as u32, (og_height * scale_y) as u32)
            .ok_or(ImgGenRendererError::SvgScaledToZeroSize { path: path.clone() })?;
        resvg::render(
            &tree,
            Transform::from_scale(scale_x, scale_y),
            &mut pixmap.as_mut(),
        );
        let mut img = RgbaImage::new(width, height);
        let offset_x = (width as i64 - pixmap.width() as i64) / 2;
        let offset_y = (height as i64 - pixmap.height() as i64) / 2;
        let svg = RgbaImage::from_raw(pixmap.width(), pixmap.height(), Vec::from(pixmap.data()))
            .ok_or(ImgGenRendererError::RasterBufferConversionFailed {
                shape: "svg",
                width: pixmap.width(),
                height: pixmap.height(),
            })?;
        overlay(&mut img, &svg, offset_x, offset_y);
        Ok(img)
    }
}
