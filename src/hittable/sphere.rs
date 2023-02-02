use crate::Hit;
use crate::hittable::aabb::AABB;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::Ray;
use crate::vector3::Point3;
use crate::vector3::Vector3;

use std::f64::consts::PI;

/// Represents a sphere in 3D space.
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

    ///
    /// Returns a (f64, f64) tuple of the Sphere's (u, v) texture coordinates at
    /// the given point.
    ///
    /// u is an f64 between [0, 1] that is the angle around the y axis from x = -1
    /// v is an f64 between [0, 1] that is the angle from y = -1 to y = 1
    ///
    /// # Arguments
    /// * `&self` - The Sphere.
    /// * `point` - The point on the sphere's surface.
    ///
    fn get_uv(&self, point: Point3) -> (f64, f64) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
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

        Some(Hit::new(
            ray,
            distance,
            normal,
            is_front,
            self.material.texture.get_color(point, self.get_uv(normal)),
            self.material.reflectivity,
            self.material.refraction_index,
        ))
    }

    ///
    /// Returns an AABB that surrounds the Sphere.
    ///
    /// # Arguments
    /// * `&self` - The Sphere.
    ///
    fn get_aabb(&self) -> AABB {
        let radius_vector = Vector3::new(self.radius, self.radius, self.radius);
        AABB::new(self.center - radius_vector, self.center + radius_vector)
    }
}