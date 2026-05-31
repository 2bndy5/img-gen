use std::fmt;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, MapAccess, Visitor},
};

use crate::{
    ColorGradient, ColorKind, ConicalGradient, LinearGradient, Presets, RadialGradient, SolidColor,
};

impl<'de> Deserialize<'de> for SolidColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        SolidColor::from_string(&s).map_err(serde::de::Error::custom)
    }
}
struct GradientVisitor;

impl<'de> Visitor<'de> for GradientVisitor {
    type Value = ColorGradient;

    fn visit_str<E>(self, gradient_str: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Some(preset) = Presets::from_string(gradient_str) {
            ColorGradient::new(None, Some(preset)).map_err(serde::de::Error::custom)
        } else {
            Err(E::invalid_value(
                serde::de::Unexpected::Str(gradient_str),
                &self,
            ))
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut gradient_spec: Vec<(f32, String)> = vec![];
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            let key = key.parse::<f32>().map_err(serde::de::Error::custom)?;
            gradient_spec.push((key, value));
        }
        let spec: Vec<(f32, &str)> = gradient_spec
            .iter()
            .map(|(k, v)| (*k, v.as_str()))
            .collect();
        ColorGradient::new(Some(spec), None).map_err(serde::de::Error::custom)
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a gradient preset or a gradient specification in the form of a HashMap<f32, String>.")
    }
}

impl<'de> Deserialize<'de> for ColorGradient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(GradientVisitor)
    }
}

impl<'de> Deserialize<'de> for ColorKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ColorKindVisitor)
    }
}

struct ColorKindVisitor;

impl<'de> Visitor<'de> for ColorKindVisitor {
    type Value = ColorKind;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a color string or a gradient map")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<ColorKind, E> {
        SolidColor::from_string(v)
            .map(ColorKind::SolidColor)
            .map_err(de::Error::custom)
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<ColorKind, E> {
        self.visit_str(&v)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<ColorKind, A::Error> {
        // Here, we'll use `serde_json::Value` as a temporary holder for map values.
        // The idea is to identify which ColorKind we're looking at based on the map keys.
        // Once we've identified the ColorKind, we can can deserialize the nested types using
        // `serde_json::from_value()`, after wrapping the map entries in `serde_json::Value::Object`.
        //
        // Despite using `serde_json` API, this keeps deserialization agnostic of the original format.
        // This will work from YAML or JSON as long as the incoming data structure is as expected.
        let mut entries: Vec<(String, serde_json::Value)> = Vec::new();
        while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
            entries.push((key, value));
        }

        let has_start = entries.iter().any(|(k, _)| k == "start");
        let has_radius = entries.iter().any(|(k, _)| k == "radius");
        let has_center = entries.iter().any(|(k, _)| k == "center");
        let has_angle = entries.iter().any(|(k, _)| k == "angle");

        let map_value = serde_json::Value::Object(entries.into_iter().collect());

        if has_start {
            return serde_json::from_value::<LinearGradient>(map_value)
                .map(ColorKind::LinearGradient)
                .map_err(de::Error::custom);
        }
        if has_radius {
            return serde_json::from_value::<RadialGradient>(map_value)
                .map(ColorKind::RadialGradient)
                .map_err(de::Error::custom);
        }
        if has_center || has_angle {
            return serde_json::from_value::<ConicalGradient>(map_value)
                .map(ColorKind::ConicalGradient)
                .map_err(de::Error::custom);
        }

        Err(de::Error::custom(
            "unknown color kind: map must have 'start' (linear), 'radius' (radial), or 'center'/'angle' (conical)",
        ))
    }
}

impl Serialize for ColorGradient {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(self.stops.len()))?;
        for (point, color) in &self.stops {
            map.serialize_entry(&point.to_string(), color)?;
        }
        map.end()
    }
}

