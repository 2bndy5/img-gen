#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, MapAccess, Visitor},
};

use super::{
    SolidColor,
    layers::{Background, Ellipse, Icon, LayerOffset, Polygon, Rectangle, Size, Typography},
};

/// An attribute to describe a [`Layer`]'s [`Mask`].
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mask {
    /// The mask's [`Size`].
    pub size: Option<Size>,

    /// The mask's [`LayerOffset`].
    #[serde(default)]
    pub offset: LayerOffset,

    /// A flag to control the behavior of the mask.
    ///
    /// False means only visible pixels are used in the mask.
    /// True means only invisible pixels are used in the mask.
    #[serde(default)]
    pub invert: bool,

    /// A background attribute for the mask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Background>,
    /// A rectangle attribute for the mask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle: Option<Rectangle>,
    /// An ellipse attribute for the mask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipse: Option<Ellipse>,
    /// An polygon attribute for the mask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Polygon>,
    /// An icon attribute for the mask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /// A typography attribute for the mask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typography: Option<Typography>,
}

/// A data structure to represent a single [`Layer`] in a [`Layout`].
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Layer {
    /// The layer's [`Size`]
    pub size: Option<Size>,

    /// The layer's [`LayerOffset`].
    #[serde(default)]
    pub offset: LayerOffset,

    /// A background attribute for the layer.
    pub background: Option<Background>,
    /// A rectangle attribute for the layer.
    pub rectangle: Option<Rectangle>,
    /// An ellipse attribute for the layer.
    pub ellipse: Option<Ellipse>,
    /// An polygon attribute for the layer.
    pub polygon: Option<Polygon>,
    /// An icon attribute for the layer.
    pub icon: Option<Icon>,
    /// A typography attribute for the layer.
    pub typography: Option<Typography>,
    /// A mask attribute for the layer.
    pub mask: Option<Mask>,
}

/// A struct to describe a [`Layout`]'s visual debug output.
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", set_all, get_all, from_py_object)
)]
#[derive(Debug, Clone, Serialize)]
pub struct Debug {
    /// A flag to enable or disable the debug output.
    pub enable: bool,
    /// A flag to control if the debug output shall show a grid of points over the layout.
    pub grid: bool,
    /// The space between points on the debug output's grid.
    pub grid_step: u32,
    /// The color used to outline debug output.
    pub color: SolidColor,
}

impl Debug {
    /// Calculate a black or white foreground color using [`Debug::color`] as a background.
    pub fn get_foreground_color(&self) -> SolidColor {
        let luminance = {
            let mut result = 0.0f32;
            for (index, c) in vec![
                &self.color.get_r(),
                &self.color.get_g(),
                &self.color.get_b(),
            ]
            .into_iter()
            .take(3)
            .enumerate()
            {
                let component = *c as f32 / 255.0;
                let new_component = if component <= 0.03928 {
                    component / 12.92
                } else {
                    ((component + 0.055) / 1.055).powf(2.4)
                };
                match index {
                    0 => {
                        result += 0.2126 * new_component;
                    }
                    1 => {
                        result += 0.7152 * new_component;
                    }
                    _ => {
                        result += 0.0722 * new_component;
                    }
                }
            }
            result
        };
        if luminance > 0.451 {
            SolidColor::new(0, 0, 0, 255)
        } else {
            SolidColor::new(255, 255, 255, 255)
        }
    }

    pub(crate) const fn default_grid_step() -> u32 {
        30
    }

    pub(crate) fn default_color() -> SolidColor {
        SolidColor::new(128, 128, 128, 255)
    }

    const fn default_grid() -> bool {
        true
    }
}

impl<'de> Deserialize<'de> for Debug {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct DebugVisitor;

