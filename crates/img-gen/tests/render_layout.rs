use img_gen::{Generator, Layout};
use serde_json::Value;

mod support;

fn read_layout_fixture(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn read_layout_fixture_value(path: &str) -> Value {
    let fixture = read_layout_fixture(path);
    serde_saphyr::from_str::<Value>(&fixture).unwrap()
}

fn serialized_layout_value(layout: &Layout) -> Value {
    let serialized = serde_saphyr::to_string(layout).unwrap();
    serde_saphyr::from_str::<Value>(&serialized).unwrap()
}

#[tokio::test]
async fn render_layout() {
    let default_layout_str = std::fs::read_to_string("tests/layouts/default.yml").unwrap();
    let layout: Layout = serde_saphyr::from_str(&default_layout_str).unwrap();
    let generator = Generator::new(vec![], Some(support::typography_font_cache_root())).unwrap();
    let img = generator.render(layout).await.unwrap();
    img.save("tests/out/layout_default.png").unwrap();
}

#[test]
fn serialize_custom_serde() {
    let cases = [
        (
            "polygon regular sides serialize as integer",
            "tests/layouts/serde_polygon_sides_input.yml",
            "/layers/0/polygon",
            "tests/layouts/serde_polygon_sides_expected.yml",
            "/regular",
        ),
        (
            "polygon irregular sides serialize as deduplicated sequence",
            "tests/layouts/serde_polygon_sides_input.yml",
            "/layers/1/polygon",
            "tests/layouts/serde_polygon_sides_expected.yml",
            "/irregular",
        ),
        (
            "linear gradient preset serializes to color stop map",
            "tests/layouts/serde_color_kind_input.yml",
            "/layers/0/background/color",
            "tests/layouts/serde_color_kind_expected.yml",
            "/linear",
        ),
        (
            "radial gradient color kind serializes to radial gradient object",
            "tests/layouts/serde_color_kind_input.yml",
            "/layers/1/background/color",
            "tests/layouts/serde_color_kind_expected.yml",
            "/radial",
        ),
        (
            "conical gradient color kind serializes to conical gradient object",
            "tests/layouts/serde_color_kind_input.yml",
            "/layers/2/background/color",
            "tests/layouts/serde_color_kind_expected.yml",
            "/conical",
        ),
        (
            "legacy font style deserializes then serializes canonical font fields",
            "tests/layouts/serde_font_legacy_input.yml",
            "/layers/0/typography/font",
            "tests/layouts/serde_font_legacy_expected.yml",
            "",
        ),
        (
            "debug bool deserializes then serializes explicit debug object",
            "tests/layouts/serde_debug_bool_input.yml",
            "/debug",
            "tests/layouts/serde_debug_bool_expected.yml",
            "",
        ),
    ];

    for (name, input_path, actual_pointer, expected_path, expected_pointer) in cases {
        let input = read_layout_fixture(input_path);
        let layout: Layout = serde_saphyr::from_str(&input).unwrap();
        let actual = serialized_layout_value(&layout);
        let actual = actual
            .pointer(actual_pointer)
            .unwrap_or_else(|| panic!("{name}: missing serialized pointer {actual_pointer}"));
        let expected_value = read_layout_fixture_value(expected_path);
        let expected = if expected_pointer.is_empty() {
            &expected_value
        } else {
            expected_value.pointer(expected_pointer).unwrap_or_else(|| {
                panic!("{name}: missing expected pointer {expected_pointer} in {expected_path}")
            })
        };

        assert_eq!(
            actual, expected,
            "{name}: serialized output at {actual_pointer} did not match expected fixture"
        );
    }
}
