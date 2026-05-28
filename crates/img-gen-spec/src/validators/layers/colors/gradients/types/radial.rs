#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

use crate::{ColorGradient, SolidColor, Spread, validators::LayerOffset};

/// A data structure to represent a radial gradient.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Debug, Clone, Deserialize)]
pub struct RadialGradient {
    #[serde(rename = "colors", alias = "preset")]
    inner: ColorGradient,

    /// The center (`Offset`) of the gradient (relative to the `Layer.offset`).
    center: LayerOffset,

    /// The radius of the gradient.
    radius: f32,

    /// The focal point `Offset`  of the gradient (relative to the `Layer.offset`).
    focal_point: Option<LayerOffset>,

    /// The radius from the focal_point (bounded internally by the gradient radius).
    #[serde(default)]
    focal_radius: f32,

    /// The gradient `Spread`
    #[cfg(feature = "pyo3")]
    #[serde(default)]
    #[pyo3(get, set)]
    pub spread: Spread,

    /// The gradient `Spread`
    #[cfg(not(feature = "pyo3"))]
    #[serde(default)]
    pub spread: Spread,
}

impl RadialGradient {
    const HOT_SPOT_OFFSETS: [(f32, f32); 4] =
        [(-0.25, -0.25), (0.25, -0.25), (-0.25, 0.25), (0.25, 0.25)];

    fn clamp_focal_radius(&mut self) {
        let focal_point = self.focal_point.unwrap_or(self.center);
        let x_diff = (focal_point.x - self.center.x) as f32;
        let y_diff = (focal_point.y - self.center.y) as f32;
        let focal_center_distance = (x_diff * x_diff + y_diff * y_diff).sqrt();
        // Keep the focal circle safely inside the end circle to avoid near-tangent
        // singular behavior. Reflect/repeat need a wider buffer than pad.
        let guard_band = match self.spread {
            Spread::Pad => (self.radius * 0.01).max(1.0),
            Spread::Reflect | Spread::Repeat => (self.radius * 0.12).max(12.0),
        };
        let max_allowed = (self.radius - focal_center_distance - guard_band).max(0.0);

        if self.focal_radius < 0.0 {
            self.focal_radius = 0.0;
            return;
        }
        if self.focal_radius > max_allowed {
            self.focal_radius = max_allowed;
        }
    }

    fn clamp_focal_point(&mut self) {
        if let Some(focal_point) = self.focal_point.as_mut() {
            let x_diff = focal_point.x - self.center.x;
            let y_diff = focal_point.y - self.center.y;
            let x_diff_f = x_diff as f32;
            let y_diff_f = y_diff as f32;
            let point_center_radius = (x_diff_f * x_diff_f + y_diff_f * y_diff_f).sqrt();
            if point_center_radius < self.radius {
                self.clamp_focal_radius();
                return; // point is within gradient radius
            }

            // Keep focal point just inside the end circle while preserving direction from center.
            let clamped_radius = (self.radius - 1.0).max(0.0);
            let scale = if point_center_radius == 0.0 {
                0.0
            } else {
                clamped_radius / point_center_radius
            };
            focal_point.x = self.center.x + (x_diff_f * scale).round() as i32;
            focal_point.y = self.center.y + (y_diff_f * scale).round() as i32;
            self.clamp_focal_radius();
        } else {
            self.clamp_focal_radius();
        }
    }

    fn solve_ratio_at_coords(&self, px: f32, py: f32) -> f32 {
        let focal_point = self.focal_point.unwrap_or(self.center);

        // Solve for `t` in the two-circle radial gradient equation:
        // |P - (F + t(C - F))| = fr + t(R - fr)
        let fx = focal_point.x as f32;
        let fy = focal_point.y as f32;
        let cx = self.center.x as f32;
        let cy = self.center.y as f32;

        let fr = self.focal_radius;
        let r = self.radius;
        let s = r - fr;

        let mx = px - fx;
        let my = py - fy;
        let dx = cx - fx;
        let dy = cy - fy;

        let m2 = (mx * mx) + (my * my);
        let d2 = (dx * dx) + (dy * dy);

        const EPS: f32 = 1.0e-6;
        const EPS_A: f32 = 1.0e-4;

        // The focal circle is the t=0 contour for two-circle radial gradients.
        // Keeping this region pinned avoids branch artifacts near the singularity.
        if m2 <= fr * fr {
            return 0.0;
        }

        let ray_ratio_guess = self.solve_ratio_from_focal_ray(mx, my, m2);

        // Degenerate case: start and end radii are effectively identical.
        if s.abs() <= EPS {
            return 1.0;
        }

        // Concentric circles: fallback to simple radial distance mapping.
        if d2 <= EPS {
            return (m2.sqrt() - fr) / s;
        }

        let a = d2 - (s * s);
        let b = -2.0 * ((mx * dx) + (my * dy) + (fr * s));
        let c = m2 - (fr * fr);

        // Near tangent (or exact tangent) turns quadratic into a near-linear solve.
        // Blend to a stable focal-ray solve when linear terms also become tiny.
        if a.abs() <= EPS_A {
            if b.abs() <= EPS {
                return ray_ratio_guess;
            }
            let t_linear = -c / b;
            if t_linear.is_finite() {
                return t_linear;
            }
            return ray_ratio_guess;
        }

        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < -EPS {
            let t_vertex = -b / (2.0 * a);
            if t_vertex.is_finite() {
                return t_vertex;
            }
            return ray_ratio_guess;
        }

        let sqrt_disc = discriminant.max(0.0).sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        let t1_valid = t1.is_finite();
        let t2_valid = t2.is_finite();
        if !t1_valid && !t2_valid {
            return ray_ratio_guess;
        }
        if t1_valid && !t2_valid {
            return t1;
        }
        if t2_valid && !t1_valid {
            return t2;
        }

        // Prefer the branch closest to a stable geometric estimate to avoid
        // branch flips near the degenerate/tangent configuration.
        if (t1 - ray_ratio_guess).abs() <= (t2 - ray_ratio_guess).abs() {
            t1
        } else {
            t2
        }
    }

