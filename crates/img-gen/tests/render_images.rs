use std::num::NonZeroU32;

use img_gen::{
    Background, Generator, Icon, Layer, LayerOffset, Layout, PreserveAspect, Size, SolidColor,
};

#[tokio::test]
async fn render_background_png() {
    let layer_top_left = Layer {
        size: Some(Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(100),
        }),
        offset: LayerOffset::default(),
        background: Some(Background {
            image: Some(String::from("tests/message.png")),
            color: Some(SolidColor::new(255, 255, 255, 127).into()),
            preserve_aspect: PreserveAspect::On,
        }),
        ..Default::default()
    };
    let layer_bottom_left = Layer {
        size: Some(Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(200),
        }),
        offset: LayerOffset { x: 0, y: 100 },
        background: Some(Background {
            image: Some(String::from("tests/message.png")),
            color: Some(SolidColor::new(255, 0, 0, 127).into()),
            preserve_aspect: PreserveAspect::Width,
        }),
        ..Default::default()
    };
    let layer_top_right = Layer {
        size: Some(Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(100),
        }),
        offset: LayerOffset { x: 100, y: 0 },
        background: Some(Background {
            image: Some(String::from("tests/message.png")),
            color: Some(SolidColor::new(0, 255, 0, 127).into()),
            preserve_aspect: PreserveAspect::Height,
        }),
        ..Default::default()
    };
    let layer_bottom_right = Layer {
        size: Some(Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(200),
        }),
        offset: LayerOffset { x: 100, y: 100 },
        background: Some(Background {
            image: Some(String::from("tests/message.png")),
            color: Some(SolidColor::new(0, 0, 255, 127).into()),
            preserve_aspect: PreserveAspect::Off,
        }),
        ..Default::default()
    };
    let layout = Layout {
        size: Size {
            width: NonZeroU32::new(300),
            height: NonZeroU32::new(300),
        },
        layers: vec![
            layer_top_left,
            layer_top_right,
            layer_bottom_left,
            layer_bottom_right,
        ],
        debug: None,
    };
    let generator = Generator::new(vec![], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_background[png].png").unwrap();
}

#[tokio::test]
async fn render_background_svg() {
    let layer_top_left = Layer {
        size: Some(Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(100),
        }),
        offset: LayerOffset::default(),
        background: Some(Background {
            image: Some(String::from("tests/message.svg")),
            color: Some(SolidColor::new(255, 255, 255, 127).into()),
            preserve_aspect: PreserveAspect::On,
        }),
        ..Default::default()
    };
    let layer_bottom_left = Layer {
        size: Some(Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(200),
        }),
        offset: LayerOffset { x: 0, y: 100 },
        background: Some(Background {
            image: Some(String::from("tests/message.svg")),
            color: Some(SolidColor::new(255, 0, 0, 127).into()),
            preserve_aspect: PreserveAspect::Width,
        }),
        ..Default::default()
    };
    let layer_top_right = Layer {
        size: Some(Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(100),
        }),
        offset: LayerOffset { x: 100, y: 0 },
        background: Some(Background {
            image: Some(String::from("tests/message.svg")),
            color: Some(SolidColor::new(0, 255, 0, 127).into()),
            preserve_aspect: PreserveAspect::Height,
        }),
        ..Default::default()
    };
    let layer_bottom_right = Layer {
        size: Some(Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(200),
        }),
        offset: LayerOffset { x: 100, y: 100 },
        background: Some(Background {
            image: Some(String::from("tests/message.svg")),
            color: Some(SolidColor::new(0, 0, 255, 127).into()),
            preserve_aspect: PreserveAspect::Off,
        }),
        ..Default::default()
    };
    let layout = Layout {
        size: Size {
            width: NonZeroU32::new(300),
            height: NonZeroU32::new(300),
        },
        layers: vec![
            layer_top_left,
            layer_top_right,
            layer_bottom_left,
            layer_bottom_right,
        ],
        debug: None,
    };
    let generator = Generator::new(vec![], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_background[svg].png").unwrap();
}

