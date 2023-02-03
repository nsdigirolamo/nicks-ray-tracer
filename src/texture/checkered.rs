use crate::color::Color;
use crate::texture::Texture;

use std::f64::consts::PI;

/// Represents a checkered texture.
pub struct Checkered {
    /// The texture of one checkered square.
    pub texture1: Box<dyn Texture>,
    /// The texture of the other checkered square.
    pub texture2: Box<dyn Texture>,
    /// The scale of the squares.
    pub scale: f64,
}

impl Checkered {

    ///
    /// Returns a checkered texture constructed from the given arguments.
    ///
    /// # Argument
    /// * `texture1` - The checkered texture's color1 field.
    /// * `color2` - The checkered texture's color2 field.
    ///
    pub fn new(texture1: Box<dyn Texture>, texture2: Box<dyn Texture>, scale: f64) -> Self {
        Self {
            texture1: texture1,
            texture2: texture2,
            scale: scale,
        }
    }
}

impl Texture for Checkered {

    ///
    /// Returns the Color value for the texture using the given arguments.
    ///
    /// # Arguments
    /// * `&self` - The texture.
    /// * `point` - The point in space where the color exists.
    ///
    fn get_color(&self, uv: (f64, f64)) -> Color {
        let scale = (1.0 / self.scale) * 20.0;
        let u = (uv.0 * (scale * PI)).sin();
        let v = (uv.1 * (scale * PI)).sin();
        if (u < 0.0 && 0.0 < v) || (0.0 < u && v < 0.0) {
            self.texture1.get_color(uv)
        } else {
            self.texture2.get_color(uv)
        }
    }
}