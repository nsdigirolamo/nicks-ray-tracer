use crate::color::Color;
use crate::texture::Texture;
use crate::vector3::Point3;

/// Represents a checkered texture.
pub struct Checkered {
    /// The texture of one checkered square.
    pub texture1: Box<dyn Texture>,
    /// The texture of the other checkered square.
    pub texture2: Box<dyn Texture>,
}

impl Checkered {

    ///
    /// Returns a checkered texture constructed from the given arguments.
    ///
    /// # Argument
    /// * `texture1` - The checkered texture's color1 field.
    /// * `color2` - The checkered texture's color2 field.
    ///
    pub fn new(texture1: Box<dyn Texture>, texture2: Box<dyn Texture>) -> Self {
        Self {
            texture1: texture1,
            texture2: texture2,
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
    /// * `uv` - The uv coordinates of the point on the texture.
    ///
    fn get_color(&self, point: Point3, uv: (f64, f64)) -> Color {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if sines < 0.0 {
            self.texture1.get_color(point, uv)
        } else {
            self.texture2.get_color(point, uv)
        }
    }
}