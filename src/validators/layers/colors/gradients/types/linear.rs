use std::f32::consts::PI;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

use crate::{ColorGradient, LayerOffset, SolidColor, Spread};

/// A data structure to represent a linear gradient
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Deserialize)]
pub struct LinearGradient {
    #[serde(rename = "colors", alias = "preset")]
    inner: ColorGradient,
    /// The gradient starting point (`Offset`).
    pub start: LayerOffset,
    /// The gradient ending point (`Offset`).
    pub end: LayerOffset,
    /// The gradient `Spread`
    #[serde(default)]
    pub spread: Spread,
}

impl LinearGradient {
    pub fn new(
        colors: ColorGradient,
        start: LayerOffset,
        end: LayerOffset,
        spread: Option<Spread>,
    ) -> Self {
        Self {
            inner: colors,
            start,
            end,
            spread: spread.unwrap_or_default(),
        }
    }

    /// A helper function to [`Generator::render()`](fn@crate::Generator::render) behavior.
    ///
    /// The given `x` and `y` values are the coordinate of the pixel in the `Layer`
    /// (relative to [`Layer::offset`](value@crate::Layer::offset)).
    pub fn get_color_at(&self, x: u32, y: u32) -> SolidColor {
        let gradient_run = self.end.x - self.start.x;
        let gradient_rise = self.end.y - self.start.y;
        let point_rise = (y as i64 - self.start.y as i64) as i32;
        let point_run = (x as i64 - self.start.x as i64) as i32;
        if gradient_run == 0 {
            // x-axis is constant
            // only use position on y-axis
            let ratio = if point_rise == 0 {
                0f32
            } else {
                point_rise as f32 / gradient_rise as f32
            };
            return self.inner.get_color_at(ratio, &self.spread);
        }
        if gradient_rise == 0 {
            // y-axis is constant
            // only use position on x-axis
            let ratio = if point_run == 0 {
                0f32
            } else {
                point_run as f32 / gradient_run as f32
            };
            return self.inner.get_color_at(ratio, &self.spread);
        };
        debug_assert!(gradient_rise != 0);
        debug_assert!(gradient_run != 0);

        let gradient_slope = gradient_rise as f32 / gradient_run as f32;
        let gradient_angle = if point_run != 0 {
            let point_slope = point_rise as f32 / point_run as f32;
            (gradient_slope - point_slope).atan2(1.0 + (gradient_slope * point_slope))
        } else {
            // point_slope is infinite
            (gradient_rise as f32 / gradient_run as f32).atan() - (PI / 2.0)
        };
        // if point_run > 0 {
        //     println!("point run: {point_run}, angle: {gradient_angle}");
        // }
        let c2 = (point_rise as f32).powf(2.0) + (point_run as f32).powf(2.0);
        let hypotenuse_length = c2.sqrt() * if c2 < 0.0 { -1.0 } else { 1.0 };
        let intersection_len = gradient_angle.cos() * hypotenuse_length;
        let c2 = (gradient_rise as f32).powf(2.0) + (gradient_run as f32).powf(2.0);
        let gradient_length = c2.sqrt() * if c2 < 0.0 { -1.0 } else { 1.0 };
        let mut ratio = intersection_len / gradient_length;
        if ((point_run < 0 && gradient_run > 0) || (point_run > 0 && gradient_run < 0))
            || (point_run == 0
                && ((point_rise < 0 && gradient_rise > 0) || (point_rise > 0 && gradient_rise < 0)))
        {
            ratio *= -1.0;
        }
        self.inner.get_color_at(ratio, &self.spread)
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use super::LinearGradient;
    use crate::Spread;

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
        let color_kind: LinearGradient = serde_saphyr::from_str(gradient_str).unwrap();
        assert!(matches!(color_kind.spread, Spread::Pad));
        assert_eq!(color_kind.get_color_at(0, 0).to_tuple(), red_tuple);
        assert_eq!(color_kind.get_color_at(100, 100).to_tuple(), blue_tuple);
    }

    #[test]
    fn deserialize_linear_gradient_preset() {
        let black_tuple = (0, 0, 0, 255);
        let white_tuple = (255, 255, 255, 255);
        let gradient_str = r#"
start:
  x: 0
  y: 0
end:
  x: 100
  y: 100
preset: MonoChrome
"#;
        let color_kind: LinearGradient = serde_saphyr::from_str(gradient_str).unwrap();
        assert!(matches!(color_kind.spread, Spread::Pad));
        assert_eq!(color_kind.get_color_at(0, 0).to_tuple(), black_tuple);
        assert_eq!(color_kind.get_color_at(100, 100).to_tuple(), white_tuple);
    }
}
