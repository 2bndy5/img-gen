use std::num::NonZeroU32;

use img_gen::{Arc, Border, Ellipse, Generator, Layer, LayerOffset, Layout, Size, SolidColor};

#[tokio::test]
async fn render() {
    let mut layout = Layout {
        size: Size {
            width: NonZeroU32::new(300),
            height: NonZeroU32::new(300),
        },
        layers: vec![],
        debug: None,
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
        LayerOffset { x: 0, y: 0 },
        LayerOffset { x: 0, y: 100 },
        LayerOffset { x: 100, y: 0 },
        LayerOffset { x: 100, y: 100 },
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
            ellipse: Some(Ellipse {
                color: SolidColor::new(g, b, r, 127).into(),
                border: if index != 0 {
                    Some(Border {
                        color: SolidColor::new(r, g, b, 63).into(),
                        width: NonZeroU32::new(20).unwrap(),
                    })
                } else {
                    None
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        layout.layers.push(layer);
    }
    let generator = Generator::new(vec![], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_ellipse.png").unwrap();
}

#[tokio::test]
async fn render_arcs() {
    let mut layout = Layout {
        size: Size {
            width: NonZeroU32::new(500),
            height: NonZeroU32::new(500),
        },
        layers: vec![],
        ..Default::default()
    };
    let arc_size = Size {
        width: NonZeroU32::new(250),
        height: NonZeroU32::new(250),
    };
    let arcs = [(315.0, 45.0), (45.0, 315.0), (-45.0, 225.0), (225.0, -45.0)];
    for (index, (start, end)) in arcs.iter().enumerate() {
        let (r, g, b) = (
            255 * ([0, 3].contains(&index)) as u8,
            255 * ([1, 3].contains(&index)) as u8,
            255 * ([2, 3].contains(&index)) as u8,
        );
        layout.layers.push(Layer {
            size: Some(arc_size),
            offset: LayerOffset {
                x: 250 * (index as i32 % 2),
                y: 250 * (index as i32 / 2),
            },
            ellipse: Some(Ellipse {
                color: SolidColor::new(g, b, r, 127).into(),
                border: Some(Border {
                    color: SolidColor::new(r, g, b, 63).into(),
                    width: NonZeroU32::new(10).unwrap(),
                }),
                arc: Some(Arc {
                    start: *start,
                    end: *end,
                }),
                border_to_origin: index > 1,
            }),
            ..Default::default()
        });
    }
    let generator = Generator::new(vec![], None).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_ellipse_arcs.png").unwrap();
}
