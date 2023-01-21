use crate::vector3::Point3;

#[derive(Default, Clone, Copy)]
pub struct Camera {
    pub image_width: i32,
    pub image_height: i32,
    pub aspect_ratio: f64,
    pub position: Point3,
    pub focal_length: f64,
    pub view_width: f64,
    pub view_height: f64,
    pub bottom_left_corner: Point3,
}

impl Camera {
    pub fn new(image_height: i32, aspect_ratio: f64, position: Point3, focal_length: f64, view_height: f64) -> Self {
        let mut c = Self {
            image_width: (image_height as f64 * aspect_ratio) as i32,
            image_height: image_height,
            aspect_ratio: aspect_ratio,
            position: position,
            focal_length: focal_length,
            view_width: view_height * aspect_ratio,
            view_height: view_height,
            bottom_left_corner: position,
        };
        c.bottom_left_corner.x -= c.view_width / 2.0;
        c.bottom_left_corner.y -= c.view_height / 2.0;
        c.bottom_left_corner.z -= c.focal_length;
        c
    }
}