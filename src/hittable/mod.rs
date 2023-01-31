pub mod sphere;

use crate::Hit;
use crate::Ray;

/// Represents any object that can be Hit by a Ray.
pub trait Hittable {

    fn get_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {
        None
    }

    fn is_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> bool {
        match self.get_hit(ray, min_dist, max_dist) {
            Some(_) => true,
            None => false,
        }
    }
}