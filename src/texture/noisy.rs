use crate::color::Color;
use crate::texture::Texture;

use noise::Perlin;
use noise::NoiseFn;
use std::f64::consts::PI;

/// Represents a noisy texture.
#[derive(Clone, Copy)]
pub struct Noisy {
    /// The perlin noise.
    pub perlin: Perlin,
    /// The perlin noise's scale. Larger values create smaller details.
    pub scale: f64,
    /// The number of times the perlin noise will be layered over itself.
    pub layers: u32,
    /// If the noise is turbulent.
    pub is_turbulent: bool,
    /// The texture's color.
    pub color: Color,
}

impl Noisy {

    ///
    /// Returns a noisy texture constructed from the given argument.
    ///
    /// # Arguments
    /// * `perlin` - The texture's perlin field.
    /// * `scale` - The texture's scale field.
    /// * `layers` - The texture's layers field.
    /// * `color` - The texture's color field.
    ///
    pub fn new(perlin: Perlin, scale: f64, layers: u32, is_turbulent: bool, color: Color) -> Self {
        Self {
            perlin: perlin,
            scale: scale,
            layers: layers,
            is_turbulent: is_turbulent,
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

        let mut accum = 0.0;
        let mut u = uv.0 * self.scale;
        let mut v = uv.1 * self.scale;
        let mut weight = 1.0;

        for level in 0..self.layers {
            let mut noise: f64;
            if self.is_turbulent {
                noise = self.perlin.get([u, v]);
            } else {
                noise = (self.perlin.get([u, v]) + 1.0) * 0.5;
            }
            accum += weight * noise;
            weight *= 0.5;
            u *= 2.0;
            v *= 2.0;
        }

        self.color * accum.abs()
    }
}