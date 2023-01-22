use crate::ray::Ray;
use crate::vector3::Point3;
use crate::vector3::Vector3;

use std::f64::consts::PI;

#[derive(Default, Clone, Copy)]
pub struct Camera {
    origin: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Point3
}

impl Camera {
    pub fn new(

        look_from: Vector3, 
        look_at: Vector3, 
        up: Vector3, 
        vfov_degrees: f64, 
        aspect_ratio: f64

    ) -> Self {

        let theta = vfov_degrees * (PI / 180.0);
        let view_height = (theta / 2.0).tan();
        let view_width = aspect_ratio * view_height;

        let w = (look_from - look_at).unit();
        let u = up.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = view_width * u;
        let vertical = view_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

        Self {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner, 
        }  
    }

    pub fn get_ray(&self, width_ratio: f64, height_ratio: f64) -> Ray {
        let h_offset = width_ratio * self.horizontal;
        let v_offset = height_ratio * self.vertical;
        Ray::new(self.origin, self.lower_left_corner + h_offset + v_offset - self.origin)
    }
}