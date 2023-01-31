use crate::hittable::aabb::AABB;
use crate::hittable::Hittable;
use crate::ray::Ray;

use std::rc::Rc;

/// Represents a node within a bounding volume hierarchy.
pub struct BoundingVolumeHierarchyNode {
    /// This node's AABB.
    bounding_box: AABB,
    /// This node's left branch.
    left: Rc<dyn Hittable>,
    /// This node's right branch.
    right: Rc<dyn Hittable>,
}

/// Acronym for BoundingVolumeHierarchyNode.
pub type BvhNode = BoundingVolumeHierarchyNode;

impl BvhNode {
    
    ///
    /// Returns a BvhNode constructed from the given arguments.
    ///
    /// # Arguments
    /// * `bounding_box` - The BvhNode's box field.
    /// * `left` - The BvhNode's left field.
    /// * `right` - The BvhNode's right field.
    ///
    pub fn new(bounding_box: AABB, left: Rc<dyn Hittable>, right: Rc<dyn Hittable>) -> Self {
        Self {
            bounding_box: bounding_box,
            left: left,
            right: right,
        }
    }
}

impl Hittable for BvhNode {

    ///
    /// Returns true if there is a Hit between the BvhNode and a given Ray.
    ///
    /// # Arguments
    /// * `&self` - The BvhNode intersected by the Ray.
    /// * `ray` - The intersecting Ray.
    /// * `min_dist` - The minimum distance along the Ray to check for a Hit.
    /// * `max_dist` - The maximum distance along the Ray to check for a Hit.
    ///
    fn is_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> bool {
        if !self.bounding_box.is_hit(ray, min_dist, max_dist) {
            false
        } else {
            let is_left_hit = self.left.is_hit(ray, min_dist, max_dist);
            let is_right_hit = self.right.is_hit(ray, min_dist, max_dist);
            is_left_hit || is_right_hit
        }
    }

    ///
    /// Returns an AABB that surrounds the BvhNode.
    ///
    /// # Arguments
    /// * `&self` - The BvhNode.
    ///
    fn get_aabb(&self) -> AABB {
        self.bounding_box
    }
}