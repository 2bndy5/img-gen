use std::num::NonZeroU32;

use img_gen::Generator;
use img_gen::validators::{
    Border, Corners, Layer, LayerOffset, Layout, Rectangle, Size, SolidColor,
};

#[tokio::test]
async fn render() {
    let mut layout = Layout {
        size: Size {
            width: NonZeroU32::new(300),
            height: NonZeroU32::new(300),
        },
        ..Default::default()
    };
    let sizes = [
        Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(100),
        },
        Size {
            width: NonZeroU32::new(100),
            height: NonZeroU32::new(200),
        },
        Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(100),
        },
        Size {
            width: NonZeroU32::new(200),
            height: NonZeroU32::new(200),
        },
    ];
    let offsets = [
        LayerOffset::default(),
        LayerOffset { x: 0, y: 100 },
        LayerOffset { x: 100, y: 0 },
        LayerOffset { x: 100, y: 100 },
    ];
    let radiuses = [0.0, 50.0, 100.0, 50.0];
    let corners = [
        vec![],
        vec![Corners::TopLeft, Corners::BottomRight],
        vec![Corners::TopRight, Corners::BottomLeft],
        Corners::ALL.to_vec(),
    ];

    for index in 0..sizes.len() {
        let (r, g, b) = (
            255 * [0usize, 1].contains(&index) as u8,
            255 * [0usize, 2].contains(&index) as u8,
            255 * [0usize, 3].contains(&index) as u8,
        );
        let layer = Layer {
            size: Some(sizes[index]),
            offset: offsets[index],
            rectangle: Some(Rectangle {
                color: SolidColor::new(g, b, r, 127).into(),
                border: if index != 0 {
                    Some(Border {
                        color: SolidColor::new(r, g, b, 63).into(),
                        width: NonZeroU32::new(20).unwrap(),
                    })
                } else {
                    None
                },
                radius: radiuses[index],
                corners: corners[index].clone(),
            }),
            ..Default::default()
        };
        layout.layers.push(layer);
    }
    let generator = Generator { layout };
    let img = generator.render().await.unwrap();
    img.save("tests/out/test_rectangle.png").unwrap();
}
