mod color;
mod hit;
mod ray;
mod sphere;
mod vector3;

use crate::color::Color;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Point3;
use crate::vector3::Vector3;

fn get_ray_color(ray: Ray, spheres: &Vec<Sphere>) -> Color {

    let min_distance = 0.01;
    let max_distance = 100.0;

    let mut found_intersect = false;
    let mut min_intersect_distance = max_distance;
    let mut hit: Hit = Default::default();

    for sphere in spheres {
        match ray.get_intersect(&sphere, min_distance, max_distance) {
            Some(h) => {
                if h.distance < min_intersect_distance {
                    min_intersect_distance = h.distance;
                    found_intersect = true;
                    hit = h;
                }
            },
            None => (),
        }
    }

    if found_intersect {
        let normal = 0.5 * (Vector3::new(1.0, 1.0, 1.0) + hit.normal); // make sure there's no negative numbers.
        Color::new(normal.x, normal.y, normal.z)
    } else {
        let s = 0.5 * (1.0 + ray.direction.unit().y);
        let c1 = Color::new(0.5, 0.7, 1.0) * s;
        let c2 = Color::new(1.0, 1.0, 1.0) * (1.0 - s);
        c1 + c2
    }
}
    
    

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

    let front = Sphere::new(Point3::new(0.0, 0.0, -5.0), 1.0);
    let ground = Sphere::new(Point3::new(0.0, -1001.0, 0.0), 1000.0);
    let spheres = vec![front, ground];

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
            println!("{}", get_ray_color(ray, &spheres));
        }
    }
    eprintln!();
}
