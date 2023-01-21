mod camera;
mod color;
mod hit;
mod ray;
mod sphere;
mod vector3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Point3;

use rand::prelude::*;

fn get_ray_color(ray: Ray, spheres: &Vec<Sphere>, depth: i32) -> Color {

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

    if depth <= 0 {

        Color::new(0.0, 0.0, 0.0)

    } else if found_intersect {

        let scattered = hit.scatter();
        get_ray_color(scattered, spheres, depth - 1) * 0.5

    } else {
        let s = 0.5 * (1.0 + ray.direction.unit().y);
        let c1 = Color::new(0.5, 0.7, 1.0) * s;
        let c2 = Color::new(1.0, 1.0, 1.0) * (1.0 - s);
        c1 + c2
    }
}
    
fn main() {

    let image_height = 1080;
    let aspect_ratio = 16.0 / 9.0;
    let position = Point3::new(0.0, 0.0, 0.0);
    let cam = Camera::new(image_height, aspect_ratio, position, 1.0, 2.0);

    let front = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Point3::new(0.0, -1000.5, -1.0), 1000.0);
    let spheres = vec![front, ground];

    let mut rng = rand::thread_rng();
    let samples_per_pixel = 100;

    println!("P3");
    println!("{} {}", cam.image_width, cam.image_height);
    println!("255");

    for row in (0..cam.image_height).rev() {
        eprint!("\r{} scanlines remaining...      ", row);
        for col in 0..cam.image_width {
            
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _sample in 0..samples_per_pixel {
                
                let rand_w: f64 = rng.gen();
                let rand_h: f64 = rng.gen();

                let w_ratio = (col as f64 + rand_w) / cam.image_width as f64;
                let h_ratio = (row as f64 + rand_h) / cam.image_height as f64;
                let mut direction = cam.bottom_left_corner;
                direction.x += cam.view_width * w_ratio;
                direction.y += cam.view_height * h_ratio;
                
                let ray = Ray::new(cam.position, direction);
                pixel_color += get_ray_color(ray, &spheres, 100);
            }

            pixel_color /= samples_per_pixel as f64;
            pixel_color.gamma_correct();
            println!("{}", pixel_color);
        }
    }
    eprintln!();
}
