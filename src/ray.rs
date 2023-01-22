use crate::hit::Hit;
use crate::vector3::Vector3;
use crate::vector3::Point3;
use crate::sphere::Sphere;

#[derive(Default, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self {
            origin: origin,
            direction: direction,
        }
    }

    pub fn get_point(&self, distance: f64) -> Point3 {
        self.origin + (distance * self.direction)
    }

    pub fn get_intersect(&self, sphere: &Sphere, min_dist: f64, max_dist: f64) -> Option<Hit> {
        
        let oc = self.origin - sphere.center;
        let a = self.direction.mag_squared();
        let half_b = oc.dot(self.direction);
        let c = oc.mag_squared() - (sphere.radius * sphere.radius);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return None; }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < min_dist || max_dist < root {
            root = (-half_b + sqrtd) / a;
            if root < min_dist || max_dist < root {
                return None;
            }
        }

        Some(Hit::new(*self, *sphere, root))
    }
}