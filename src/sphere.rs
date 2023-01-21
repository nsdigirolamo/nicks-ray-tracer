use crate::vector3::Point3;

#[derive(Default, Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center: center,
            radius: radius,
        }
    }
}