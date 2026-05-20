use img_gen::{Generator, Layout};

#[tokio::test]
async fn render_layout() {
    let default_layout_str = std::fs::read_to_string("tests/layouts/default.yml").unwrap();
    let layout: Layout = serde_saphyr::from_str(&default_layout_str).unwrap();
    let generator = Generator { layout };
    let image = generator.render().await.unwrap();
    image.save("tests/out/layout_default.png").unwrap();
}
