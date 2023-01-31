pub mod sphere;

use crate::Hit;
use crate::Ray;

/// Represents any object in 3D space that can be Hit by a Ray.
pub trait Hittable {

    ///
    /// Returns Some(Hit) if there is a Hit between the Hittable and a given Ray.
    ///
    /// # Arguments
    /// * `&self` - The Hittable intersected by the Ray.
    /// * `ray` - The intersecting Ray.
    /// * `min_dist` - The minimum distance along the Ray to check for a Hit.
    /// * `max_dist` - The maximum distance along the Ray to check for a Hit.
    ///
    fn get_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {
        None
    }

    ///
    /// Returns true if there is a Hit between the Hittable and a given Ray.
    ///
    /// # Arguments
    /// * `&self` - The Hittable intersected by the Ray.
    /// * `ray` - The intersecting Ray.
    /// * `min_dist` - The minimum distance along the Ray to check for a Hit.
    /// * `max_dist` - The maximum distance along the Ray to check for a Hit.
    ///
    fn is_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> bool {
        match self.get_hit(ray, min_dist, max_dist) {
            Some(_) => true,
            None => false,
        }
    }
}