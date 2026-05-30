use crate::{ImgGenSpecError, Result};
use colorgrad::Color;
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// A class to represent a solid color.
///
/// Instantiate a `Color` from a 4 unsigned integers in range [0, 255].
/// Each parameter corresponds to the `Color` attributes:
///
/// - `r` for red
/// - `g` for green
/// - `b` for blue
/// - `a` for alpha (AKA opacity)
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Debug, Clone, Default)]
pub struct SolidColor {
    inner: Color,
}

/// A fully transparent RGBA color.
pub const TRANSPARENT: SolidColor = SolidColor {
    inner: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    },
};

impl SolidColor {
    /// Instantiate a [`SolidColor`] object from the given color components:
    ///
    /// - `r` for red
    /// - `g` for green
    /// - `b` for blue
    /// - `a` for alpha (transparency).
    ///   `0` is completely transparent; `255` is completely opaque.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            inner: Color::from_rgba8(r, g, b, a),
        }
    }

    /// Instantiate a [`SolidColor`] from a CSS-style string.
    ///
    /// Valid color values include
    ///
    /// - named colors (see [CSS specifications](https://www.w3.org/TR/css-color-4/#named-colors))
    /// - hexadecimal color codes (`#f00` or `#ff0000` or `#ff0000ff`)
    /// - RGB or RGBA strings (`"rgb(255, 0, 0)"` or `"rgba(255, 0, 0, 1.0"`)
    /// - HSL or HSLA string (`"hsl(0, 1.0, 1.0)"` or `"hsla(0, 1.0, 1.0, 1.0)"`)
    /// - HWB or HWBA string (`"hwb(0, 1.0, 1.0)"` or `"hwba(0, 1.0, 1.0, 1.0)"`)
    /// - HSV or HSVA string (`"hsv(0, 1.0, 1.0)"` or `"hsva(0, 1.0, 1.0, 1.0)"`)
    ///
    /// Returns an [`Err`] if the value is not a CSS color value.
    pub fn from_string(val: &str) -> Result<Self> {
        Ok(SolidColor {
            inner: Color::from_html(val).map_err(|e| ImgGenSpecError::InvalidCssColor {
                value: val.to_string(),
                reason: e.to_string(),
            })?,
        })
    }

    /// Returns the color components as an RGBA tuple.
    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (
            (self.inner.r * 255.0 + 0.5) as u8,
            (self.inner.g * 255.0 + 0.5) as u8,
            (self.inner.b * 255.0 + 0.5) as u8,
            (self.inner.a * 255.0 + 0.5) as u8,
        )
    }

    /// The color's red component.
    pub fn get_r(&self) -> u8 {
        (self.inner.r * 255.0 + 0.5) as u8
    }

    /// The color's green component.
    pub fn get_g(&self) -> u8 {
        (self.inner.g * 255.0 + 0.5) as u8
    }

    /// The color's blue component.
    pub fn get_b(&self) -> u8 {
        (self.inner.b * 255.0 + 0.5) as u8
    }

    /// The color's alpha component (AKA opacity).
    pub fn get_a(&self) -> u8 {
        (self.inner.a * 255.0 + 0.5) as u8
    }

    /// Sets the red component.
    pub fn set_r(&mut self, val: u8) {
        self.inner.r = val as f32 / 255.0;
    }

    /// Sets the green component.
    pub fn set_g(&mut self, val: u8) {
        self.inner.g = val as f32 / 255.0;
    }

    /// Sets the blue component.
    pub fn set_b(&mut self, val: u8) {
        self.inner.b = val as f32 / 255.0;
    }

    /// Sets the alpha component.
    pub fn set_a(&mut self, val: u8) {
        self.inner.a = val as f32 / 255.0;
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use super::SolidColor;

    #[test]
    fn from_attr() {
        let mut color = SolidColor::new(0, 0, 0, 255);
        assert_eq!(
            (color.get_r(), color.get_g(), color.get_b(), color.get_a()),
            color.to_tuple()
        );
        color.set_r(255);
        assert_eq!(color.get_r(), 255u8);
        color.set_g(255);
        assert_eq!(color.get_g(), 255u8);
        color.set_b(255);
        assert_eq!(color.get_b(), 255u8);
        color.set_a(127);
        assert_eq!(color.get_a(), 127u8);
    }

    #[test]
    fn from_str() {
        let color = SolidColor::from_string("red");
        assert!(color.is_ok());
        assert_eq!(color.unwrap().get_r(), 255u8);
        assert!(SolidColor::from_string("nan").is_err());
    }

    #[test]
    fn deserialize() {
        let expected_tuple = (255, 0, 0, 255);
        let parsed: SolidColor = serde_saphyr::from_str(r#""red""#).unwrap();
        assert_eq!(parsed.to_tuple(), expected_tuple);

        let parsed: SolidColor = serde_saphyr::from_str(r#""rgba(255, 0, 0, 1.0)""#).unwrap();
        assert_eq!(parsed.to_tuple(), expected_tuple);

        let parsed: SolidColor = serde_saphyr::from_str(r#""rgb(255, 0, 0)""#).unwrap();
        assert_eq!(parsed.to_tuple(), expected_tuple);

        let parsed: SolidColor = serde_saphyr::from_str(r#""hsla(0, 1.0, 0.5, 1.0)""#).unwrap();
        assert_eq!(parsed.to_tuple(), expected_tuple);
    }
}
