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
    /// Returns a Sphere constructed from the given arguments.
    ///
    /// # Arguments
    /// * `center` - The Sphere's center field.
    /// * `radius` - The Sphere's radius field.
    /// * `material` - The Sphere's material field.
    ///
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere {

    ///
    /// Returns Some(Hit) if there is a Hit between the Sphere and a given Ray.
    ///
    /// # Arguments
    /// * `&self` - The Sphere intersected by the Ray.
    /// * `ray` - The intersecting Ray.
    /// * `min_dist` - The minimum distance along the Ray to check for a Hit.
    /// * `max_dist` - The maximum distance along the Ray to check for a Hit.
    ///
    fn get_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {
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

        let distance = root;
        let point = ray.get_point(distance);
        let mut normal = (point - self.center) / self.radius;
        let is_front = ray.direction.dot(normal) < 0.0;
        if !is_front { normal = -normal };

        Some(Hit::new(ray, distance, normal, is_front, self.material))
    }
}