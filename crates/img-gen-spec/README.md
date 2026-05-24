# `img-gen-spec`

The specifications used to generate images.

This crate provides an API to create a deterministic structure for generating images.
It does not generate images on its own. Rather, this crate's API is used as a parameter
to the `Generator` API in `img-gen-renderer` crate.

## Examples

The easiest way to use this API is to deserialize a YAML or JSON string.

```rust
use img_gen_spec::Layout;
use serde::Deserialize;

let yaml_str = r#"
layout:
  layers:
    - background:
        color: indigo
      typography:
        content: Hello world
        align: center
        color: white
"#;
let layout: Layout = serde_saphyr::from_str(yaml_str).unwrap();
// now pass this `layout` to a supported image generator
```

You can of course use the API directly instead of deserialization.

```rust
use img_gen_spec::{Layout, Layer, Background, Typography, TypographyAlign};

let layout = Layout {
    layers: vec![
        Layer {
            background: Some(Background {
                color: SolidColor::from_str("indigo").unwrap().into(),
                ..Default::default()
            }),
            typography: Some(Typography {
                content: "Hello world".to_string(),
                align: TypographyAlign::Center,
                color: SolidColor::from_str("indigo").unwrap().into(),
                ..Default::default()
            }),
            ..Default::default()
        }
    ],
    ..Default::default()
};
// now pass this `layout` to a supported image generator
```

## History

Originally, this crate's API was inspired by the "Material for MkDocs" `social`
plugin. That social plugin was used to generate social media preview cards.

Then the social plugin was ported into a Sphinx extension for the same purpose
(generate social media cards). But along the way, many more features were added.

What this crate is today is the culmination of all this history distilled into
an API that details a specification for generating images (social media preview
cards or otherwise).
