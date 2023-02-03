pub mod monochrome;
pub mod checkered;
pub mod noisy;

use crate::color::Color;

pub trait Texture {

    ///
    /// Returns the Color value for the texture using the given arguments.
    ///
    /// # Arguments
    /// * `&self` - The texture.
    /// * `point` - The point in space where the color exists.
    /// * `uv` - The uv coordinates of the point on the texture.
    ///
    fn get_color(&self, uv: (f64, f64)) -> Color;
}