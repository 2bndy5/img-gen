use std::num::{NonZeroI32, NonZeroU32};

mod support;

use img_gen::{
    Background, Border, ColorGradient, Debug, Font, Generator, Layer, LayerOffset, Layout, Line,
    LineHeight, LinearGradient, PreserveAspect, Presets, Size, SolidColor, Spread, Typography,
    TypographyAlign, Weight,
};

/// White-background layout with a single constrained typography layer.
fn basic_layout(
    canvas_w: u32,
    canvas_h: u32,
    layer_w: u32,
    layer_h: u32,
    typography: Typography,
) -> Layout {
    let layer_offset = LayerOffset {
        x: (canvas_w as i32 - layer_w as i32) / 2,
        y: (canvas_h as i32 - layer_h as i32) / 2,
    };
    let bg_layer = Layer {
        background: Some(Background {
            color: Some(SolidColor::default().into()),
            image: None,
            preserve_aspect: PreserveAspect::Off,
        }),
        ..Default::default()
    };
    let layer = Layer {
        size: Some(Size {
            width: NonZeroU32::new(layer_w),
            height: NonZeroU32::new(layer_h),
        }),
        offset: layer_offset,
        typography: Some(typography),
        ..Default::default()
    };
    Layout {
        size: Size {
            width: NonZeroU32::new(canvas_w),
            height: NonZeroU32::new(canvas_h),
        },
        layers: vec![bg_layer, layer],
        debug: Some(Debug {
            enable: true,
            grid: false,
            ..Default::default()
        }),
    }
}

/// `overflow = true`: font is shrunk until all text fits within the layer
#[tokio::test]
async fn render_shrink_to_fit() {
    let layer_w = 200u32;
    let layer_h = 60u32;
    let canvas_w = 400u32;
    let canvas_h = 200u32;

    let typography = Typography {
        content: "All of this text must fit inside the small layer without being cut off."
            .to_string(),
        line: Line {
            amount: NonZeroI32::new(3).unwrap(),
            height: LineHeight::new(1.0).unwrap(),
        },
        overflow: true, // shrink font to fit
        color: SolidColor::from_string("white").unwrap().into(),
        ..Default::default()
    };

    let layout = basic_layout(canvas_w, canvas_h, layer_w, layer_h, typography);
    let generator = Generator::new(vec![], Some(support::typography_font_cache_root())).unwrap();
    let img = generator.render(layout).await.unwrap();

    img.save("tests/out/test_typography_shrink_to_fit.png")
        .unwrap();
}

/// `overflow = false`: text wraps within the layer width; content that still
/// doesn't fit vertically is replaced with a trailing ellipsis so that the
/// rendered output stays within the layer height.
#[tokio::test]
async fn render_wrap_with_ellipsis() {
    let layer_w: u32 = 550;
    let layer_h: u32 = 350;
    let gradient_start = LayerOffset {
        x: 0,
        y: layer_h as i32 / 2,
    };
    let gradient_end = LayerOffset {
        x: layer_w as i32 / 2,
        y: layer_h as i32 / 2,
    };

    let typography = Typography {
        content:
            "This sentence is intentionally very long so that it definitely overflows the small \
         layer height and must be truncated with an ellipsis at the end."
                .to_string(),
        line: Line {
            amount: NonZeroI32::new(5).unwrap(),
            height: LineHeight::new(1.2).unwrap(),
        },
        overflow: false, // wrap + ellipsis
        font: Font {
            family: "Playfair Display".to_string(),
            weight: Weight::Regular,
            style: "normal".to_string(),
            subset: "latin".to_string(),
            path: None,
        },
        border: Some(Border {
            width: NonZeroU32::new(2).unwrap(),
            color: LinearGradient::new(
                ColorGradient::new(None, Some(Presets::FabledSunset)).unwrap(),
                gradient_start,
                gradient_end,
                Some(Spread::Reflect),
            )
            .into(),
        }),
        color: LinearGradient::new(
            ColorGradient::new(None, Some(Presets::MillenniumPine)).unwrap(),
            gradient_start,
            gradient_end,
            Some(Spread::Reflect),
        )
        .into(),
        ..Default::default()
    };

    let layout = basic_layout(600, 400, layer_w, layer_h, typography);
    let generator = Generator::new(vec![], Some(support::typography_font_cache_root())).unwrap();
    let img = generator.render(layout).await.unwrap();

    img.save("tests/out/test_typography_wrap_ellipsis.png")
        .unwrap();
}

#[tokio::test]
async fn render_center() {
    let layer_w = 200u32;
    let layer_h = 100u32;
    let canvas_w = 400u32;
    let canvas_h = 200u32;
    let typography = Typography {
        content: "Center\nthis\ntext.".to_string(),
        align: TypographyAlign::CenterCenter,
        line: Line {
            amount: NonZeroI32::new(3).unwrap(),
            height: LineHeight::new(1.0).unwrap(),
        },
        overflow: true, // shrink font to fit
        color: SolidColor::from_string("white").unwrap().into(),
        ..Default::default()
    };

    let layout = basic_layout(canvas_w, canvas_h, layer_w, layer_h, typography);
    let generator = Generator::new(vec![], Some(support::typography_font_cache_root())).unwrap();
    let img = generator.render(layout).await.unwrap();

    img.save("tests/out/test_typography_center.png").unwrap();
}

#[tokio::test]
async fn render_end_bottom() {
    let layer_w = 200u32;
    let layer_h = 100u32;
    let canvas_w = 400u32;
    let canvas_h = 200u32;

    let typography = Typography {
        content: "This\nstarts at the `EndBottom`.".to_string(),
        align: TypographyAlign::EndBottom,
        line: Line {
            amount: NonZeroI32::new(3).unwrap(),
            height: LineHeight::new(1.0).unwrap(),
        },
        overflow: true, // shrink font to fit
        color: SolidColor::from_string("white").unwrap().into(),
        ..Default::default()
    };

    let layout = basic_layout(canvas_w, canvas_h, layer_w, layer_h, typography);
    let generator = Generator::new(vec![], Some(support::typography_font_cache_root())).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/test_typography_end_bottom.png")
        .unwrap();
}
