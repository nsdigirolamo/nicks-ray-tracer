pub mod monochrome;
pub mod checkered;

use crate::color::Color;
use crate::vector3::Point3;

pub trait Texture {

    ///
    /// Returns the Color value for the texture using the given arguments.
    ///
    /// # Arguments
    /// * `&self` - The texture.
    /// * `point` - The point in space where the color exists.
    /// * `uv` - The uv coordinates of the point on the texture.
    ///
    fn get_color(&self, point: Point3, uv: (f64, f64)) -> Color;
}