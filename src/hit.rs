use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Point3;
use crate::vector3::rand_vector3;
use crate::vector3::Vector3;

use rand::Rng;

#[derive(Default, Clone, Copy)]
pub struct Hit {
    pub ray: Ray,
    pub sphere: Sphere,
    pub distance: f64,
    pub point: Point3,
    pub normal: Vector3,
    pub is_front: bool,
}

impl Hit {
    pub fn new(ray: Ray, sphere: Sphere, distance: f64) -> Self {

        let point = ray.get_point(distance);
        let sphere_normal = (point - sphere.center) / sphere.radius;
        let is_front = ray.direction.dot(sphere_normal) < 0.0;
        let mut normal = sphere_normal;
        if !is_front { normal = -sphere_normal }

        Self {
            ray: ray,
            sphere: sphere,
            distance: distance,
            point: point,
            normal: normal,
            is_front: is_front,
        }
    }

    pub fn scatter(&self) -> Ray {

        let mut direction = self.normal.unit() + rand_vector3().unit();
        if direction.near_zero() { direction = self.normal }

        match self.sphere.material.reflectivity {
            Some(reflectivity) => {
                let unit_direction = self.ray.direction.unit();
                direction = unit_direction.reflect(self.normal);
                direction += reflectivity * rand_vector3();
            },
            None => ()
        }

        match self.sphere.material.refraction_index {
            Some(ri) => {

                let mut rng = rand::thread_rng();

                let mut refraction_ratio = ri;
                if self.is_front { refraction_ratio = 1.0 / ri }

                let unit_direction = self.ray.direction.unit();
                let cos_theta = (-unit_direction).dot(self.normal).min(1.0);
                let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen() {
                    direction = unit_direction.reflect(self.normal);
                } else {
                    direction = unit_direction.refract(self.normal, refraction_ratio)
                }
            },
            None => ()
        }

        Ray::new(self.point, direction) 
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}