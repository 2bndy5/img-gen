use crate::SolidColor;
use pyo3::prelude::*;

#[pymethods]
impl SolidColor {
    /// Creates a solid color from ``r``, ``g``, ``b``, and ``a`` components.
    ///
    /// Each component is clamped to range [0, 255].
    #[new]
    #[pyo3(
        text_signature = "(r: int = 0, g: int = 0, b: int = 0, a: int = 0) -> None",
        signature = (r = 0, g = 0, b = 0, a = 0)
    )]
    pub fn new_py(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(r, g, b, a)
    }

    /// Instantiate a `SolidColor` from an string typically used in CSS.
    ///
    /// Valid color values include
    ///
    /// - named colors (see `CSS specifications <https://www.w3.org/TR/css-color-4/#named-colors>`_)
    /// - hexadecimal color codes (``#f00`` or ``#ff0000`` or ``#ff0000ff``)
    /// - RGB or RGBA strings (``"rgb(255, 0, 0)"`` or ``"rgba(255, 0, 0, 1.0"``)
    /// - HSL or HSLA string (``"hsl(0, 1.0, 1.0)"`` or ``"hsla(0, 1.0, 1.0, 1.0)"``)
    /// - HWB or HWBA string (``"hwb(0, 1.0, 1.0)"`` or ``"hwba(0, 1.0, 1.0, 1.0)"``)
    /// - HSV or HSVA string (``"hsv(0, 1.0, 1.0)"`` or ``"hsva(0, 1.0, 1.0, 1.0)"``)
    ///
    /// :throws: A `ValueError` is raised if the given value is not a proper CSS color value.
    #[staticmethod]
    #[pyo3(text_signature = "(val: str) -> Color", name = "from_string")]
    pub fn from_string_py(val: &str) -> PyResult<SolidColor> {
        Self::from_string(val).map_err(crate::python_binding::map_to_value_err)
    }

    /// Return a 4-integer tuple representing the `Color`.
    ///
    /// Each integer in the tuple will be in range [0, 255] and
    /// corresponds to the parameters in the `SolidColor` constructor.
    #[pyo3(text_signature = "() -> Tuple[int, int, int, int]", name = "to_tuple")]
    pub fn to_tuple_py(&self) -> (u8, u8, u8, u8) {
        self.to_tuple()
    }

    /// The color's red component.
    #[getter(r)]
    pub fn get_r_py(&self) -> u8 {
        self.get_r()
    }

    /// The color's green component.
    #[getter(g)]
    pub fn get_g_py(&self) -> u8 {
        self.get_g()
    }

    /// The color's blue component.
    #[getter(b)]
    pub fn get_b_py(&self) -> u8 {
        self.get_b()
    }

    /// The color's alpha component (AKA opacity).
    #[getter(a)]
    pub fn get_a_py(&self) -> u8 {
        self.get_a()
    }

    /// Sets the red component from ``val``.
    #[setter(r)]
    pub fn set_r_py(&mut self, val: u8) {
        self.set_r(val);
    }

    /// Sets the green component from ``val``.
    #[setter(g)]
    pub fn set_g_py(&mut self, val: u8) {
        self.set_g(val);
    }

    /// Sets the blue component from ``val``.
    #[setter(b)]
    pub fn set_b_py(&mut self, val: u8) {
        self.set_b(val);
    }

    /// Sets the alpha component from ``val``.
    #[setter(a)]
    pub fn set_a_py(&mut self, val: u8) {
        self.set_a(val);
    }

    /// Calculate a black or white foreground color using `Debug.color` as a background.
    #[pyo3(name = "get_foreground_color", text_signature = "() -> SolidColor")]
    pub fn get_foreground_color_py(&self) -> SolidColor {
        self.get_foreground_color()
    }

    /// Deserialize a `SolidColor` object from a YAML string.
    #[staticmethod]
    pub fn from_yaml_str(yaml_str: String) -> PyResult<Self> {
        crate::python_binding::parse_yaml_last_wins(&yaml_str)
    }

    /// Deserialize a `SolidColor` object from a JSON string.
    #[staticmethod]
    pub fn from_json_str(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `SolidColor` object to a JSON string.
    pub fn as_json_str(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }

    /// Serialize the `SolidColor` object to a YAML string.
    pub fn as_yaml_str(&self) -> PyResult<String> {
        serde_saphyr::to_string(self).map_err(crate::python_binding::map_to_value_err)
    }
}
