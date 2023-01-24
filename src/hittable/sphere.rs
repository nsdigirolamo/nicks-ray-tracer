use crate::Hit;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::Ray;
use crate::vector3::Point3;

/// Represents a sphere in 3D space.
#[derive(Default, Clone, Copy)]
pub struct Sphere {
    /// The sphere's center.
    pub center: Point3,
    /// The sphere's radius.
    pub radius: f64,
    /// The sphere's material.
    pub material: Material,
}

impl Sphere {

    ///
    /// Returns a Sphere with the given arguments.
    ///
    /// # Arguments
    /// * `center` - The sphere's center.
    /// * `radius` - The sphere's radius.
    /// * `material` - The sphere's material.
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn get_intersect(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.mag_squared() - (self.radius * self.radius);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return None; }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < min_dist || max_dist < root {
            root = (-half_b + sqrtd) / a;
            if root < min_dist || max_dist < root {
                return None;
            }
        }

        Some(Hit::new(ray, *self, root))
    }
}