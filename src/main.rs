mod vector3;
mod ray;
mod sphere;

use crate::vector3::Point3;
use crate::ray::Ray;
use crate::sphere::Sphere;

fn main() {

    let image_width = 1920;
    let image_height = 1080;
    let aspect_ratio = image_width as f64 / image_height as f64;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let view_distance = 1.0;
    let view_height = 1.0;
    let view_width = view_height * aspect_ratio;

    let mut bottom_left_corner = Point3::new(0.0, 0.0, 0.0);
    bottom_left_corner.x -= view_width / 2.0;
    bottom_left_corner.y -= view_height / 2.0;
    bottom_left_corner.z -= view_distance;

    let sphere = Sphere::new(Point3::new(0.0, 0.0, -3.0), 1.0);
    
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for row in (0..image_height).rev() {
        for col in 0..image_width {
            let w_ratio = col as f64 / image_width as f64;
            let h_ratio = row as f64 / image_height as f64;
            let mut direction = bottom_left_corner;
            direction.x += view_width * w_ratio;
            direction.y += view_height * h_ratio;
            
            let ray = Ray::new(origin, direction);

            match ray.get_intersect(&sphere) {
                Some(t) => println!("255 0 0"),
                None    => println!("0 255 0"),
            }
        }
    }
}
