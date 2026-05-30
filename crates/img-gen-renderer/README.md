# `img-gen-renderer`

A crate to generate images from a deterministic data structure.

See [img-gen-spec] about how to build the deterministic data structure that
this library uses.

## Example

This library simply provides a `Generator` struct and an `Image` struct (which
basically wraps around the `image::RgbaImage` struct).

```rust
use img_gen_renderer::Generator;

#[tokio::main]
async fn main() {
    // see img-gen-spec for proper example of building a Layout
    let layout = img_gen_spec::Layout{
        debug: img_gen_spec::Debug {
            enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // add any external images paths (file or dir) here
    let external_resource_paths = vec![];
    let cache_root = None; // using default cache dir
    let generator = Generator::new(external_resource_paths, cache_root).unwrap();

    let img = generator.render(layout).await.unwrap();

    // now do whatever you want with the image data
    let img_hash = img.get_sha256().unwrap();
    img.save(format!("{img_hash}.png").as_str()).unwrap();
}
```

## Cache-enabled

This library's `Generator` struct employs a cache to reduce repeated HTTP requests,
when downloading fonts (and their metadata), while rendering an image.

The cache location can be explicitly specified (see `Generator` API docs).

[img-gen-spec]: https://crates.io/crates/img-gen-spec