#[tokio::test]
async fn render_icon_png() {
    let layer_top_left = Layer {
        size: Some(Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(100),
        }),
        offset: LayerOffset::default(),
        icon: Some(Icon {
            image: String::from("tests/message.png"),
            color: Some(SolidColor::new(255, 255, 255, 127).into()),
            preserve_aspect: PreserveAspect::On,
        }),
        ..Default::default()
    };
    let layer_bottom_left = Layer {
        size: Some(Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(200),
        }),
        offset: LayerOffset { x: 0, y: 100 },
        icon: Some(Icon {
            image: String::from("tests/message.png"),
            color: Some(SolidColor::new(255, 0, 0, 127).into()),
            preserve_aspect: PreserveAspect::Width,
        }),
        ..Default::default()
    };
    let layer_top_right = Layer {
        size: Some(Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(100),
        }),
        offset: LayerOffset { x: 100, y: 0 },
        icon: Some(Icon {
            image: String::from("tests/message.png"),
            color: Some(SolidColor::new(0, 255, 0, 127).into()),
            preserve_aspect: PreserveAspect::Height,
        }),
        ..Default::default()
    };
    let layer_bottom_right = Layer {
        size: Some(Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(200),
        }),
        offset: LayerOffset { x: 100, y: 100 },
        icon: Some(Icon {
            image: String::from("tests/message.png"),
            color: Some(SolidColor::new(0, 0, 255, 127).into()),
            preserve_aspect: PreserveAspect::Off,
        }),
        ..Default::default()
    };
    let layout = Layout {
        size: Size {
            width: NonZeroU32::new(300),
            height: NonZeroU32::new(300),
        },
        layers: vec![
            layer_top_left,
            layer_top_right,
            layer_bottom_left,
            layer_bottom_right,
        ],
        debug: None,
    };
    let generator = Generator::new(vec![], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_icon[png].png").unwrap();
}

#[tokio::test]
async fn render_icon_svg() {
    let layer_top_left = Layer {
        size: Some(Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(100),
        }),
        offset: LayerOffset::default(),
        icon: Some(Icon {
            image: String::from("tests/message.svg"),
            color: Some(SolidColor::new(255, 255, 255, 127).into()),
            preserve_aspect: PreserveAspect::On,
        }),
        ..Default::default()
    };
    let layer_bottom_left = Layer {
        size: Some(Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(200),
        }),
        offset: LayerOffset { x: 0, y: 100 },
        icon: Some(Icon {
            image: String::from("tests/message.svg"),
            color: Some(SolidColor::new(255, 0, 0, 127).into()),
            preserve_aspect: PreserveAspect::Width,
        }),
        ..Default::default()
    };
    let layer_top_right = Layer {
        size: Some(Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(100),
        }),
        offset: LayerOffset { x: 100, y: 0 },
        icon: Some(Icon {
            image: String::from("tests/message.svg"),
            color: Some(SolidColor::new(0, 255, 0, 127).into()),
            preserve_aspect: PreserveAspect::Height,
        }),
        ..Default::default()
    };
    let layer_bottom_right = Layer {
        size: Some(Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(200),
        }),
        offset: LayerOffset { x: 100, y: 100 },
        icon: Some(Icon {
            image: String::from("tests/message.svg"),
            color: Some(SolidColor::new(0, 0, 255, 127).into()),
            preserve_aspect: PreserveAspect::Off,
        }),
        ..Default::default()
    };
    let layout = Layout {
        size: Size {
            width: NonZeroU32::new(300),
            height: NonZeroU32::new(300),
        },
        layers: vec![
            layer_top_left,
            layer_top_right,
            layer_bottom_left,
            layer_bottom_right,
        ],
        debug: None,
    };
    let generator = Generator::new(vec![], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_icon[svg].png").unwrap();
}

#[tokio::test]
async fn render_builtin_svg() {
    let mut layout = Layout {
        size: Size {
            width: NonZeroU32::new(500),
            height: NonZeroU32::new(500),
        },
        layers: vec![],
        ..Default::default()
    };
    let layer_size = Size {
        width: NonZeroU32::new(250),
        height: NonZeroU32::new(250),
    };
    let icons = [
        "material/cat",
        "simple/rust",
        "octicons/mark-github-16",
        "fontawesome/solid/language",
    ];
    for (index, icon) in icons.iter().enumerate() {
        let (r, g, b) = (
            255 * [0, 1].contains(&index) as u8,
            255 * [0, 2].contains(&index) as u8,
            255 * [0, 3].contains(&index) as u8,
        );
        let layer = Layer {
            size: Some(layer_size),
            offset: LayerOffset {
                x: 250 * (index as i32 % 2),
                y: 250 * (index as i32 / 2),
            },
            icon: Some(Icon {
                image: String::from(*icon),
                color: Some(SolidColor::new(r, g, b, 127).into()),
                preserve_aspect: PreserveAspect::On,
            }),
            ..Default::default()
        };
        layout.layers.push(layer);
    }
    let generator = Generator::new(vec![], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/render_builtin_icons.png").unwrap();
}
