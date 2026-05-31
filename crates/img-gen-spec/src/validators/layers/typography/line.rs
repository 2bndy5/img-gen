use std::num::NonZeroI32;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{ImgGenSpecError, Result};

/// A custom type to ensure the minimum number of lines is a positive, non-zero [`f32`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LineHeight(f32);

impl Default for LineHeight {
    fn default() -> Self {
        Self(1.0)
    }
}

impl LineHeight {
    /// Creates a line-height ratio where `height` is not equal to zero.
    pub fn new(height: f32) -> Option<Self> {
        if height == 0.0 {
            None
        } else {
            Some(Self(height))
        }
    }

    /// Returns the validated line-height ratio.
    pub fn get(&self) -> f32 {
        self.0
    }

    /// Deserializes a non-zero line-height ratio.
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;

        let val = f32::deserialize(deserializer)?;
        Self::new(val).ok_or_else(|| {
            serde::de::Error::custom(format!("LineHeight must not be zero, got {val}"))
        })
    }
}

/// A property to implicitly describe the size of the text in a
/// [`Typography`](struct@super::Typography) attribute.
#[cfg_attr(feature = "pyo3", pyclass(module = "img_gen", from_py_object))]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn get_font_size(&self, height: u32, border_width: Option<u32>) -> Result<u32> {
        if height == 0 {
            return Err(ImgGenSpecError::InvalidLayerHeight);
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn test_line_height() {
        assert!(LineHeight::new(1.0).is_some());
        assert!(LineHeight::new(0.0).is_none());
        assert!(LineHeight::new(-1.0).is_some());
    }

    #[test]
    fn test_line_font_size() {
        let line = Line {
            amount: NonZeroI32::new(2).unwrap(),
            height: LineHeight(1.5),
        };
        let err = line.get_font_size(0, Some(10)).unwrap_err();
        assert!(matches!(err, ImgGenSpecError::InvalidLayerHeight));
    }

    #[test]
    fn deserialize_bad_line_height() {
        let json = r#"
        {
            "amount": 2,
            "height": 0.0
        }
        "#;
        let err = serde_json::from_str::<Line>(json).unwrap_err();
        assert!(err.to_string().contains("LineHeight must not be zero"));
    }
}
