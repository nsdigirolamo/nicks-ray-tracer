use crate::Hit;
use crate::hittable::Hittable;
use crate::hittable::bvh_node::BvhNode;
use crate::hittable::bvh_node::construct_bvh_root;
use crate::ray::Ray;

use std::rc::Rc;

/// Represents everything that could be seen in a rendered image.
pub struct Scene {
    /// All hittable objects within a scene.
    hittables: Vec<Rc<dyn Hittable>>,
    /// The root node of this scene's bounding volume hierarchy.
    bvh_root: BvhNode,
}

impl Scene {

    ///
    /// Returns a Scene with the given arguments.
    ///
    /// # Arguments
    /// * `hittables` - All hittable objects within a scene.
    ///
    pub fn new(initial_hittable: Rc<dyn Hittable>) -> Self {

        let mut hittables = vec![initial_hittable];
        Self {
            bvh_root: construct_bvh_root(&mut hittables, 0, 1),
            hittables: hittables,
        }
    }

    ///
    /// Adds a Rc<dyn Hittable> to the Scene's hittables field.
    ///
    /// # Arguments
    /// * `&mut self` - The Scene.
    /// * `hittable` - The hittable to add.
    ///
    pub fn push(&mut self, hittable: Rc<dyn Hittable>) {
        self.hittables.push(hittable);
        let length = self.hittables.len();
        self.bvh_root = construct_bvh_root(&mut self.hittables, 0, length);
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

        self.bvh_root.get_hit(ray, min_dist, max_dist)

    }
}