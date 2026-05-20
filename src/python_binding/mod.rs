use pyo3::prelude::*;
mod generator;
mod validators;

use crate::{
    Background, Border, ColorGradient, ColorKind, ConicalGradient, Corners, Debug, Ellipse, Font,
    Generator, Icon, Image, Layer, LayerOffset, Layout, Line, LinearGradient, Mask, Polygon,
    PreserveAspect, Presets, RadialGradient, Rectangle, Size, SolidColor, Spread, Typography,
    TypographyAlign, Weight,
};

/// This module exposes all the public Python API needed to generate images.
#[pymodule]
fn img_gen(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Border>()?;
    m.add_class::<Background>()?;
    m.add_class::<Icon>()?;
    m.add_class::<PreserveAspect>()?;
    m.add_class::<SolidColor>()?;
    m.add_class::<Debug>()?;
    m.add_class::<Ellipse>()?;
    m.add_class::<Polygon>()?;
    m.add_class::<Font>()?;
    m.add_class::<LayerOffset>()?;
    m.add_class::<Layer>()?;
    m.add_class::<Layout>()?;
    m.add_class::<Line>()?;
    m.add_class::<Mask>()?;
    m.add_class::<Rectangle>()?;
    m.add_class::<Corners>()?;
    m.add_class::<Size>()?;
    m.add_class::<Typography>()?;
    m.add_class::<TypographyAlign>()?;
    m.add_class::<Weight>()?;
    m.add_class::<ColorKind>()?;
    m.add_class::<SolidColor>()?;
    m.add_class::<ColorGradient>()?;
    m.add_class::<LinearGradient>()?;
    m.add_class::<RadialGradient>()?;
    m.add_class::<ConicalGradient>()?;
    m.add_class::<Presets>()?;
    m.add_class::<Spread>()?;
    m.add_class::<Image>()?;
    m.add_class::<Generator>()?;
    Ok(())
}
