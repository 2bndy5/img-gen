# img-gen

[![Rust][rust-ci-badge]][rust-ci-link]
[![Python][py-ci-badge]][py-ci-link]
[![codecov][codecov-badge]][codecov-link]

Generate images from a deterministic data structure.

This project is written in Rust but ships as a Python package also.

## Distributions

### Rust

| name | version | docs |
|------|---------|------|
| `img-gen` | [![crates.io][img-gen-crate-badge]][img-gen-crate-link] | [![docs.rs][img-gen-crate-docs-badge]][img-gen-crate-docs-link] |
| `img-gen-renderer` | [![crates.io][img-gen-renderer-crate-badge]][img-gen-renderer-crate-link] | [![docs.rs][img-gen-renderer-crate-docs-badge]][img-gen-renderer-crate-docs-link] |
| `img-gen-spec` | [![crates.io][img-gen-spec-crate-badge]][img-gen-spec-crate-link] | [![docs.rs][img-gen-spec-crate-docs-badge]][img-gen-spec-crate-docs-link] |

### Python

| name | version | docs |
|------|---------|------|
| `img-gen` | [![PyPI][pypi-badge]][pypi-link] | [![gh-pages][gh-pages-badge]][gh-pages-link] |

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
[img-gen-crate-badge]: https://img.shields.io/crates/v/img-gen
[img-gen-crate-link]: https://crates.io/crates/img-gen
[img-gen-crate-docs-badge]: https://img.shields.io/docsrs/img-gen
[img-gen-crate-docs-link]: https://docs.rs/img-gen/
[img-gen-renderer-crate-badge]: https://img.shields.io/crates/v/img-gen-renderer
[img-gen-renderer-crate-link]: https://crates.io/crates/img-gen-renderer
[img-gen-renderer-crate-docs-badge]: https://img.shields.io/docsrs/img-gen-renderer
[img-gen-renderer-crate-docs-link]: https://docs.rs/img-gen-renderer/
[img-gen-spec-crate-badge]: https://img.shields.io/crates/v/img-gen-spec
[img-gen-spec-crate-link]: https://crates.io/crates/img-gen-spec
[img-gen-spec-crate-docs-badge]: https://img.shields.io/docsrs/img-gen-spec
[img-gen-spec-crate-docs-link]: https://docs.rs/img-gen-spec/
[pypi-badge]: https://img.shields.io/pypi/v/img-gen
[pypi-link]: https://pypi.org/project/img-gen
[gh-pages-link]: https://2bndy5.github.io/img-gen
[gh-pages-badge]: https://img.shields.io/github/deployments/2bndy5/img-gen/github-pages?label=docs
