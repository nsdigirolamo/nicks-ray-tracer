use crate::Hit;
use crate::hittable::Hittable;
use crate::ray::Ray;

/// Represents everything that could be seen in a rendered image.
pub struct Scene {
    /// All hittable objects within a scene.
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    
    ///
    /// Returns a Scene with the given arguments.
    ///
    /// # Arguments
    /// * `hittables` - All hittable objects within a scene.
    ///
    pub fn new() -> Self {
        Self { hittables: Vec::new() }
    }

    ///
    /// Returns a Hit for the closest intersection between the given ray and all
    /// hittables within a scene. If no such intersection exists, return None.
    ///
    /// # Arguments
    /// * `&self` - The scene the ray exists in.
    /// * `ray` - The ray that will possibly intersect with a hittable in the scene.
    /// * `min_dist` - The minimum distance along the ray to look for an intersection.
    /// * `max_dist` - The maximum distance along the ray to look for an intersection.
    ///
    pub fn get_intersect(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {
        let mut found_intersect = false;
        let mut min_intersect_distance = max_dist;
        let mut hit: Hit = Default::default();

        for hittable in &self.hittables {
            match hittable.get_hit(ray, min_dist, max_dist) {
                Some(h) => {
                    if h.distance < min_intersect_distance {
                        min_intersect_distance = h.distance;
                        found_intersect = true;
                        hit = h;
                    }
                },
                None => (),
            }
        }

        if found_intersect { Some(hit) } else { None }
    }
}