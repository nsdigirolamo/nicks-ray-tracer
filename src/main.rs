mod color;
mod ray;
mod sphere;
mod vector3;

use crate::color::Color;
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

    let front = Sphere::new(Point3::new(0.0, -51.0, 0.0), 50.0);
    
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for row in (0..image_height).rev() {
        eprint!("\r{} scanlines remaining...      ", row);
        for col in 0..image_width {
            let w_ratio = col as f64 / image_width as f64;
            let h_ratio = row as f64 / image_height as f64;
            let mut direction = bottom_left_corner;
            direction.x += view_width * w_ratio;
            direction.y += view_height * h_ratio;
            
            let ray = Ray::new(origin, direction);

            match ray.get_intersect(&front, 0.01, 100.0) {
                Some(t) => {
                    println!("{}", Color::new(1.0, 0.0, 0.0));
                }
                None => {
                    let s = 0.5 * (1.0 + ray.direction.unit().y);
                    let c1 = Color::new(0.5, 0.7, 1.0) * s;
                    let c2 = Color::new(1.0, 1.0, 1.0) * (1.0 - s);
                    println!("{}", c1 + c2);
                },
            }
        }
    }
    eprintln!();
}
