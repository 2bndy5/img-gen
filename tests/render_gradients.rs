use std::num::NonZeroU32;

use img_gen::{
    Background, ColorGradient, ConicalGradient, Generator, Layer, LayerOffset, Layout,
    LinearGradient, RadialGradient, Size, Spread,
};

const GRADIENT_SPEC: [(f32, &str); 4] =
    [(0.0, "green"), (0.1, "red"), (0.5, "green"), (1.0, "blue")];

async fn linear_gradient(spread: Spread) {
    let color_spec = ColorGradient::new(Some(GRADIENT_SPEC.to_vec()), None).unwrap();
    let mut layout = Layout {
        size: Size {
            width: NonZeroU32::new(500),
            height: NonZeroU32::new(500),
        },
        layers: vec![],
        debug: None,
    };
    let gradient_domains = vec![
        // (start_offset, end_offset)
        (LayerOffset { x: 50, y: 50 }, LayerOffset { x: 200, y: 200 }),
        (LayerOffset { x: 200, y: 200 }, LayerOffset { x: 50, y: 50 }),
        (
            LayerOffset { x: 125, y: 50 },
            LayerOffset { x: 125, y: 200 },
        ),
        (
            LayerOffset { x: 50, y: 125 },
            LayerOffset { x: 200, y: 125 },
        ),
    ];
    for (index, (start_offset, end_offset)) in gradient_domains.into_iter().enumerate() {
        let gradient =
            LinearGradient::new(color_spec.clone(), start_offset, end_offset, Some(spread));
        let layer = Layer {
            size: Some(Size {
                width: NonZeroU32::new(250),
                height: NonZeroU32::new(250),
            }),
            offset: LayerOffset {
                x: 250 * (index as i32 % 2),
                y: 250 * (index as i32 / 2),
            },
            background: Some(Background {
                color: Some(gradient.into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        layout.layers.push(layer);
    }
    let generator = Generator { layout };
    let img = generator.render().await.unwrap();
    img.save(format!("tests/out/linear_gradient[{}].png", spread).as_str())
        .unwrap();
}

#[tokio::test]
async fn linear_gradient_pad() {
    linear_gradient(Spread::Pad).await;
}

#[tokio::test]
async fn linear_gradient_reflect() {
    linear_gradient(Spread::Reflect).await;
}

#[tokio::test]
async fn linear_gradient_repeat() {
    linear_gradient(Spread::Repeat).await;
}

async fn radial_gradient(spread: Spread) {
    let color_spec = ColorGradient::new(Some(GRADIENT_SPEC.to_vec()), None).unwrap();
    let gradient_domains = vec![
        // (focal_point, focal_radius)
        (None, None),
        (Some(LayerOffset { x: 75, y: 75 }), Some(0.0)),
        (Some(LayerOffset { x: 75, y: 75 }), Some(37.5)),
        (Some(LayerOffset { x: 75, y: 75 }), Some(100.0)),
    ];
    let mut layout = Layout {
        size: Size {
            width: NonZeroU32::new(500),
            height: NonZeroU32::new(500),
        },
        layers: vec![],
        debug: None,
    };
    for (index, (focal_point, focal_radius)) in gradient_domains.into_iter().enumerate() {
        let gradient = RadialGradient::new(
            color_spec.clone(),
            LayerOffset { x: 125, y: 125 },
            125.0,
            focal_point,
            focal_radius,
            Some(spread),
        );
        let layer = Layer {
            size: Some(Size {
                width: NonZeroU32::new(250),
                height: NonZeroU32::new(250),
            }),
            offset: LayerOffset {
                x: 250 * (index as i32 % 2),
                y: 250 * (index as i32 / 2),
            },
            background: Some(Background {
                color: Some(gradient.into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        layout.layers.push(layer);
    }

    let generator = Generator { layout };
    let img = generator.render().await.unwrap();
    img.save(format!("tests/out/radial_gradient[{}].png", spread).as_str())
        .unwrap();
}

#[tokio::test]
async fn radial_gradient_pad() {
    radial_gradient(Spread::Pad).await;
}

#[tokio::test]
async fn radial_gradient_reflect() {
    radial_gradient(Spread::Reflect).await;
}

#[tokio::test]
async fn radial_gradient_repeat() {
    radial_gradient(Spread::Repeat).await;
}

#[tokio::test]
async fn conical_gradient() {
    let color_spec = ColorGradient::new(Some(GRADIENT_SPEC.to_vec()), None).unwrap();
    let gradient_domains = vec![
        // (center, angle)
        (LayerOffset { x: 50, y: 125 }, None),
        (LayerOffset { x: 125, y: 125 }, Some(-45.0)),
        (LayerOffset { x: 200, y: 125 }, Some(-180.0)),
        (LayerOffset { x: 125, y: 125 }, Some(90.0)),
    ];
    let mut layout = Layout {
        size: Size {
            width: NonZeroU32::new(500),
            height: NonZeroU32::new(500),
        },
        layers: vec![],
        debug: None,
    };
    for (index, (center, angle)) in gradient_domains.into_iter().enumerate() {
        let gradient = ConicalGradient::new(color_spec.clone(), center, angle);
        let layer = Layer {
            size: Some(Size {
                width: NonZeroU32::new(250),
                height: NonZeroU32::new(250),
            }),
            offset: LayerOffset {
                x: 250 * (index as i32 % 2),
                y: 250 * (index as i32 / 2),
            },
            background: Some(Background {
                color: Some(gradient.into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        layout.layers.push(layer);
    }
    let generator = Generator { layout };
    let img = generator.render().await.unwrap();
    img.save("tests/out/conical_gradient.png").unwrap();
}
