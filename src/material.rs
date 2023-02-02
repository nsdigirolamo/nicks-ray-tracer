use crate::texture::Texture;

/// Represents the material of a Hittable
pub struct Material {
    /// The material's texture.
    pub texture: Box<dyn Texture>,
    /// The material's reflectivity, if applicable.
    pub reflectivity: Option<f64>,
    /// The material's refraction index, if applicable.
    pub refraction_index: Option<f64>,
}

impl Material {
    
    ///
    /// Returns a Material with the given arguments.
    ///
    /// # Arguments
    /// * `albedo` - The material's albedo.
    /// * `reflectivity` - The material's reflectivity, if applicable.
    /// * `refraction_index` - The material's refraction index, if applicable.
    ///
    pub fn new(texture: Box<dyn Texture>, reflectivity: Option<f64>, refraction_index: Option<f64>) -> Self {        
        Self { 
            texture: texture, 
            reflectivity: reflectivity,
            refraction_index: refraction_index 
        }
    }
}