use crate::color::Color;
use crate::texture::Texture;

use noise::Perlin;
use noise::NoiseFn;

/// Represents a noisy texture.
#[derive(Clone, Copy)]
pub struct Noisy {
    /// The perlin noise.
    pub perlin: Perlin,
    /// The perlin noise's scale.
    pub scale: f64,
    /// The texture's color.
    pub color: Color,
}

impl Noisy {
    
    ///
    /// Returns a noisy texture constructed from the given argument.
    ///
    /// # Argument
    /// * `seed` - The perlin noise's seed.
    /// * `color` - The noisy texture's color.
    ///
    pub fn new(perlin: Perlin, scale: f64, color: Color) -> Self {
        Self {
            perlin: perlin,
            scale: scale,
            color: color,
        }
    }
}

impl Texture for Noisy {

    ///
    /// Returns the Color value for the texture using the given arguments.
    ///
    /// # Arguments
    /// * `&self` - The texture.
    /// * `point` - The point in space where the color exists.
    /// * `uv` - The uv coordinates of the point on the texture.
    ///
    #[allow(unused_variables)]
    fn get_color(&self, uv: (f64, f64)) -> Color {

        let u = uv.0 * self.scale;
        let v = uv.1 * self.scale;

        let lower_limit = 0.5;
        let upper_limit = lower_limit + 2.0;
        let val = (self.perlin.get([u, v]) + (1.0 + lower_limit)) * (1.0 / upper_limit);
        self.color * val
    }
}