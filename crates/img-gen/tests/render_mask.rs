use std::num::NonZeroU32;

use img_gen::{
    Background, Corners, Generator, Layer, LayerOffset, Layout, Mask, Rectangle, Size, SolidColor,
};

fn mk_mask(inverted: bool) -> Mask {
    Mask {
        size: Some(Size {
            width: NonZeroU32::new(40),
            height: NonZeroU32::new(40),
        }),
        offset: LayerOffset { x: 30, y: 30 },
        invert: inverted,
        rectangle: Some(Rectangle {
            color: SolidColor::new(255, 255, 255, 255).into(),
            radius: 15.0,
            corners: vec![Corners::TopRight, Corners::BottomLeft],
            border: None,
        }),
        ..Default::default()
    }
}

fn mk_layout(inverted: bool) -> Layout {
    let layer = Layer {
        size: Some(Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(100),
        }),
        offset: LayerOffset::default(),
        mask: Some(mk_mask(inverted)),
        background: Some(Background {
            color: Some(SolidColor::new(255, 0, 0, 255).into()),
            image: None,
            preserve_aspect: img_gen::PreserveAspect::Off,
        }),
        ..Default::default()
    };
    let layers = vec![layer];
    Layout {
        size: Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(100),
        },
        layers,
        debug: None,
    }
}

#[tokio::test]
async fn render_mask() {
    let layout = mk_layout(false);

    let img = Generator { layout }.render().await.unwrap();
    assert_eq!(img.data.get_pixel(50, 50).0[3], 255);
    assert_eq!(img.data.get_pixel(10, 10).0[3], 0);
    img.save("tests/out/test_mask.png").unwrap();
}

#[tokio::test]
async fn render_mask_inverted() {
    let layout = mk_layout(true);

    let img = Generator { layout }.render().await.unwrap();
    assert_eq!(img.data.get_pixel(50, 50).0[3], 0);
    assert_eq!(img.data.get_pixel(10, 10).0[3], 255);
    img.save("tests/out/test_mask_inverted.png").unwrap();
}
