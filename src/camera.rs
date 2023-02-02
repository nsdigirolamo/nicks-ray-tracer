use crate::ray::Ray;
use crate::vector3::Point3;
use crate::vector3::rand_vector2;
use crate::vector3::Vector3;

use std::f64::consts::PI;

/// Represents the camera used to capture a rendered image.
#[allow(dead_code)]
pub struct Camera {
    /// The position of the camera.
    origin: Point3,
    /// The point representing the horizontal border of the camera's view.
    horizontal: Point3,
    /// The point representing the vertical border of the camera's view.
    vertical: Point3,
    /// The relative lower left corner of the camera's view.
    lower_left_corner: Point3,
    /// The direction the camera is facing.
    view_direction: Vector3,
    /// The vertical vector orthogonal to the view direction.
    view_vertical: Vector3,
    /// The horizontal vector orthogonal to the view direction.
    view_horizontal: Vector3,
    /// The radius of the camera's lens.
    lens_radius: f64,
}

impl Camera {

    ///
    /// Returns a Camera with fields constructed from the given arguments.
    ///
    /// # Arguments
    /// * `look_from` - The camera's location in space.
    /// * `look_at` - The point in space the camera will be looking to.
    /// * `up` - The camera's vertical up direction.
    /// * `vfov_degrees` - The camera's vertical field of view in degrees.
    /// * `aperature` - The camera's aperature diameter.
    /// * `focus_distance` - The distance from the camera where the image will be in focus.
    ///
    pub fn new(

        look_from: Point3, 
        look_at: Point3, 
        up: Vector3, 
        vfov_degrees: f64, 
        aspect_ratio: f64,
        aperature: f64,
        focus_distance: f64,

    ) -> Self {

        let theta = vfov_degrees * (PI / 180.0);
        let view_height = 2.0 * (theta / 2.0).tan();
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

    ///
    /// Returns a Ray through the camera's view at the given width and height.
    ///
    /// # Arguments
    /// `&self` - The camera that the ray is originating from.
    /// `width_ratio` - How far horizontally the ray is applied to the image.
    /// `height_ratio` - How far vertically the ray is applied to the image.
    ///
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