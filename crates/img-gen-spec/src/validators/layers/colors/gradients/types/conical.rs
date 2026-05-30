#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

use crate::{ColorGradient, LayerOffset, SolidColor, Spread};

/// A data structure to represent a conical gradient.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Debug, Clone, Deserialize)]
pub struct ConicalGradient {
    #[serde(rename = "colors", alias = "preset")]
    inner: ColorGradient,

    /// The center point (`Offset`) of the gradient (relative to the `Layer.offset`).
    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub center: LayerOffset,

    /// The center point (`Offset`) of the gradient (relative to the `Layer.offset`).
    #[cfg(not(feature = "pyo3"))]
    pub center: LayerOffset,

    /// The starting angle (in degrees) of the gradient.
    ///
    /// A ``0`` degree angle is at 3 o'clock. This angle increases counter-clockwise.
    #[serde(default)]
    angle: f32,
}

fn clamp_angle(val: &mut f32) {
    while *val >= 360.0 {
        *val -= 360.0;
    }
    while *val < 0.0 {
        *val += 360.0;
    }
}

impl ConicalGradient {
    /// Creates a conical gradient from `colors`, `center`, and
    /// an optional starting `angle`.
    pub fn new(colors: ColorGradient, center: LayerOffset, angle: Option<f32>) -> Self {
        let mut a = angle.unwrap_or(0.0f32);
        clamp_angle(&mut a);
        Self {
            inner: colors,
            center,
            angle: a,
        }
    }

    /// The starting angle (in degrees) of the gradient.
    ///
    /// A ``0`` degree angle is at 3 o'clock. This angle increases counter-clockwise.
    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    /// Sets the starting angle in degrees.
    ///
    /// Values are normalized into the inclusive range `[0.0, 360.0)`.
    pub fn set_angle(&mut self, val: f32) {
        let mut a = val;
        clamp_angle(&mut a);
        self.angle = a;
    }

    /// A helper function for renderers to get the color at a certain point.
    ///
    /// The given `x` and `y` values are the coordinate of the pixel in the `Layer`
    /// (relative to [`Layer::offset`](value@crate::Layer::offset)).
    pub fn get_color_at(&self, x: u32, y: u32) -> SolidColor {
        let y_diff = (y as i64 - self.center.y as i64) as f32;
        let x_diff = (x as i64 - self.center.x as i64) as f32;
        let mut angle = y_diff.atan2(x_diff).to_degrees() + self.angle;
        if angle >= 360.0 {
            angle -= 360.0;
        }
        if angle < 0.0 {
            angle += 360.0;
        }
        self.inner.get_color_at(1.0 - (angle / 360.0), &Spread::Pad)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

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
        let conical: ConicalGradient = serde_saphyr::from_str(gradient_str).unwrap();
        assert_eq!(conical.get_angle(), 0.0);
        assert_eq!(conical.center.x, 1);
        assert_eq!(conical.center.y, 1);
        assert_eq!(conical.get_color_at(2, 1).to_tuple(), blue_tuple);
        assert_eq!(conical.get_color_at(50, 0).to_tuple(), red_tuple);
    }

    #[test]
    fn deserialize_conical_gradient_preset() {
        let gradient_str = r#"
center:
  x: 1
  y: 1
angle: 0.0
preset: MonoChrome
"#;
        let conical: ConicalGradient = serde_saphyr::from_str(gradient_str).unwrap();
        assert_eq!(conical.get_angle(), 0.0);
        assert_eq!(conical.center.x, 1);
        assert_eq!(conical.center.y, 1);
    }
}
