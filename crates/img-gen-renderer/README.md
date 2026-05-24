# `img-gen-renderer`

A crate to generate images from a deterministic data structure.

See [img-gen-specs] about how to build the deterministic data structure that
this library uses.

## Example

This library simply provides a `Generator` struct and an `Image` struct (which
basically wraps around the `image::RgbaImage` struct).

```rust
use img_gen_renderer::Generator;

#[tokio::main]
async fn main() {
    // see img-gen-specs for proper example of building a Layout
    let layout = img_gen_specs::Layout{
        debug: img_gen_specs::Debug {
            enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let generator = Generator { layout };
    let img = generator.render().await.unwrap();

    // now do whatever you want with the image data
    let img_hash = img.get_sha256().unwrap();
    img.save(format!("{img_hash}.png").as_str()).unwrap();
}
```

## Cache-enabled

This library's `Generator` struct employ a cache to reduce repeated HTTP requests,
like downloading fonts or external images, while rendering an image.

The cache location can be explicitly specified (see `Generator` API docs).

[img-gen-specs]: https://crates.io/crates/img-gen-specs
