use std::num::NonZeroU32;

use img_gen::{
    Border, Generator, Layer, LayerOffset, Layout, Polygon, PolygonSides, Size, SolidColor,
};
use img_gen_spec::{IrregularPolygonSides, RegularPolygonSides};

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
        LayerOffset::default(),
        LayerOffset { x: 0, y: 100 },
        LayerOffset { x: 100, y: 0 },
        LayerOffset { x: 100, y: 100 },
    ];
    let sides = [
        Some(PolygonSides::default()),
        RegularPolygonSides::new(4).map(Into::into),
        RegularPolygonSides::new(5).map(Into::into),
        IrregularPolygonSides::new(vec![
            LayerOffset { x: 40, y: 20 },
            LayerOffset { x: 190, y: 40 },
            LayerOffset { x: 170, y: 180 },
            LayerOffset { x: 20, y: 150 },
        ])
        .map(Into::into),
    ];
    let rotation = [90.0, 0.0, 0.0, -45.0];

    for index in 0..sizes.len() {
        let (r, g, b) = (
            255 * [0usize, 1].contains(&index) as u8,
            255 * [0usize, 2].contains(&index) as u8,
            255 * [0usize, 3].contains(&index) as u8,
        );
        let layer = Layer {
            size: Some(sizes[index]),
            offset: offsets[index],
            polygon: Some(Polygon {
                color: SolidColor::new(g, b, r, 127).into(),
                border: if index != 0 {
                    Some(Border {
                        color: SolidColor::new(r, g, b, 63).into(),
                        width: NonZeroU32::new(20).unwrap(),
                    })
                } else {
                    None
                },
                sides: sides[index].clone().unwrap(),
                rotation: rotation[index],
            }),
            ..Default::default()
        };
        layout.layers.push(layer);
    }
    let generator = Generator { layout };
    let img = generator.render().await.unwrap();
    img.save("tests/out/test_polygon.png").unwrap();
}
