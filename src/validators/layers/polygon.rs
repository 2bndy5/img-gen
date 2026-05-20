use super::{Border, ColorKind};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

/// A custom type to ensure the minimum number of polygon sides is 3.
#[derive(Debug, Clone, Copy)]
pub struct PolygonSides(u32);

impl Default for PolygonSides {
    fn default() -> Self {
        Self(3)
    }
}

impl PolygonSides {
    /// Instantiate a [`PolygonSides`] object.
    ///
    /// Returns [`None`] if the given number of sides is less than 3.
    pub fn new(sides: u32) -> Option<Self> {
        if sides < 3 { None } else { Some(Self(sides)) }
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;

        let sides = u32::deserialize(deserializer)?;
        Self::new(sides).ok_or_else(|| {
            serde::de::Error::custom(format!("PolygonSides cannot be less than 3, got {sides}"))
        })
    }
}

/// An attribute to represent a [`Polygon`] rendered in the layer.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Polygon {
    /// The border (if specified) ro render around the polygon.
    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub border: Option<Border>,
    /// The [`Border`] (if specified) ro render around the polygon.
    #[cfg(not(feature = "pyo3"))]
    pub border: Option<Border>,

    /// The color used to fill the polygon.
    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub color: ColorKind,

    /// The color used to fill the polygon.
    #[cfg(not(feature = "pyo3"))]
    pub color: ColorKind,

    #[serde(default, deserialize_with = "PolygonSides::deserialize")]
    pub sides: PolygonSides,

    /// The rotation applied to the rendered polygon.
    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[serde(default)]
    pub rotation: f32,
    /// The rotation applied to the rendered polygon.
    #[cfg(not(feature = "pyo3"))]
    #[serde(default)]
    pub rotation: f32,
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use super::PolygonSides;

    #[test]
    fn sides() {
        assert!(PolygonSides::new(2).is_none());
        assert!(PolygonSides::default().0 == 3);
    }

    #[test]
    fn deserialize_sides() {
        use serde::de::{IntoDeserializer, value::U32Deserializer};

        let yaml = 2_u32;
        let deserializer: U32Deserializer<serde_saphyr::Error> = yaml.into_deserializer();
        let deserialized = PolygonSides::deserialize(deserializer)
            .unwrap_err()
            .to_string();
        eprintln!("Deserialization error: {deserialized}");
        assert!(deserialized.contains("PolygonSides cannot be less than 3, got 2"));

        let yaml = 5_u32;
        let deserializer: U32Deserializer<serde_saphyr::Error> = yaml.into_deserializer();
        let deserialized = PolygonSides::deserialize(deserializer).unwrap();
        assert!(deserialized.get() == 5);
    }
}
