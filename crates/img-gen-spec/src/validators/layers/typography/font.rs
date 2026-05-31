use std::path::PathBuf;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use fontsource_downloader::Weight as FsWeight;
use parley::{FontFamily, FontStyle, FontWeight};
use serde::{Deserialize, Deserializer, Serialize};

/// An enumeration of the possible font weights.
#[cfg_attr(
    feature = "pyo3",
    pyclass(eq, eq_int, module = "img_gen", from_py_object)
)]
#[derive(Debug, PartialEq, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Weight {
    #[serde(alias = "100")]
    /// Maps to CSS font weight `100`.
    Thin = 100,
    #[serde(alias = "300")]
    /// Maps to CSS font weight `300`.
    Light = 300,
    #[serde(alias = "400")]
    /// Maps to CSS font weight `400`.
    #[default]
    Regular = 400,
    #[serde(alias = "500")]
    /// Maps to CSS font weight `500`.
    Medium = 500,
    #[serde(alias = "700")]
    /// Maps to CSS font weight `700`.
    Bold = 700,
    #[serde(alias = "900")]
    /// Maps to CSS font weight `900`.
    Black = 900,
}

impl Weight {
    pub(crate) const fn value(self) -> u16 {
        self as u16
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value.trim().to_lowercase().as_str() {
            "thin" | "100" => Some(Weight::Thin),
            "light" | "300" => Some(Weight::Light),
            "normal" | "regular" | "400" => Some(Weight::Regular),
            "medium" | "500" => Some(Weight::Medium),
            "bold" | "700" => Some(Weight::Bold),
            "black" | "900" => Some(Weight::Black),
            _ => None,
        }
    }
}

impl From<&str> for Weight {
    fn from(value: &str) -> Self {
        Self::parse(value).unwrap_or(Weight::Regular)
    }
}

/// A property to implicitly describe the font used in a
/// [`Typography`](struct@super::Typography) attribute.
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone)]
pub struct Font {
    /// The font family's name.
    pub family: String,
    /// The font family's style.
    pub style: String,
    /// The font's `Weight`.
    pub weight: Weight,
    /// The font family's lingual subset.
    ///
    /// The valid options for this can vary depending on the chosen font `family`.
    pub subset: String,
    /// An optional path to the font's ``.ttf`` file.
    pub path: Option<String>,
}

impl Default for Font {
    fn default() -> Self {
        Font {
            family: Self::default_font_family(),
            style: Self::default_font_style(),
            weight: Weight::default(),
            subset: Self::default_font_subset(),
            path: None,
        }
    }
}

impl Font {
    pub(crate) fn from_parts(
        family: String,
        style: Option<String>,
        weight: Option<Weight>,
        subset: Option<String>,
        path: Option<String>,
    ) -> Self {
        let raw_style = style.unwrap_or_else(Self::default_font_style);
        let (weight, style) = match weight {
            Some(weight) => (weight, Self::drop_legacy_weight_prefix(&raw_style)),
            None => match Self::parse_legacy_style(&raw_style) {
                Some((legacy_weight, legacy_style)) => (legacy_weight, legacy_style),
                None => (Weight::default(), raw_style),
            },
        };

        Self {
            family,
            style,
            weight,
            subset: subset.unwrap_or_else(Self::default_font_subset),
            path,
        }
    }

    /// Creates a font from a family name and an optional style string.
    ///
    /// Legacy weight prefixes in `style` are normalized into [`Font::weight`] when present.
    pub fn from_family_style(family: String, style: Option<String>) -> Self {
        Self::from_parts(family, style, None, None, None)
    }

    pub(crate) fn parse_legacy_style(style: &str) -> Option<(Weight, String)> {
        let mut parts = style.split_whitespace();
        let first = parts.next()?;
        let parsed_weight = Weight::parse(first)?;

        let remainder = parts.collect::<Vec<_>>().join(" ");
        let parsed_style = if remainder.is_empty() {
            Self::default_font_style()
        } else {
            remainder.to_lowercase()
        };

        Some((parsed_weight, parsed_style))
    }

    pub(crate) fn drop_legacy_weight_prefix(style: &str) -> String {
        let mut parts = style.split_whitespace();
        let Some(first) = parts.next() else {
            return Self::default_font_style();
        };

        if Weight::parse(first).is_none() {
            return style.to_string();
        }

        let remainder = parts.collect::<Vec<_>>().join(" ");
        if remainder.is_empty() {
            Self::default_font_style()
        } else {
            remainder.to_lowercase()
        }
    }

