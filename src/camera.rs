use crate::ray::Ray;
use crate::vector3::Point3;
use crate::vector3::rand_vector2;
use crate::vector3::Vector3;

use std::f64::consts::PI;

#[derive(Default, Clone, Copy)]
pub struct Camera {
    origin: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Point3,
    view_direction: Vector3,
    view_vertical: Vector3,
    view_horizontal: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(

        look_from: Vector3, 
        look_at: Vector3, 
        up: Vector3, 
        vfov_degrees: f64, 
        aspect_ratio: f64,
        aperature: f64,
        focus_distance: f64,

    ) -> Self {

        let theta = vfov_degrees * (PI / 180.0);
        let view_height = (theta / 2.0).tan();
        let view_width = aspect_ratio * view_height;

        let view_direction = (look_from - look_at).unit();
        let view_vertical = up.cross(view_direction).unit();
        let view_horizontal = view_direction.cross(view_vertical);

        let origin = look_from;
        let horizontal = focus_distance * view_width * view_vertical;
        let vertical = focus_distance * view_height * view_horizontal;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_distance * view_direction;

        Self {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            view_direction: view_direction,
            view_vertical: view_vertical,
            view_horizontal: view_horizontal,
            lens_radius: aperature / 2.0,
        }  
    }

    pub fn get_ray(&self, width_ratio: f64, height_ratio: f64) -> Ray {

        let rd = self.lens_radius * rand_vector2();
        let origin_offset = self.view_horizontal * rd.x + self.view_vertical * rd.y;
        let origin = self.origin + origin_offset;

        let h_offset = width_ratio * self.horizontal;
        let v_offset = height_ratio * self.vertical;
        let direction = self.lower_left_corner + h_offset + v_offset - origin;

        Ray::new(origin, direction)
    }
}