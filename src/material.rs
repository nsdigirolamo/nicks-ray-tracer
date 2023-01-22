use crate::color::Color;

#[derive(Default, Clone, Copy)]
pub struct Material {
    pub albedo: Color,
    pub reflectivity: Option<f64>,
    pub refraction_index: Option<f64>,
}

impl Material {
    pub fn new(albedo: Color, reflectivity: Option<f64>, refraction_index: Option<f64>) -> Self {        
        Self { 
            albedo: albedo, 
            reflectivity: reflectivity,
            refraction_index: refraction_index 
        }
    }
}