    /// Returns the font family in the format expected by text layout code.
    pub fn font_family(&self) -> FontFamily<'_> {
        FontFamily::named(&self.family)
    }

    /// Returns the parsed font style.
    pub fn font_style(&self) -> FontStyle {
        FontStyle::parse_css(self.style.trim()).unwrap_or(FontStyle::Normal)
    }

    /// Returns the numeric font weight.
    pub fn font_weight(&self) -> FontWeight {
        FontWeight::new(self.weight.value() as f32)
    }

    /// Returns the configured font path as a [`PathBuf`], when present.
    pub fn path_buf(&self) -> Option<PathBuf> {
        self.path.as_ref().map(PathBuf::from)
    }

    pub(crate) fn default_font_family() -> String {
        "Roboto".to_string()
    }

    pub(crate) fn default_font_style() -> String {
        "normal".to_string()
    }

    pub(crate) fn default_font_subset() -> String {
        "latin".to_string()
    }
}

/// Intermediate deserialization shape used to support both the current
/// explicit `weight` field and the legacy style format (`"{weight} {style}"`).
#[derive(Serialize, Deserialize)]
struct FontSerde {
    #[serde(default = "Font::default_font_family")]
    family: String,
    #[serde(default = "Font::default_font_style")]
    style: String,
    weight: Option<Weight>,
    #[serde(default = "Font::default_font_subset")]
    subset: String,
    path: Option<String>,
}

impl<'de> Deserialize<'de> for Font {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parsed = FontSerde::deserialize(deserializer)?;
        Ok(Font::from_parts(
            parsed.family,
            Some(parsed.style),
            parsed.weight,
            Some(parsed.subset),
            parsed.path,
        ))
    }
}

impl Serialize for Font {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        FontSerde {
            family: self.family.clone(),
            style: self.style.clone(),
            weight: Some(self.weight),
            subset: self.subset.clone(),
            path: self.path.clone(),
        }
        .serialize(serializer)
    }
}

impl From<&Weight> for FsWeight {
    fn from(value: &Weight) -> Self {
        Self::from(*value)
    }
}

impl From<Weight> for FsWeight {
    fn from(value: Weight) -> Self {
        match value {
            Weight::Thin => FsWeight::Thin,
            Weight::Light => FsWeight::Light,
            Weight::Regular => FsWeight::Normal,
            Weight::Medium => FsWeight::Medium,
            Weight::Bold => FsWeight::Bold,
            Weight::Black => FsWeight::Black,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn convert_weight() {
        assert_eq!(FsWeight::from(Weight::Thin), FsWeight::Thin);
        assert_eq!(FsWeight::from(Weight::Light), FsWeight::Light);
        assert_eq!(FsWeight::from(Weight::Regular), FsWeight::Normal);
        assert_eq!(FsWeight::from(Weight::Medium), FsWeight::Medium);
        assert_eq!(FsWeight::from(Weight::Bold), FsWeight::Bold);
        assert_eq!(FsWeight::from(Weight::Black), FsWeight::Black);
    }

    #[test]
    fn from_family_style_legacy() {
        let font = Font::from_family_style("Roboto".to_string(), Some("Bold Italic".to_string()));

        assert_eq!(font.family, "Roboto");
        assert_eq!(font.weight, Weight::Bold);
        assert_eq!(font.style, "italic");
        assert_eq!(font.subset, Font::default_font_subset());
        assert!(font.path.is_none());

        let font = Font::from_family_style("Roboto".to_string(), None);
        assert_eq!(font.weight, Weight::default());
        assert_eq!(font.style, Font::default_font_style());
    }

    #[test]
    fn weight_from_str() {
        assert_eq!(Weight::from("thin"), Weight::Thin);
        assert_eq!(Weight::from("light"), Weight::Light);
        assert_eq!(Weight::from("regular"), Weight::Regular);
        assert_eq!(Weight::from("medium"), Weight::Medium);
        assert_eq!(Weight::from("bold"), Weight::Bold);
        assert_eq!(Weight::from("black"), Weight::Black);
        assert_eq!(Weight::from("something-else"), Weight::Regular);
    }

    #[test]
    fn deserialize_legacy_weight_and_style() {
        let font: Font = serde_saphyr::from_str(
            r#"
family: Roboto
style: Bold Italic
"#,
        )
        .unwrap();

        assert_eq!(font.family, "Roboto");
        assert_eq!(font.weight, Weight::Bold);
        assert_eq!(font.style, "italic");
    }

    #[test]
    fn deserialize_legacy_weight_only() {
        let font: Font = serde_saphyr::from_str(
            r#"
family: Roboto
style: 700
"#,
        )
        .unwrap();

        assert_eq!(font.weight, Weight::Bold);
        assert_eq!(font.style, Font::default_font_style());
    }

    #[test]
    fn deserialize_legacy_weight_precedence() {
        let font: Font = serde_saphyr::from_str(
            r#"
family: Roboto
style: bold italic
weight: light
"#,
        )
        .unwrap();

        assert_eq!(font.weight, Weight::Light);
        assert_eq!(font.style, "italic");
    }
}
