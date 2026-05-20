use std::num::NonZeroU32;

use img_gen::{Debug, Generator, Layer, LayerOffset, Layout, Size};

#[tokio::test]
async fn render() {
    let layout = Layout {
        size: Size {
            width: NonZeroU32::new(300),
            height: NonZeroU32::new(300),
        },
        layers: vec![Layer {
            size: Some(Size {
                width: NonZeroU32::new(250),
                height: NonZeroU32::new(250),
            }),
            offset: LayerOffset { x: 25, y: 25 },
            ..Default::default()
        }],
        debug: Some(Debug {
            enable: true,
            grid: true,
            grid_step: 25,
            ..Default::default()
        }),
    };
    let generator = Generator { layout };
    let img = generator.render().await.unwrap();
    img.save("tests/out/test_debug.png").unwrap();
}
