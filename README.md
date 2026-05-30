# img-gen

[![Rust][rust-ci-badge]][rust-ci-link]
[![Python][py-ci-badge]][py-ci-link]
[![codecov][codecov-badge]][codecov-link]

Generate images from a deterministic data structure.

This project is written in Rust but ships as a Python package also.

## Examples

The below examples use the following YAML doc as a file named `example-layout.yml`.

```yaml
layers:
  - background:
      color: "#4051b5"
  - icon:
      image: material/cat
      color: white
    offset: { x: 100, y: 100 }
    size: { width: 100, height: 100 }
```

### Python

```python
import asyncio # the only supported async runtime in python
from pathlib import Path

# configure loggers before importing this lib
from img_gen import Layout, Generator

async def main():
    generator = Generator(
        # add any external image/font paths (file or dir) here
        external_resource_paths=[],
        cache_root=None,  # use default cache dir
    )

    layout = Layout.from_yaml_str(yaml_str)
    yaml_str = Path("example-layout.yml").read_text(encoding="utf-8")

    # Generator.render() is async
    img = await generator.render(layout)

    img_hash = img.sha256
    img.save(f"{img_hash}.png")
```

### Rust

```rust
use img_gen::{Generator, Layout};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let external_resource_paths = vec![]; // not using external images/fonts
    let cache_root = None; // use default value
    let generator = Generator::new(external_resource_paths, cache_root).unwrap();

    // using safer/newer yaml deserializing crate `serde_saphyr`
    let yaml_str = std::fs::read("example-layout.yml").unwrap();
    let layout: Layout = serde_saphyr::from_str(&yaml_str).unwrap();

    let img = generator.render(layout).await.unwrap();

    let img_hash = img.get_sha256().unwrap();
    img.save(format!("{img_hash}.png").as_str()).unwrap();
}
```

[codecov-badge]: https://codecov.io/gh/2bndy5/img-gen/graph/badge.svg?token=QRODAHAOXL
[codecov-link]: https://codecov.io/gh/2bndy5/img-gen
[py-ci-badge]: https://github.com/2bndy5/img-gen/actions/workflows/python.yml/badge.svg
[py-ci-link]: https://github.com/2bndy5/img-gen/actions/workflows/python.yml
[rust-ci-badge]: https://github.com/2bndy5/img-gen/actions/workflows/rust.yml/badge.svg
[rust-ci-link]: https://github.com/2bndy5/img-gen/actions/workflows/rust.yml
