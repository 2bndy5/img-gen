use super::{Border, ColorKind, LayerOffset};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

/// A custom type to ensure regular polygons have at least 3 sides.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
pub struct RegularPolygonSides(u32);

impl RegularPolygonSides {
    /// Returns [`None`] when `sides < 3`.
    pub fn new(sides: u32) -> Option<Self> {
        if sides < 3 { None } else { Some(Self(sides)) }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

/// A custom type to ensure irregular polygons have at least 3 offsets.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
pub struct IrregularPolygonSides(Vec<LayerOffset>);

impl IrregularPolygonSides {
    /// Returns [`None`] when `offsets.len() < 3`.
    pub fn new(offsets: Vec<LayerOffset>) -> Option<Self> {
        let mut unique = Vec::new();
        for offset in offsets {
            if !unique.contains(&offset) {
                unique.push(offset);
            }
        }
        if unique.len() < 3 {
            None
        } else {
            Some(Self(unique))
        }
    }

    pub fn as_slice(&self) -> &[LayerOffset] {
        &self.0
    }
}

/// A custom type to ensure polygon data has a minimum of 3 points.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Debug, Clone)]
pub enum PolygonSides {
    Regular(RegularPolygonSides),
    Irregular(IrregularPolygonSides),
}

impl From<RegularPolygonSides> for PolygonSides {
    fn from(value: RegularPolygonSides) -> Self {
        Self::Regular(value)
    }
}

impl From<IrregularPolygonSides> for PolygonSides {
    fn from(value: IrregularPolygonSides) -> Self {
        Self::Irregular(value)
    }
}

impl Default for PolygonSides {
    fn default() -> Self {
        RegularPolygonSides(3).into()
    }
}

impl PolygonSides {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, SeqAccess, Visitor};

        struct PolygonSidesVisitor;

        impl<'de> Visitor<'de> for PolygonSidesVisitor {
            type Value = PolygonSides;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an integer >= 3 or a sequence of at least 3 unique offsets")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let sides = u32::try_from(value)
                    .map_err(|_| E::custom(format!("PolygonSides value overflow: {value}")))?;
                RegularPolygonSides::new(sides)
                    .map(Into::into)
                    .ok_or_else(|| {
                        E::custom(format!(
                            "Regular Polygons cannot have less than 3 sides, got {sides}"
                        ))
                    })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut offsets = Vec::new();
                while let Some(offset) = seq.next_element::<LayerOffset>()? {
                    offsets.push(offset);
                }
                IrregularPolygonSides::new(offsets)
                    .map(Into::into)
                    .ok_or_else(|| {
                        A::Error::custom("Irregular Polygons cannot have less than 3 unique points")
                    })
            }
        }

        deserializer.deserialize_any(PolygonSidesVisitor)
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

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[serde(default, deserialize_with = "PolygonSides::deserialize")]
    pub sides: PolygonSides,

    #[cfg(not(feature = "pyo3"))]
    #[serde(default, deserialize_with = "PolygonSides::deserialize")]
    pub sides: PolygonSides,

    /// The rotation applied to the rendered polygon.
    ///
    /// Does not affect `PolygonSides.Irregular`.
    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[serde(default)]
    pub rotation: f32,
    /// The rotation applied to the rendered polygon.
    ///
    /// Does not affect [`PolygonSides::Irregular`].
    #[cfg(not(feature = "pyo3"))]
    #[serde(default)]
    pub rotation: f32,
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::{IrregularPolygonSides, PolygonSides, RegularPolygonSides};
    use crate::LayerOffset;

    #[test]
    fn sides() {
        assert!(RegularPolygonSides::new(2).is_none());
        assert!(matches!(PolygonSides::default(), PolygonSides::Regular(v) if v.get() == 3));
        assert!(IrregularPolygonSides::new(vec![LayerOffset::default(); 2]).is_none());

        assert!(RegularPolygonSides::new(2).is_none());
        assert!(RegularPolygonSides::new(3).is_some());
        assert!(IrregularPolygonSides::new(vec![LayerOffset::default(); 2]).is_none());
        assert!(
            IrregularPolygonSides::new(vec![
                LayerOffset { x: 0, y: 0 },
                LayerOffset { x: 100, y: 0 },
                LayerOffset { x: 50, y: 100 },
            ])
            .is_some()
        );

        let regular: PolygonSides = RegularPolygonSides::new(3).unwrap().into();
        assert!(matches!(regular, PolygonSides::Regular(v) if v.get() == 3));

        let irregular: PolygonSides = IrregularPolygonSides::new(vec![
            LayerOffset { x: 0, y: 0 },
            LayerOffset { x: 100, y: 0 },
            LayerOffset { x: 50, y: 100 },
        ])
        .unwrap()
        .into();
        assert!(matches!(irregular, PolygonSides::Irregular(v) if v.as_slice().len() == 3));
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
        assert!(deserialized.contains("Regular Polygons cannot have less than 3 sides"));

        let yaml = 5_u32;
        let deserializer: U32Deserializer<serde_saphyr::Error> = yaml.into_deserializer();
        let deserialized = PolygonSides::deserialize(deserializer).unwrap();
        assert!(matches!(deserialized, PolygonSides::Regular(v) if v.get() == 5));
    }

    #[test]
    fn deserialize_irregular_sides() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        struct TestPolygonSides {
            #[serde(deserialize_with = "PolygonSides::deserialize")]
            sides: PolygonSides,
        }

        let expected = [
            LayerOffset { x: 0, y: 0 },
            LayerOffset { x: 100, y: 0 },
            LayerOffset { x: 50, y: 100 },
        ];

        let yaml = "sides:\n  - { x: 0, y: 0 }\n  - { x: 100, y: 0 }\n  - { x: 50, y: 100 }\n";
        let deserialized: TestPolygonSides = serde_saphyr::from_str(yaml).unwrap();

        if let PolygonSides::Irregular(points) = deserialized.sides {
            let points = points.as_slice();
            assert!(points.len() == expected.len());
            assert!(points[0].x == expected[0].x && points[0].y == expected[0].y);
            assert!(points[1].x == expected[1].x && points[1].y == expected[1].y);
            assert!(points[2].x == expected[2].x && points[2].y == expected[2].y);
        } else {
            panic!("Expected irregular polygon sides");
        }
    }

    #[test]
    fn deserialize_irregular_sides_invalid_len() {
        use serde::de::value::{Error, MapDeserializer, SeqDeserializer};

        let first = MapDeserializer::<_, Error>::new([("x", 0_i32), ("y", 0_i32)].into_iter());
        let second = MapDeserializer::<_, Error>::new([("x", 100_i32), ("y", 0_i32)].into_iter());
        let deserializer = SeqDeserializer::<_, Error>::new([first, second].into_iter());

        let error = PolygonSides::deserialize(deserializer)
            .unwrap_err()
            .to_string();
        assert!(error.contains("Irregular Polygons cannot have less than 3 unique points"));
    }

    #[test]
    fn deserialize_sides_invalid_str() {
        use serde::de::value::StrDeserializer;

        let value = StrDeserializer::<serde_saphyr::Error>::new("not-a-valid-polygon-sides");
        let error = PolygonSides::deserialize(value).unwrap_err().to_string();
        assert!(error.contains("an integer >= 3 or a sequence of at least 3 unique offsets"));
    }
}
