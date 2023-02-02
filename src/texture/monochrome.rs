use crate::color::Color;
use crate::texture::Texture;
use crate::vector3::Point3;

/// Represents a monochromatic texture.
#[derive(Clone, Copy)]
pub struct Monochrome {
    /// The texture's color.
    pub color: Color,
}

impl Monochrome {
    
    ///
    /// Returns a monochrome texture constructed from the given argument.
    ///
    /// # Argument
    /// * `color` - The monochrome texture's color.
    ///
    pub fn new(color: Color) -> Self {
        Self {
            color: color,
        }
    }
}

impl Texture for Monochrome {

    ///
    /// Returns the Color value for the texture using the given arguments.
    ///
    /// # Arguments
    /// * `&self` - The texture.
    /// * `point` - The point in space where the color exists.
    /// * `uv` - The uv coordinates of the point on the texture.
    ///
    #[allow(unused_variables)]
    fn get_color(&self, point: Point3, uv: (f64, f64)) -> Color {
        self.color
    }
}