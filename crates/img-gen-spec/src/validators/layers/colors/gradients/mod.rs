use super::SolidColor;

mod gradient_presets;
pub use gradient_presets::Presets;

mod types;
pub use types::{ConicalGradient, LinearGradient, RadialGradient};

use crate::{ImgGenSpecError, Result};
use colorgrad::{Gradient, GradientBuilder, LinearGradient as RustLinearGradient};
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

fn parse_color_map_to_gradient<'s>(gradient_spec: &[(f32, &'s str)]) -> Vec<(f32, &'s str)> {
    let mono;
    let gradient_spec = if gradient_spec.is_empty() {
        mono = Presets::get_stops(Presets::MonoChrome);
        &mono
    } else {
        gradient_spec
    };

    let mut normalized: Vec<(f32, &'s str)> = Vec::new();

    for (point, color) in gradient_spec {
        if normalized.is_empty() {
            normalized.push((*point, color));
            continue;
        }

        let mut inserted = false;
        for i in 0..normalized.len() {
            if normalized[i].0 >= *point {
                normalized.insert(i, (*point, color));
                inserted = true;
                break;
            }
        }

        if !inserted {
            normalized.push((*point, color));
        }
    }

    if let Some((point, color)) = normalized.first()
        && *point > 0.0
    {
        normalized.insert(0, (0.0, *color));
    }
    if let Some((point, color)) = normalized.last()
        && *point < 1.0
    {
        normalized.push((1.0, *color));
    }

    normalized
}

/// A class to represent a gradient of colors.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Clone, Debug)]
pub struct ColorGradient {
    pub(super) inner: RustLinearGradient,
    pub(super) stops: Vec<(f32, SolidColor)>,
}

impl ColorGradient {
    fn from_str_spec(spec: &[(f32, &str)]) -> Result<Self> {
        let normalized = parse_color_map_to_gradient(spec);

        let mut stops = Vec::with_capacity(normalized.len());
        let mut domain = Vec::with_capacity(normalized.len());
        let mut color_strs = Vec::with_capacity(normalized.len());
        for (point, color_str) in &normalized {
            stops.push((*point, SolidColor::from_string(color_str)?));
            domain.push(*point);
            color_strs.push(*color_str);
        }

        let inner = GradientBuilder::new()
            .html_colors(&color_strs)
            .domain(&domain)
            .build::<RustLinearGradient>()
            .map_err(|e| ImgGenSpecError::InvalidGradientSpec {
                reason: e.to_string(),
            })?;

        Ok(Self { inner, stops })
    }

    /// Instantiate a [`ColorGradient`] object.
    ///
    /// The `spec` parameter is an optional list of `(f32, String)` pairs.
    /// The `float` value is a percentage ranging `[0.0, 1.0]` that describes
    /// where the color (the `str` value) should take place.
    /// See [`SolidColor::from_string()`] for supported String values that describe the color.
    ///
    /// If `spec` parameter is not specified, then the optional `preset` parameter
    /// (which defaults to [`Presets::MonoChrome`]) is used instead.
    pub fn new(spec: Option<Vec<(f32, &str)>>, preset: Option<Presets>) -> Result<Self> {
        let preset_stops;
        let effective: &[(f32, &str)] = if let Some(ref s) = spec {
            s
        } else {
            preset_stops = Presets::get_stops(preset.unwrap_or(Presets::MonoChrome));
            &preset_stops
        };
        Self::from_str_spec(effective)
    }

    /// A helper function to get the interpolated color of the gradient at the specified `position`.
    ///
    /// If `position` is beyond the range `[0, 1.0]`, then the `spread` parameter is
    /// applied accordingly. See the `Spread` for more details.
    pub fn get_color_at(&self, position: f32, spread: &Spread) -> SolidColor {
        let [r, g, b, a] = match spread {
            Spread::Pad => self.inner.at(position),
            Spread::Reflect => self.inner.reflect_at(position),
            Spread::Repeat => self.inner.repeat_at(position),
        }
        .to_rgba8();
        SolidColor::new(r, g, b, a)
    }
}

/// A enumeration to describe gradient spread behavior at the bounds of the gradient.
///
/// This only applies to `LinearGradient` and `RadialGradient`, not `ConicalGradient`.
#[cfg_attr(
    feature = "pyo3",
    pyclass(eq, eq_int, module = "img_gen", from_py_object)
)]
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Spread {
    /// Extends the edge colors beyond the gradient bounds.
    #[default]
    Pad,
    /// Mirrors the gradient repeatedly beyond the gradient bounds.
    Reflect,
    /// Restarts the gradient from the beginning beyond the gradient bounds.
    Repeat,
}

impl fmt::Display for Spread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Spread::Pad => write!(f, "pad"),
            Spread::Reflect => write!(f, "reflect"),
            Spread::Repeat => write!(f, "repeat"),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn deserialize_gradient_spec() {
        let yaml = r#"{0.0: "red", 0.5: "green", 1.0: "blue"}"#;
        let gradient: ColorGradient = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(
            gradient.get_color_at(0.0, &Spread::Pad).to_tuple(),
            (255, 0, 0, 255)
        );
        // assert_eq!(gradient.get_color_at(0.5, &Spread::Pad).to_tuple(), (0, 255, 0, 255));
        assert_eq!(
            gradient.get_color_at(1.0, &Spread::Pad).to_tuple(),
            (0, 0, 255, 255)
        );
    }

    #[test]
    fn deserialize_gradient_preset() {
        let yaml = r#"MonoChrome"#;
        let gradient: ColorGradient = serde_saphyr::from_str(yaml).unwrap();
        let expected = ColorGradient::new(None, Some(Presets::MonoChrome)).unwrap();
        let spread = Spread::Pad;
        assert_eq!(
            gradient.get_color_at(0.5, &spread).to_tuple(),
            expected.get_color_at(0.5, &spread).to_tuple()
        );

        let yaml = r#"Non-existent"#;
        serde_saphyr::from_str::<ColorGradient>(yaml).unwrap_err();
    }

    #[test]
    fn parse_empty_gradient_spec() {
        let normalized = parse_color_map_to_gradient(&[]);
        let domain: Vec<f32> = normalized.iter().map(|(point, _)| *point).collect();
        let colors: Vec<&str> = normalized.into_iter().map(|(_, color)| color).collect();
        assert_eq!(domain, vec![0.0, 1.0]);
        assert_eq!(colors, vec!["black", "white"]);
    }

    /// parses an unordered gradient spec whose range is not a full [0, 1.0],
    /// and ensures it is sorted and normalized correctly
    #[test]
    fn parse_unordered_gradient_spec() {
        let spec = vec![(0.5, "green"), (0.1, "red"), (0.9, "blue")];
        let normalized = parse_color_map_to_gradient(&spec);
        let domain: Vec<f32> = normalized.iter().map(|(point, _)| *point).collect();
        let colors: Vec<&str> = normalized.into_iter().map(|(_, color)| color).collect();
        assert_eq!(domain, vec![0.0, 0.1, 0.5, 0.9, 1.0]);
        assert_eq!(colors, vec!["red", "red", "green", "blue", "blue"]);
    }

    #[test]
    fn preset_to_index() {
        assert_eq!(Presets::MonoChrome.index(), 0);
        assert_eq!(Presets::try_from_index(255), None);
    }
}
