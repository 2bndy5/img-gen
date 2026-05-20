use super::SolidColor;

mod gradient_presets;
pub use gradient_presets::Presets;

mod types;
pub use types::{ConicalGradient, LinearGradient, RadialGradient};

use crate::{ImgGenSpecError, Result};
use colorgrad::{Gradient, GradientBuilder, LinearGradient as RustLinearGradient};
use serde::Deserialize;
use std::fmt;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Normalize a `gradient_spec` into a range (`[0, 1.0]`) of color `&str`s
pub(super) fn parse_color_map_to_gradient<'s>(
    gradient_spec: &Vec<(f32, &'s str)>,
) -> (Vec<f32>, Vec<&'s str>) {
    // first sort the spec by values (f23)
    let mut domain = vec![];
    let mut colors = vec![];
    // if no colors were specified, return black to white as default
    if gradient_spec.is_empty() {
        domain.push(0.0);
        colors.push("black");
        domain.push(1.0);
        colors.push("white");
        return (domain, colors);
    }
    for (point, color) in gradient_spec {
        if domain.is_empty() {
            domain.push(*point);
            colors.push(*color);
        } else {
            let mut inserted = false;
            for i in 0..domain.len() {
                if domain[i] >= *point {
                    let index = i.saturating_sub(1);
                    domain.insert(index, *point);
                    colors.insert(index, *color);
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                domain.push(*point);
                colors.push(*color);
            }
        }
    }
    // ensure domain range is at least 0.0 to 1.0
    if let Some(point) = domain.first()
        && *point > 0.0
        && let Some(first_color) = colors.first()
    {
        domain.insert(0, 0.0);
        colors.insert(0, first_color);
    }
    if let Some(point) = domain.last()
        && *point < 1.0
        && let Some(last_color) = colors.last()
    {
        domain.push(1.0);
        colors.push(last_color);
    }
    (domain, colors)
}

/// A class to represent a gradient of colors.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Clone, Debug)]
pub struct ColorGradient {
    pub(super) inner: RustLinearGradient,
}

impl ColorGradient {
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
        let gradient = if let Some(color_spec) = spec {
            let (domain, colors) = parse_color_map_to_gradient(&color_spec);
            GradientBuilder::new()
                .html_colors(&colors)
                .domain(&domain)
                .build::<RustLinearGradient>()
        } else {
            Presets::get_gradient(preset.unwrap_or(Presets::MonoChrome))
        }
        .map_err(|e| ImgGenSpecError::InvalidGradientSpec {
            reason: e.to_string(),
        })?;
        Ok(ColorGradient { inner: gradient })
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
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Spread {
    #[default]
    Pad,
    Reflect,
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
        let empty_spec = vec![];
        let (domain, colors) = parse_color_map_to_gradient(&empty_spec);
        assert_eq!(domain, vec![0.0, 1.0]);
        assert_eq!(colors, vec!["black", "white"]);
    }

    /// parses an unordered gradient spec whose range is not a full [0, 1.0],
    /// and ensures it is sorted and normalized correctly
    #[test]
    fn parse_unordered_gradient_spec() {
        let spec = vec![(0.5, "green"), (0.1, "red"), (0.9, "blue")];
        let (domain, colors) = parse_color_map_to_gradient(&spec);
        assert_eq!(domain, vec![0.0, 0.1, 0.5, 0.9, 1.0]);
        assert_eq!(colors, vec!["red", "red", "green", "blue", "blue"]);
    }
}
