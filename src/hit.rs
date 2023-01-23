use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Point3;
use crate::vector3::rand_vector3;
use crate::vector3::Vector3;

use rand::Rng;

/// Represents the hit between a ray and a sphere.
#[derive(Default, Clone, Copy)]
pub struct Hit {
    /// The ray that hit the sphere.
    pub ray: Ray,
    /// The sphere hit by the ray.
    pub sphere: Sphere,
    /// The position along the ray that the hit point is located.
    pub distance: f64,
    /// The point in space where the hit occurred.
    pub point: Point3,
    /// The normal of the surface of the sphere.
    pub normal: Vector3,
    /// Whether or not the ray has hit the front of the sphere's surface.
    pub is_front: bool,
}

impl Hit {
    
    ///
    /// Returns a Hit between a ray and a sphere at the given distance. Note that
    /// this function niavely assumes the given arguments are correct. A hit will
    /// be constructed even if the ray doesn't actually intersect with the sphere.
    ///
    /// # Arguments
    /// * `ray` - The ray that hit the sphere.
    /// * `sphere` - The sphere hit by the ray.
    /// * `distance` - The distance along the ray where the hit occurred.
    ///
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

    ///
    /// Returns a Ray that has been scattered based on the given hit's fields.
    ///
    /// # Arguments
    /// * `&self` - The hit that results in a scattered ray.
    ///
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

///
/// Returns the reflectance of a material as an f64. The reflectance is based on
/// the angle of the incoming ray and the index of refraction of the material.
///
/// # Arguments
/// * `cosine` - The angle of the incoming ray.
/// * `ref_idx` - The refraction index of the material being hit by the ray.
///
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}