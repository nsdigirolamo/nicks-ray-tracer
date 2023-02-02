use crate::color::Color;
use crate::texture::Texture;

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
    /// Returns the monochrome texture's color value.
    ///
    /// # Arguments
    /// * `&self` - The texture.
    /// * `u` - The texture's u coordinate.
    /// * `v` - The texture's v coordinate.
    ///
    fn get_color(&self, u: f64, v: f64) -> Color {
        self.color
    }
}