    fn solve_ratio_from_focal_ray(&self, mx: f32, my: f32, m2: f32) -> f32 {
        const EPS: f32 = 1.0e-6;
        let focal_point = self.focal_point.unwrap_or(self.center);

        if m2 <= EPS {
            return 0.0;
        }

        let point_dist = m2.sqrt();
        let ux = mx / point_dist;
        let uy = my / point_dist;

        let fx = focal_point.x as f32;
        let fy = focal_point.y as f32;
        let cx = self.center.x as f32;
        let cy = self.center.y as f32;

        let fcx = fx - cx;
        let fcy = fy - cy;
        let b = 2.0 * ((ux * fcx) + (uy * fcy));
        let c = (fcx * fcx) + (fcy * fcy) - (self.radius * self.radius);
        let disc = (b * b) - (4.0 * c);
        if disc < 0.0 {
            return 0.0;
        }

        let sqrt_disc = disc.sqrt();
        let d1 = (-b - sqrt_disc) / 2.0;
        let d2 = (-b + sqrt_disc) / 2.0;
        let end_dist = d1.max(d2);
        let span = end_dist - self.focal_radius;
        if span.abs() <= EPS {
            return if point_dist <= self.focal_radius {
                0.0
            } else {
                1.0
            };
        }

        (point_dist - self.focal_radius) / span
    }

    pub fn new(
        colors: ColorGradient,
        center: LayerOffset,
        radius: f32,
        focal_point: Option<LayerOffset>,
        focal_radius: Option<f32>,
        spread: Option<Spread>,
    ) -> Self {
        let mut gradient = Self {
            inner: colors,
            center,
            radius,
            focal_point,
            focal_radius: focal_radius.unwrap_or_default(),
            spread: spread.unwrap_or_default(),
        };
        gradient.clamp_focal_point();
        gradient
    }

    /// The center (`Offset`) of the gradient (relative to the `Layer.offset`).
    pub fn get_center(&self) -> LayerOffset {
        self.center
    }

    pub fn set_center(&mut self, val: LayerOffset) {
        self.focal_point = Some(val);
        self.focal_radius = 0.0f32;
        self.center = val;
    }

    /// The radius of the gradient.
    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn set_radius(&mut self, val: f32) {
        self.radius = val;
        self.clamp_focal_point();
    }

    /// The focal point (`Offset`)  of the gradient (relative to the `Layer.offset`).
    pub fn get_focal_point(&self) -> LayerOffset {
        self.focal_point.unwrap_or(self.center)
    }

    pub fn set_focal_point(&mut self, val: LayerOffset) {
        self.focal_point = Some(val);
        self.clamp_focal_point();
    }

    pub fn get_focal_radius(&self) -> f32 {
        self.focal_radius
    }

    pub fn set_focal_radius(&mut self, val: f32) {
        self.focal_radius = val;
        self.clamp_focal_point();
    }

