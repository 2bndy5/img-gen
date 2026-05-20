#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

use super::{
    SolidColor,
    layers::{Background, Ellipse, Icon, LayerOffset, Polygon, Rectangle, Size, Typography},
};

/// An attribute to describe a [`Layer`]'s [`Mask`].
///
/// See [`LayerAttrKind::Mask`].
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "img_gen", get_all, set_all, from_py_object)
)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Mask {
    /// The mask's [`Size`].
    pub size: Option<Size>,
    /// The mask's [`LayerOffset`].
    pub offset: LayerOffset,
    /// A flag to control the behavior of the mask.
    ///
    /// False means only visible pixels are used in the mask.
    /// True means only invisible pixels are used in the mask.
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
#[derive(Debug, Clone, Default, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
pub struct Debug {
    /// A flag to enable or disable the debug output.
    #[serde(default)]
    pub enable: bool,
    /// A flag to control if the debug output shall show a grid of points over the layout.
    #[serde(default = "Debug::default_grid")]
    pub grid: bool,
    /// The space between points on the debug output's grid.
    #[serde(default = "Debug::default_grid_step")]
    pub grid_step: u32,
    /// The color used to outline debug output.
    #[serde(default = "Debug::default_color")]
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
#[derive(Debug, Clone, Default, Deserialize)]
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
