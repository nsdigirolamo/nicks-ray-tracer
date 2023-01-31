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
    #[allow(dead_code)]
    pub fn new(bounding_box: AABB, left: Rc<dyn Hittable>, right: Rc<dyn Hittable>) -> Self {
        Self {
            bounding_box: bounding_box,
            left: left,
            right: right,
        }
    }
}

/*
pub fn construct_bvh(hittables: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Rc<dyn Hittable> {

    let left: &Rc<dyn Hittable>;
    let right: &Rc<dyn Hittable>;
    let comparator = bounding_box_compare_x;
    let hittables_span = end - start;

    if hittables_span == 1 {

        left = &hittables[start];
        right = &hittables[start];

    } else if hittables_span == 2 {
        
        if comparator(&hittables[start], &hittables[end - 1]) == Ordering::Less {
            left = &hittables[start];
            right = &hittables[end - 1];
        } else {
            left = &hittables[end - 1];
            right = &hittables[start]
        }

    } else {
        
        hittables.sort_unstable_by(comparator);
        let middle = start + (hittables_span / 2);

        let left = construct_bvh(hittables, start, middle);
        let right = construct_bvh(hittables, middle, end);
        let bounding_box = surrounding_box(left.get_aabb(), right.get_aabb());
        return Rc::new(BvhNode::new(bounding_box, &left, &right));
    }

    let bounding_box = surrounding_box(left.get_aabb(), right.get_aabb());
    return Rc::new(BvhNode::new(bounding_box, left, right));
}

pub fn bounding_box_compare_x(h1: &Rc<dyn Hittable>, h2: &Rc<dyn Hittable>) -> Ordering {
    if h1.get_aabb().min.x < h2.get_aabb().min.x {
        Ordering::Less
    } else if h1.get_aabb().max.x > h2.get_aabb().max.x {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
*/

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