use std::num::NonZeroU32;

use img_gen::{
    Border, Corners, Generator, Layer, LayerOffset, Layout, Rectangle, Size, SolidColor,
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

    for (index, (((size, offset), radius), corners)) in sizes
        .into_iter()
        .zip(offsets)
        .zip(radiuses)
        .zip(corners)
        .enumerate()
    {
        let (r, g, b) = (
            255 * [0usize, 1].contains(&index) as u8,
            255 * [0usize, 2].contains(&index) as u8,
            255 * [0usize, 3].contains(&index) as u8,
        );
        let layer = Layer {
            size: Some(size),
            offset,
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
                radius,
                corners,
            }),
            ..Default::default()
        };
        layout.layers.push(layer);
    }
    let generator = Generator::new(vec![], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_rectangle.png").unwrap();
}
