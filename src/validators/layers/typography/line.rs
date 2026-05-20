use std::num::NonZeroI32;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::Deserialize;

use crate::{ImgGenError, Result};

/// A custom type to ensure the minimum number of lines is a positive, non-zero [`f32`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineHeight(f32);

impl Default for LineHeight {
    fn default() -> Self {
        Self(1.0)
    }
}

impl LineHeight {
    pub fn new(height: f32) -> Option<Self> {
        if height <= 0.0 {
            None
        } else {
            Some(Self(height))
        }
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;

        let val = f32::deserialize(deserializer)?;
        Self::new(val).ok_or_else(|| {
            serde::de::Error::custom(format!("LineHeight must be greater than zero, got {val}"))
        })
    }
}

/// A property to implicitly describe the size of the text in a
/// [`Typography`](struct@super::Typography) attribute.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Debug, Clone, Deserialize)]
pub struct Line {
    /// The maximum number of lines in the layer.
    ///
    /// This value shall not be less than or equal to zero.
    #[serde(default = "Line::default_line_amount")]
    pub amount: NonZeroI32,
    /// The height ratio of each line in the layer.
    ///
    /// This value shall not be less than or equal to zero.
    #[serde(
        default = "LineHeight::default",
        deserialize_with = "LineHeight::deserialize"
    )]
    pub height: LineHeight,
}

impl Line {
    const fn default_line_amount() -> NonZeroI32 {
        #[allow(clippy::unwrap_used, reason = "1 != 0, and this is a const fn")]
        NonZeroI32::new(1).unwrap()
    }

    /// Calculate the font size given the max `height` bound.
    pub(crate) fn get_font_size(&self, height: u32, border_width: Option<u32>) -> Result<u32> {
        if height == 0 {
            return Err(ImgGenError::InvalidLayerHeight);
        }
        let available_height = height.saturating_sub(border_width.unwrap_or_default()) as f32;
        let max_size = available_height / (self.amount.get() as f32 * self.height.get());
        Ok(max_size.max(1.0) as u32)
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            amount: Self::default_line_amount(),
            height: LineHeight::default(),
        }
    }
}