        impl<'de> Visitor<'de> for DebugVisitor {
            type Value = Debug;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean or a debug config object")
            }

            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Debug, E> {
                Ok(if v {
                    Debug {
                        enable: true,
                        ..Debug::default()
                    }
                } else {
                    Debug::default()
                })
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Debug, A::Error> {
                let mut enable = None;
                let mut grid = None;
                let mut grid_step = None;
                let mut color = None;

                while let Some(key) = map.next_key::<std::borrow::Cow<str>>()? {
                    match key.as_ref() {
                        "enable" => enable = Some(map.next_value()?),
                        "grid" => grid = Some(map.next_value()?),
                        "grid_step" => grid_step = Some(map.next_value()?),
                        "color" => color = Some(map.next_value()?),
                        unknown => {
                            return Err(de::Error::unknown_field(
                                unknown,
                                &["enable", "grid", "grid_step", "color"],
                            ));
                        }
                    }
                }

                Ok(Debug {
                    enable: enable.unwrap_or(false),
                    grid: grid.unwrap_or_else(Debug::default_grid),
                    grid_step: grid_step.unwrap_or_else(Debug::default_grid_step),
                    color: color.unwrap_or_else(Debug::default_color),
                })
            }
        }

        deserializer.deserialize_any(DebugVisitor)
    }
}

impl Default for Debug {
    fn default() -> Self {
        Self {
            enable: false,
            grid: Self::default_grid(),
            grid_step: Self::default_grid_step(),
            color: Self::default_color(),
        }
    }
}

/// A data structure used to represent a generated image's layout.
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", set_all, get_all, from_py_object)
)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Layout {
    /// The layout's `Size`
    #[serde(default)]
    pub size: Size,

    /// A list of the layout's `Layer` objects.
    #[serde(default)]
    pub layers: Vec<Layer>,

    /// An optional `Debug` attribute can be used to show the constraints of the `Layout`'s `layers`.
    pub debug: Option<Debug>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::Debug;

    fn assert_all_defaults(d: &Debug) {
        assert_eq!(d.grid, Debug::default_grid());
        assert_eq!(d.grid_step, Debug::default_grid_step());
        assert_eq!(d.color.to_tuple(), Debug::default_color().to_tuple());
    }

    #[test]
    fn debug_bool_true() {
        let d: Debug = serde_saphyr::from_str("true").unwrap();
        assert!(d.enable);
        assert_all_defaults(&d);
    }

    #[test]
    fn debug_bool_false() {
        let d: Debug = serde_saphyr::from_str("false").unwrap();
        assert!(!d.enable);
        assert_all_defaults(&d);
    }

    #[test]
    fn debug_map_full() {
        let yaml = "enable: true\ngrid: false\ngrid_step: 50\ncolor: \"blue\"\n";
        let d: Debug = serde_saphyr::from_str(yaml).unwrap();
        assert!(d.enable);
        assert!(!d.grid);
        assert_eq!(d.grid_step, 50);
        assert_eq!(d.color.to_tuple(), (0, 0, 255, 255));
    }

    #[test]
    fn debug_map_defaults() {
        let d: Debug = serde_saphyr::from_str("{}").unwrap();
        assert!(!d.enable);
        assert_all_defaults(&d);
    }

    #[test]
    fn debug_map_unknown_field() {
        let result: Result<Debug, _> = serde_saphyr::from_str("unknown_key: true\n");
        assert!(result.is_err());
    }

    #[test]
    fn debug_invalid_type() {
        // Passing an integer triggers the unimplemented visitor path, which calls expecting()
        let result: Result<Debug, _> = serde_saphyr::from_str("42");
        assert!(result.is_err());
    }

    #[test]
    fn debug_foreground_dark_color() {
        // Black (0,0,0): all channels <= 0.03928 threshold -> component / 12.92 path (L111)
        // Luminance = 0 <= 0.451 -> white foreground
        let d: Debug = serde_saphyr::from_str("color: \"black\"\n").unwrap();
        assert_eq!(d.get_foreground_color().to_tuple(), (255, 255, 255, 255));
    }

    #[test]
    fn debug_foreground_bright_color() {
        // White (255,255,255): luminance ~= 1.0 > 0.451 -> black foreground (L130)
        let d: Debug = serde_saphyr::from_str("color: \"white\"\n").unwrap();
        assert_eq!(d.get_foreground_color().to_tuple(), (0, 0, 0, 255));
    }
}
