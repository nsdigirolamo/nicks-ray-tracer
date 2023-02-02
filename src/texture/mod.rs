pub mod monochrome;

use crate::color::Color;

pub trait Texture {

    ///
    /// Returns the Color value for the texture at the given u, v coordinates.
    ///
    /// # Arguments
    /// * `&self` - The texture.
    /// * `u` - The texture's u coordinate.
    /// * `v` - The texture's v coordinate.
    ///
    fn get_color(&self, u: f64, v: f64) -> Color;
}