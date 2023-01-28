use crate::vector3::Vector3;
use crate::vector3::Point3;

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
}