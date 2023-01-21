use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Vector3;

#[derive(Default, Clone, Copy)]
pub struct Hit {
    pub ray: Ray,
    pub sphere: Sphere,
    pub distance: f64,
    pub normal: Vector3,
}

impl Hit {
    pub fn new(ray: Ray, sphere: Sphere, distance: f64, normal: Vector3) -> Self {
        Self {
            ray: ray,
            sphere: sphere,
            distance: distance,
            normal: normal.unit(),
        }
    }
}