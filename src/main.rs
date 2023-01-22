mod camera;
mod color;
mod hit;
mod material;
mod ray;
mod sphere;
mod vector3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hit::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Point3;
use crate::vector3::Vector3;

use rand::Rng;

fn get_ray_color(ray: Ray, spheres: &Vec<Sphere>, depth: i32) -> Color {

    let min_distance = 0.001;
    let max_distance = 1000.0;

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

        let ray_color = get_ray_color(hit.scatter(), spheres, depth - 1);
        let mut mat_color = hit.sphere.material.albedo;
        mat_color.r *= ray_color.r;
        mat_color.g *= ray_color.g;
        mat_color.b *= ray_color.b;
        mat_color

    } else {

        let s = 0.5 * (1.0 + ray.direction.unit().y);
        let c1 = Color::new(0.5, 0.7, 1.0) * s;
        let c2 = Color::new(1.0, 1.0, 1.0) * (1.0 - s);
        c1 + c2

    }
}
    
fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_height = 1080;
    let image_width = (image_height as f64 * aspect_ratio) as i32;
    let vfov = 60.0;

    let look_from = Vector3::new(-2.0, 2.0, 1.0);
    let look_to = Vector3::new(0.0, 0.0, -1.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    
    let cam = Camera::new(look_from, look_to, up, vfov, aspect_ratio);

    let ground_mat = Material::new(Color::new(0.8, 0.8, 0.0), None, None);
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground_mat);

    let center_mat = Material::new(Color::new(0.8, 0.8, 0.8), None, None);
    let center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, center_mat);

    let left_mat = Material::new(Color::new(1.0, 1.0, 1.0), Some(0.3), Some(1.5));
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, left_mat);

    let right_mat = Material::new(Color::new(0.8, 0.6, 0.2), Some(1.0), None);
    let right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, right_mat);
    
    let spheres = vec![ground, center, left, right];

    let mut rng = rand::thread_rng();
    let samples_per_pixel = 100;
    let max_bounce_depth = 50;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for row in (0..image_height).rev() {
        eprint!("\r{} scanlines remaining...      ", row);
        for col in 0..image_width {
            
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _sample in 0..samples_per_pixel {
                
                let rand_w: f64 = rng.gen_range(-1.0..1.0);
                let rand_h: f64 = rng.gen_range(-1.0..1.0);
                let w = (col as f64 + rand_w) / image_width as f64;
                let h = (row as f64 + rand_h) / image_height as f64;
                
                let ray = cam.get_ray(w, h);
                pixel_color += get_ray_color(ray, &spheres, max_bounce_depth);
            }

            pixel_color /= samples_per_pixel as f64;
            pixel_color.gamma_correct();
            println!("{}", pixel_color);
        }
    }
    eprintln!();
}