    /// A helper function for renderers to get the color at a certain point.
    ///
    /// The given `x` and `y` values are the coordinate of the pixel in the `Layer`
    /// (relative to [`Layer::offset`](value@crate::Layer::offset)).
    pub fn get_color_at(&self, x: u32, y: u32) -> SolidColor {
        let focal_point = self.focal_point.unwrap_or(self.center);

        // Sample at pixel center.
        let sample_x = x as f32 + 0.5;
        let sample_y = y as f32 + 0.5;
        let focal_x = focal_point.x as f32;
        let focal_y = focal_point.y as f32;

        // Qt-like hot spot mitigation: anti-alias only around the focal singularity.
        // This avoids a hard pixel hot spot when focal_radius ~= 0 while keeping
        // the rest of the gradient unchanged.
        let is_zero_focal = self.focal_radius <= 1.0e-6;
        let focal_dx = sample_x - focal_x;
        let focal_dy = sample_y - focal_y;
        let focal_dist2 = (focal_dx * focal_dx) + (focal_dy * focal_dy);
        let hot_spot_radius = match self.spread {
            Spread::Pad => 1.0,
            Spread::Reflect | Spread::Repeat => 2.0,
        };

        if is_zero_focal && focal_dist2 <= hot_spot_radius * hot_spot_radius {
            let mut r_sum: u32 = 0;
            let mut g_sum: u32 = 0;
            let mut b_sum: u32 = 0;
            let mut a_sum: u32 = 0;

            for (ox, oy) in Self::HOT_SPOT_OFFSETS {
                let ratio = self.solve_ratio_at_coords(sample_x + ox, sample_y + oy);
                let (r, g, b, a) = self.inner.get_color_at(ratio, &self.spread).to_tuple();
                r_sum += r as u32;
                g_sum += g as u32;
                b_sum += b as u32;
                a_sum += a as u32;
            }

            return SolidColor::new(
                (r_sum / 4) as u8,
                (g_sum / 4) as u8,
                (b_sum / 4) as u8,
                (a_sum / 4) as u8,
            );
        }

        let ratio = self.solve_ratio_at_coords(sample_x, sample_y);
        self.inner.get_color_at(ratio, &self.spread)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    fn sample_gradient() -> ColorGradient {
        ColorGradient::new(Some(vec![(0.0, "red"), (1.0, "blue")]), None).unwrap()
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
        let radial: RadialGradient = serde_saphyr::from_str(gradient_str).unwrap();
        assert!(matches!(radial.spread, Spread::Pad));
        assert_eq!(radial.get_radius(), 100.0);
        assert_eq!(radial.get_color_at(50, 50).to_tuple(), red_tuple);
        assert_eq!(radial.get_color_at(150, 150).to_tuple(), blue_tuple);
    }

    #[test]
    fn deserialize_radial_gradient_preset() {
        let gradient_str = r#"
radius: 100.0
center:
  x: 50
  y: 50
focal_point:
  x: 50
  y: 50
focal_radius: 0.0
preset: MonoChrome
"#;
        let radial: RadialGradient = serde_saphyr::from_str(gradient_str).unwrap();
        assert!(matches!(radial.spread, Spread::Pad));
        assert_eq!(radial.get_radius(), 100.0);
    }

    #[test]
    fn radial_gradient_getters_and_setters_round_trip() {
        let center = LayerOffset { x: 10, y: 20 };
        let mut radial = RadialGradient::new(
            sample_gradient(),
            center,
            40.0,
            Some(LayerOffset { x: 13, y: 24 }),
            Some(3.0),
            Some(Spread::Pad),
        );

        let focal_point = radial.get_focal_point();
        assert_eq!(radial.get_center().x, 10);
        assert_eq!(radial.get_center().y, 20);
        assert_eq!(radial.get_radius(), 40.0);
        assert_eq!(focal_point.x, 13);
        assert_eq!(focal_point.y, 24);
        assert_eq!(radial.get_focal_radius(), 3.0);

        radial.set_center(LayerOffset { x: -5, y: 6 });
        let centered_focal_point = radial.get_focal_point();
        assert_eq!(radial.get_center().x, -5);
        assert_eq!(radial.get_center().y, 6);
        assert_eq!(centered_focal_point.x, -5);
        assert_eq!(centered_focal_point.y, 6);
        assert_eq!(radial.get_focal_radius(), 0.0);

        radial.set_radius(18.0);
        radial.set_focal_point(LayerOffset { x: -2, y: 10 });
        radial.set_focal_radius(6.0);

        let updated_focal_point = radial.get_focal_point();
        assert_eq!(radial.get_radius(), 18.0);
        assert_eq!(updated_focal_point.x, -2);
        assert_eq!(updated_focal_point.y, 10);
        assert_eq!(radial.get_focal_radius(), 6.0);
    }

    #[test]
    fn radial_gradient_clamps_negative_focal_radius_without_focal_point() {
        let center = LayerOffset { x: 8, y: -3 };
        let radial = RadialGradient::new(
            sample_gradient(),
            center,
            12.0,
            None,
            Some(-4.0),
            Some(Spread::Pad),
        );

        let focal_point = radial.get_focal_point();
        assert_eq!(focal_point.x, center.x);
        assert_eq!(focal_point.y, center.y);
        assert_eq!(radial.get_focal_radius(), 0.0);
    }

    #[test]
    fn radial_gradient_clamps_out_of_bounds_focal_values() {
        let radial = RadialGradient::new(
            sample_gradient(),
            LayerOffset { x: 0, y: 0 },
            10.0,
            Some(LayerOffset { x: 30, y: 0 }),
            Some(20.0),
            Some(Spread::Pad),
        );

        let focal_point = radial.get_focal_point();
        assert_eq!(focal_point.x, 9);
        assert_eq!(focal_point.y, 0);
        assert_eq!(radial.get_focal_radius(), 0.0);

        let reflected = RadialGradient::new(
            sample_gradient(),
            LayerOffset { x: 0, y: 0 },
            200.0,
            Some(LayerOffset { x: 50, y: 0 }),
            Some(180.0),
            Some(Spread::Reflect),
        );

        assert_eq!(reflected.get_focal_radius(), 126.0);
    }
}
