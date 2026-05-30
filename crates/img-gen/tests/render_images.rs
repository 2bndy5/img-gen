use std::{num::NonZeroU32, path::PathBuf};

use img_gen::{
    Background, Generator, Icon, Layer, LayerOffset, Layout, PreserveAspect, Size, SolidColor,
};

mod support;

fn mk_layout(bg: Option<&str>, icon: Option<&str>) -> Layout {
    let mut layout = Layout {
        size: Size {
            width: NonZeroU32::new(300),
            height: NonZeroU32::new(300),
        },
        layers: vec![],
        debug: None,
    };
    let params = [
        (
            PreserveAspect::On,
            Size {
                width: NonZeroU32::new(100),
                height: NonZeroU32::new(100),
            },
            LayerOffset { x: 0, y: 0 },
        ),
        (
            PreserveAspect::Width,
            Size {
                width: NonZeroU32::new(100),
                height: NonZeroU32::new(200),
            },
            LayerOffset { x: 0, y: 100 },
        ),
        (
            PreserveAspect::Height,
            Size {
                width: NonZeroU32::new(200),
                height: NonZeroU32::new(100),
            },
            LayerOffset { x: 100, y: 0 },
        ),
        (
            PreserveAspect::Off,
            Size {
                width: NonZeroU32::new(200),
                height: NonZeroU32::new(200),
            },
            LayerOffset { x: 100, y: 100 },
        ),
    ];
    for (index, (aspect, size, offset)) in params.into_iter().enumerate() {
        let (r, g, b) = (
            255 * [0usize, 1].contains(&index) as u8,
            255 * [0usize, 2].contains(&index) as u8,
            255 * [0usize, 3].contains(&index) as u8,
        );
        let layer = Layer {
            size: Some(size),
            offset,
            background: bg.map(|img| Background {
                image: Some(String::from(img)),
                preserve_aspect: aspect,
                color: Some(SolidColor::new(r, g, b, 127).into()),
            }),
            icon: icon.map(|img| Icon {
                image: String::from(img),
                preserve_aspect: aspect,
                color: Some(SolidColor::new(r, g, b, 127).into()),
            }),
            ..Default::default()
        };
        layout.layers.push(layer);
    }
    layout
}

#[tokio::test]
async fn render_background_png() {
    let layout = mk_layout(Some("tests/message.png"), None);
    let generator = Generator::new(vec![PathBuf::from(".")], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_background[png].png").unwrap();
}

#[tokio::test]
async fn render_background_svg() {
    let layout = mk_layout(Some("tests/message.svg"), None);
    let generator = Generator::new(vec![PathBuf::from("tests/message.svg")], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_background[svg].png").unwrap();
}

#[tokio::test]
async fn render_svg_webfont_text() {
    let layout = mk_layout(None, Some("asset-webfont.svg"));
    let generator = Generator::new(
        vec![PathBuf::from("tests")],
        Some(support::typography_font_cache_root()),
    )
    .unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_svg_webfont_text.png").unwrap();
}

#[tokio::test]
async fn render_icon_png() {
    let layout = mk_layout(None, Some("tests/message.png"));
    let generator = Generator::new(vec![], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_icon[png].png").unwrap();
}

#[tokio::test]
async fn render_icon_svg() {
    let layout = mk_layout(None, Some("tests/message.svg"));
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

#[tokio::test]
async fn blank_bg() {
    let layout = Layout {
        size: Size {
            width: NonZeroU32::new(250),
            height: NonZeroU32::new(250),
        },
        layers: vec![Layer {
            background: Some(Background::default()),
            ..Default::default()
        }],
        debug: None,
    };
    let generator = Generator::new(vec![], None).unwrap();
    generator.render(layout).await.unwrap();
}

#[tokio::test]
async fn preserve_tall() {
    let layout = mk_layout(Some("tests/asset_tall.png"), None);
    let generator = Generator::new(vec![PathBuf::from(".")], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_preserve_tall.png").unwrap();
}
