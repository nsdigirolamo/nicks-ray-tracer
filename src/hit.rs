use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::rand_vector3;
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
            normal: normal,
        }
    }

    pub fn scatter(&self) -> Ray {

        let hit_point = self.ray.get_point(self.distance);
        let mut direction: Vector3;

        match self.sphere.material.reflectivity {
            Some(reflectivity) => {
                direction = self.ray.direction.unit().reflect(self.normal);
                direction += reflectivity * rand_vector3();
            },
            None => {
                direction = self.normal.unit() + rand_vector3().unit();
                if direction.near_zero() { direction = self.normal }
            }
        }

        Ray::new(hit_point, direction) 
    }
}