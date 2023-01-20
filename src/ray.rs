use crate::vector3::Vector3;
use crate::vector3::Point3;
use crate::sphere::Sphere;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self {
            origin: origin,
            direction: direction,
        }
    }

    pub fn get_point(&self, t: f64) -> Point3 {
        self.origin + (t * self.direction)
    }

    pub fn get_intersect(&self, sphere: &Sphere) -> Option<f64> {
        
        let d: Vector3 = self.direction;
        let o: Point3 = self.origin;
        let c: Point3 = sphere.center;
        let r: f64 = sphere.radius;

        // Equation: wikipedia.org/wiki/Line-sphere_intersection
        let a: f64 = d.dot(d);
        let b: f64 = 2.0 * (d.dot(o - c));
        let c: f64 = (o - c).dot(o - c) - r.powf(2.0);

        let discriminant: f64 = b.powf(2.0) - 4.0 * a * c;
        
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}