use img_gen::{Generator, Layout};

mod support;

#[tokio::test]
async fn render_layout() {
    let default_layout_str = std::fs::read_to_string("tests/layouts/default.yml").unwrap();
    let layout: Layout = serde_saphyr::from_str(&default_layout_str).unwrap();
    let generator = Generator::new(vec![], Some(support::typography_font_cache_root())).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/layout_default.png").unwrap();
}
