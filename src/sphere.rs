use crate::material::Material;
use crate::vector3::Point3;

/// Represents a sphere in 3D space.
#[derive(Default, Clone, Copy)]
pub struct Sphere {
    /// The sphere's center.
    pub center: Point3,
    /// The sphere's radius.
    pub radius: f64,
    /// The sphere's material.
    pub material: Material,
}

impl Sphere {

    ///
    /// Returns a Sphere with the given arguments.
    ///
    /// # Arguments
    /// * `center` - The sphere's center.
    /// * `radius` - The sphere's radius.
    /// * `material` - The sphere's material.
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }
}