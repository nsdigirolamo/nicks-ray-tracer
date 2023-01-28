use crate::color::Color;

/// Represents the material of an object.
#[derive(Default, Clone, Copy)]
pub struct Material {
    /// The material's albedo.
    pub albedo: Color,
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
    pub fn new(albedo: Color, reflectivity: Option<f64>, refraction_index: Option<f64>) -> Self {        
        Self { 
            albedo: albedo, 
            reflectivity: reflectivity,
            refraction_index: refraction_index 
        }
    }
}