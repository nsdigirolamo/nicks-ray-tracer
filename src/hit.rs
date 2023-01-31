use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::Point3;
use crate::vector3::rand_vector3;
use crate::vector3::Vector3;

use rand::Rng;

/// Represents the intersection between a Ray and a Hittable.
#[derive(Default, Clone, Copy)]
pub struct Hit {
    /// The Ray that intersects the Hittable.
    pub ray: Ray,
    /// The distance along the Ray where the intersection is located.
    pub distance: f64,
    /// The position in space where the intersection is located.
    pub point: Point3,
    /// The normal of the Hittable's surface.
    pub normal: Vector3,
    /// True if the Ray intersects the front of the Hittable's surface.
    pub is_front: bool,
    /// The Material of the intersected Hittable.
    pub material: Material,
}

impl Hit {
    
    ///
    /// Returns a Hit between a Ray and a Hittable at the given distance along
    /// the Ray. Note that this function niavely assumes the given arguments
    /// are correct. A Hit will be constructed even if the Ray doesn't actually
    /// intersect with the Hittable.
    ///
    /// # Arguments
    /// * `ray` - The Hit's ray field.
    /// * `distance` - The Hit's distance field.
    /// * `point` - The Hit's point field.
    /// * `normal` - The Hit's normal field.
    /// * `is_front` - The Hit's is_front field.
    /// * `material` - The Hit's material field.
    ///
    pub fn new(ray: Ray, distance: f64, normal: Vector3, is_front: bool, material: Material) -> Self {
        Self {
            ray: ray,
            distance: distance,
            point: ray.get_point(distance),
            normal: normal,
            is_front: is_front,
            material: material,
        }
    }

    ///
    /// Returns a Ray that has been scattered based on the Hit's fields.
    ///
    /// # Arguments
    /// * `&self` - The Hit.
    ///
    pub fn scatter(&self) -> Ray {

        let mut direction = self.normal.unit() + rand_vector3().unit();
        if direction.near_zero() { direction = self.normal }

        match self.material.reflectivity {
            Some(reflectivity) => {
                let unit_direction = self.ray.direction.unit();
                direction = unit_direction.reflect(self.normal);
                direction += reflectivity * rand_vector3();
            },
            None => ()
        }

        match self.material.refraction_index {
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
/// Returns an f64 representing the reflectance of a Material. The reflectance
/// is calculated based on the angle of an incoming Ray and the Material's index
/// of refraction. 
///
/// # Arguments
/// * `cosine` - The angle of an incoming Ray.
/// * `ref_idx` - The Material's index of refraction.
///
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}