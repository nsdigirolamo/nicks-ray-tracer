use crate::color::Color;

#[derive(Default, Clone, Copy)]
pub struct Material {
    pub albedo: Color,
}

impl Material {
    pub fn new(albedo: Color) -> Self {        
        Self { albedo: albedo, }
    }
}