impl Serialize for ColorKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ColorKind::LinearGradient(g) => g.serialize(serializer),
            ColorKind::RadialGradient(g) => g.serialize(serializer),
            ColorKind::ConicalGradient(g) => g.serialize(serializer),
            ColorKind::SolidColor(c) => serializer.serialize_str(&c.to_rgba_css_string()),
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use crate::{ColorGradient, ColorKind, ConicalGradient, LinearGradient, RadialGradient};

    #[test]
    fn deserialize_solid_color() {
        let color_str = "rgba(255, 0, 0, 1.0)";
        let color_kind: ColorKind = serde_saphyr::from_str(color_str).unwrap();
        assert_eq!(
            serde_json::to_value(&color_kind).unwrap(),
            serde_json::json!("rgba(255, 0, 0, 1)"),
        );
    }

    #[test]
    fn deserialize_linear_gradient() {
        let blue_tuple = (0, 0, 255, 255);
        let red_tuple = (255, 0, 0, 255);
        let gradient_str = r#"
start:
  x: 0
  y: 0
end:
  x: 100
  y: 100
colors:
  0.0: red
  1.0: blue
"#;
        let color_kind: ColorKind = serde_saphyr::from_str(gradient_str).unwrap();
        let value = serde_json::to_value(&color_kind).unwrap();
        assert_eq!(value["spread"], serde_json::json!("pad"));
        assert_eq!(value["start"], serde_json::json!({"x": 0, "y": 0}));
        assert_eq!(value["end"], serde_json::json!({"x": 100, "y": 100}));
        let linear_gradient = serde_json::from_value::<LinearGradient>(value).unwrap();
        assert_eq!(linear_gradient.get_color_at(0, 0).to_tuple(), red_tuple);
        assert_eq!(
            linear_gradient.get_color_at(100, 100).to_tuple(),
            blue_tuple
        );
    }

    #[test]
    fn deserialize_radial_gradient() {
        let blue_tuple = (0, 0, 255, 255);
        let red_tuple = (255, 0, 0, 255);
        let gradient_str = r#"
radius: 100.0
center:
    x: 50
    y: 50
focal_point:
    x: 50
    y: 50
focal_radius: 0.0
colors:
    0.0: red
    0.1: red
    1.0: blue
"#;
        let color_kind: ColorKind = serde_saphyr::from_str(gradient_str).unwrap();
        let value = serde_json::to_value(&color_kind).unwrap();
        assert_eq!(value["spread"], serde_json::json!("pad"));
        assert_eq!(value["radius"], serde_json::json!(100.0));
        assert_eq!(value["center"], serde_json::json!({"x": 50, "y": 50}));
        let radial = serde_json::from_value::<RadialGradient>(value).unwrap();
        assert_eq!(radial.get_color_at(50, 50).to_tuple(), red_tuple);
        assert_eq!(radial.get_color_at(150, 150).to_tuple(), blue_tuple);
    }

    #[test]
    fn deserialize_conical_gradient() {
        let blue_tuple = (0, 0, 255, 255);
        let red_tuple = (255, 0, 0, 255);
        let gradient_str = r#"
center:
    x: 1
    y: 1
angle: 0.0
colors:
    0.0: red
    0.1: red
    1.0: blue
"#;
        let color_kind: ColorKind = serde_saphyr::from_str(gradient_str).unwrap();
        let value = serde_json::to_value(&color_kind).unwrap();
        assert_eq!(value["center"], serde_json::json!({"x": 1, "y": 1}));
        assert_eq!(value["angle"], serde_json::json!(0.0));
        let conical = serde_json::from_value::<ConicalGradient>(value).unwrap();
        assert_eq!(conical.get_color_at(2, 1).to_tuple(), blue_tuple);
        assert_eq!(conical.get_color_at(50, 0).to_tuple(), red_tuple);
    }

    #[test]
    fn fail_deserialize_color_kind() {
        let invalid_str = r"unsupported-key: value";
        let err = serde_saphyr::from_str::<ColorKind>(invalid_str).unwrap_err();
        assert!(err.to_string().contains("unknown color kind"));

        let invalid_str = r"42";
        let err = serde_saphyr::from_str::<ColorKind>(invalid_str).unwrap_err();
        assert!(err.to_string().contains("a color string or a gradient map"));
    }

    #[test]
    fn serialize_color_gradient_preset_as_map() {
        let gradient: ColorGradient = serde_saphyr::from_str("MonoChrome").unwrap();
        let value = serde_json::to_value(&gradient).unwrap();
        assert_eq!(
            value.get("0").or_else(|| value.get("0.0")),
            Some(&serde_json::Value::String("rgba(0, 0, 0, 1)".to_string()))
        );
        assert_eq!(
            value.get("1").or_else(|| value.get("1.0")),
            Some(&serde_json::Value::String(
                "rgba(255, 255, 255, 1)".to_string()
            ))
        );
    }

    #[test]
    fn serialize_color_gradient_stops_as_map() {
        let gradient: ColorGradient = serde_saphyr::from_str("{0.0: red, 1.0: blue}").unwrap();
        let value = serde_json::to_value(&gradient).unwrap();
        assert_eq!(
            value.get("0").or_else(|| value.get("0.0")),
            Some(&serde_json::Value::String("rgba(255, 0, 0, 1)".to_string()))
        );
        assert_eq!(
            value.get("1").or_else(|| value.get("1.0")),
            Some(&serde_json::Value::String("rgba(0, 0, 255, 1)".to_string()))
        );
    }
}
