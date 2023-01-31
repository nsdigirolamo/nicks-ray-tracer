use crate::hittable::Hittable;
use crate::Ray;
use crate::vector3::Point3;

use std::mem::swap;

/// Represents an axis-aligned bounding box in 3D space.
#[derive(Default, Clone, Copy)]
pub struct AxisAlignedBoundingBox {
    /// The minimum corner of the bounding box.
    pub min: Point3,
    /// The maximum corner of the bounding box.
    pub max: Point3,
}

/// Acronym for AxisAlignedBoundingBox.
pub type AABB = AxisAlignedBoundingBox;

impl AABB {

    ///
    /// Returns an AABB constructed from the given arguments.
    ///
    /// # Arguments
    /// * `min` - The AABB's min field.
    /// * `max` - The AABB's max field.
    ///
    pub fn new(min: Point3, max: Point3) -> Self {
        Self {
            min: min,
            max: max,
        }
    }
}

///
/// Returns an AABB that surrounds the two given AABBs.
///
/// # Arguments
/// * `b1` - One of the surrounded AABBs.
/// * `b2` - One of the surrounded AABBs.
///
#[allow(dead_code)]
pub fn surrounding_box(b1: AABB, b2: AABB) -> AABB {
    let min = Point3::new(
        b1.min.x.min(b2.min.x),
        b1.min.y.min(b2.min.y),
        b1.min.z.min(b2.min.z),
    );
    let max = Point3::new(
        b1.max.x.max(b2.max.x),
        b1.max.y.max(b2.max.y),
        b1.max.z.max(b2.max.z),
    );
    AABB::new(min, max)
}

impl Hittable for AABB {

    ///
    /// Returns true if there is a Hit between the AABB and a given Ray.
    ///
    /// # Arguments
    /// * `&self` - The AABB intersected by the Ray.
    /// * `ray` - The intersecting Ray.
    /// * `min_dist` - The minimum distance along the Ray to check for a Hit.
    /// * `max_dist` - The maximum distance along the Ray to check for a Hit.
    ///
    #[allow(unused_variables)]
    fn is_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> bool {

        let inverse_x_direction = ray.direction.x;
        let mut x_min_dist = (self.min.x - ray.origin.x) * inverse_x_direction;
        let mut x_max_dist = (self.max.x - ray.origin.x) * inverse_x_direction;
        if inverse_x_direction < 0.0 { swap(&mut x_min_dist, &mut x_max_dist) }

        let inverse_y_direction = ray.direction.y;
        let mut y_min_dist = (self.min.y - ray.origin.y) * inverse_y_direction;
        let mut y_max_dist = (self.max.y - ray.origin.y) * inverse_y_direction;
        if inverse_y_direction < 0.0 { swap(&mut y_min_dist, &mut y_max_dist) }

        let inverse_z_direction = ray.direction.z;
        let mut z_min_dist = (self.min.z - ray.origin.z) * inverse_z_direction;
        let mut z_max_dist = (self.max.z - ray.origin.z) * inverse_z_direction;
        if inverse_z_direction < 0.0 { swap(&mut z_min_dist, &mut z_max_dist) }

        let xy_overlap = x_min_dist < y_max_dist && y_min_dist < x_max_dist;
        let yz_overlap = y_min_dist < z_max_dist && z_min_dist < y_max_dist;
        let zx_overlap = z_min_dist < x_max_dist && x_min_dist < z_max_dist;

        xy_overlap && yz_overlap && zx_overlap
    }

    ///
    /// Returns the AABB.
    ///
    /// # Arguments
    /// * `&self` - The AABB.
    ///
    fn get_aabb(&self) -> AABB {
        *self
    }
}