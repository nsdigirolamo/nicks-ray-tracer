pub mod sphere;

use crate::Hit;
use crate::Ray;

/// Represents any object that can be hit by a Ray.
pub trait Hittable {

    ///
    /// Returns a Hit if there is an intersection between the hittable and the
    /// given ray. If no such intersection exists, return None.
    ///
    /// # Arguments
    /// * `&self` - The hittable that the ray possibly intersects.
    /// * ray - The ray that is possibly intersecting the hittable.
    /// * min_dist - The minimum distance along the ray to look for an intersection.
    /// * max_dist - The maximum distance along the ray to look for an intersection.
    ///
    fn get_intersect(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit>;
}