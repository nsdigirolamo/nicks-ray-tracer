use crate::hit::Hit;
use crate::vector3::Vector3;
use crate::vector3::Point3;
use crate::sphere::Sphere;

/// Represents a ray in 3D space.
#[derive(Default, Clone, Copy)]
pub struct Ray {
    /// The ray's origin.
    pub origin: Point3,
    /// The ray's direction.
    pub direction: Vector3,
}

impl Ray {

    ///
    /// Returns a Ray with the given arguments.
    ///
    /// # Arguments
    /// * `origin` - The ray's origin.
    /// * `direction` - The ray's direction.
    ///
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self {
            origin: origin,
            direction: direction,
        }
    }

    ///
    /// Returns a Point3 on the ray at the given distance.
    ///
    /// # Arguments
    /// * `&self` - The ray that the point is on.
    /// * `distance` - The distance along the ray where the point is located.
    ///
    pub fn get_point(&self, distance: f64) -> Point3 {
        self.origin + (distance * self.direction)
    }

    ///
    /// Returns a Hit if there is an intersection between the given ray and
    /// the given sphere. If no such intersection exists, None is returned.
    ///
    /// # Arguments
    /// * `&self` - The ray that possibly intersects with the sphere.
    /// * sphere - The sphere that is possibly intersected by the ray.
    /// * min_dist - The minimum distance along the ray to look for an intersection.
    /// * max_dist - The maximum distance along the ray to look for an intersection.
    ///
    pub fn get_intersect(&self, sphere: &Sphere, min_dist: f64, max_dist: f64) -> Option<Hit> {
        
        let oc = self.origin - sphere.center;
        let a = self.direction.mag_squared();
        let half_b = oc.dot(self.direction);
        let c = oc.mag_squared() - (sphere.radius * sphere.radius);

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

        Some(Hit::new(*self, *sphere, root))
    }
}