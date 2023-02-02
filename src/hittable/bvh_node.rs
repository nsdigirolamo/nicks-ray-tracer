use crate::hittable::aabb::AABB;
use crate::hittable::aabb::surrounding_box;
use crate::hit::Hit;
use crate::hittable::Hittable;
use crate::ray::Ray;

use rand::Rng;
use std::rc::Rc;
use std::cmp::Ordering;

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

///
/// Returns a BvhNode that is the root of a BVH. The BVH is constructed from the
/// values in src_hittables that are between the start and end indices. These values
/// are sorted, and then construct_bvh is recursively called to make a new BVH
/// out of the sorted values. All values will be sorted again and again until
/// they make up the leaves of the hierarchy.
///
/// # Arguments
/// * `src_hittables` - The Hittables that will make up the leaves of the hierarchy.
/// * `start` - Where to start looking for values in src_hittables.
/// * `end` - Where to stop looking for values in src_hittables.
///
pub fn construct_bvh_root(src_hittables: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {

    let mut rng = rand::thread_rng();
    let comparator: fn(&Rc<dyn Hittable>, &Rc<dyn Hittable>) -> Ordering;

    match rng.gen_range(0..3) {
        0 => comparator = bounding_box_compare_x,
        1 => comparator = bounding_box_compare_y,
        _ => comparator = bounding_box_compare_z,
    }

    let left: Rc<dyn Hittable>;
    let right: Rc<dyn Hittable>;

    let hittables = &mut src_hittables[start..end].to_vec();

    if hittables.len() == 1 {

        left = Rc::clone(&hittables[0]);
        right = Rc::clone(&hittables[0]);

    } else if hittables.len() == 2 {

        if comparator(&hittables[0], &hittables[1]) == Ordering::Less {
            left = Rc::clone(&hittables[0]);
            right = Rc::clone(&hittables[1]);
        } else {
            left = Rc::clone(&hittables[1]);
            right = Rc::clone(&hittables[0]);
        }

    } else {

        hittables.sort_unstable_by(comparator);

        let middle = hittables.len() / 2;
        left = construct_bvh(hittables, 0, middle);
        right = construct_bvh(hittables, middle, hittables.len());
    }

    let bounding_box = surrounding_box(left.get_aabb(), right.get_aabb());
    BvhNode::new(bounding_box, left, right)
}

///
/// Returns a Rc<dyn Hittable> that is simply the return of construct_bvh_root
/// wrapped in a Rc pointer. This needs to be done because the left and right nodes
/// of a BVH node are Rc<dyn Hittable> to allow for the leaves to be something
/// other than BVH nodes.
///
/// # Arguments
/// * `src_hittables` - The Hittables that will make up the leaves of the hierarchy.
/// * `start` - Where to start looking for values in src_hittables.
/// * `end` - Where to stop looking for values in src_hittables.
///
pub fn construct_bvh(src_hittables: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Rc<dyn Hittable> {
    Rc::new(construct_bvh_root(src_hittables, start, end))
}

///
/// Returns an Ordering based on the x positions of the given Hittable's AABBs.
/// This is a utility function used by construct_bvh_root to sort its list of
/// Rc<dyn Hittable> pointers.
///
/// # Arguments
/// * `h1` - The first Hittable to have its AABB compared.
/// * `h2` - The second Hittable to have its AABB compared.
///
fn bounding_box_compare_x(h1: &Rc<dyn Hittable>, h2: &Rc<dyn Hittable>) -> Ordering {
    if h1.get_aabb().min.x < h2.get_aabb().min.x {
        Ordering::Less
    } else if h1.get_aabb().max.x > h2.get_aabb().max.x {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

///
/// Returns an Ordering based on the y positions of the given Hittable's AABBs.
/// This is a utility function used by construct_bvh_root to sort its list of
/// Rc<dyn Hittable> pointers.
///
/// # Arguments
/// * `h1` - The first Hittable to have its AABB compared.
/// * `h2` - The second Hittable to have its AABB compared.
///
fn bounding_box_compare_y(h1: &Rc<dyn Hittable>, h2: &Rc<dyn Hittable>) -> Ordering {
    if h1.get_aabb().min.y < h2.get_aabb().min.y {
        Ordering::Less
    } else if h1.get_aabb().max.y > h2.get_aabb().max.y {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

///
/// Returns an Ordering based on the z positions of the given Hittable's AABBs.
/// This is a utility function used by construct_bvh_root to sort its list of
/// Rc<dyn Hittable> pointers.
///
/// # Arguments
/// * `h1` - The first Hittable to have its AABB compared.
/// * `h2` - The second Hittable to have its AABB compared.
///
fn bounding_box_compare_z(h1: &Rc<dyn Hittable>, h2: &Rc<dyn Hittable>) -> Ordering {
    if h1.get_aabb().min.z < h2.get_aabb().min.z {
        Ordering::Less
    } else if h1.get_aabb().max.z > h2.get_aabb().max.z {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

impl Hittable for BvhNode {

    ///
    /// Returns Some(Hit) if there is a Hit between the BvhNode and a given Ray.
    ///
    /// # Arguments
    /// * `&self` - The BvhNode intersected by the Ray.
    /// * `ray` - The intersecting Ray.
    /// * `min_dist` - The minimum distance along the Ray to check for a Hit.
    /// * `max_dist` - The maximum distance along the Ray to check for a Hit.
    ///
    fn get_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {

        if !self.bounding_box.is_hit(ray, min_dist, max_dist) { return None; }
        let mut closest_distance = max_dist;
        let mut hit: Option<Hit> = None;

        match self.left.get_hit(ray, min_dist, max_dist) {
            Some(h) => { 
                if h.distance < closest_distance {
                    closest_distance = h.distance;
                    hit = Some(h);
                }
            },
            None => (),
        }

        match self.right.get_hit(ray, min_dist, max_dist) {
            Some(h) => { 
                if h.distance < closest_distance {
                    // closest_distance = h.distance;
                    hit = Some(h);
                } 
            },
            None => (),
        }

        hit
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