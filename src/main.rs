mod camera;
mod color;
mod hit;
mod hittable;
mod material;
mod ray;
mod scene;
mod texture;
mod vector3;

use crate::camera::Camera;
use crate::color::*;
use crate::hit::Hit;
use crate::hittable::sphere::Sphere;
use crate::material::Material;
use crate::texture::monochrome::Monochrome;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector3::Point3;
use crate::vector3::Vector3;

use rand::Rng;
use std::rc::Rc;

///
/// Returns the Color of a ray. Checks to see if the given ray intersects with
/// any of the given spheres, and will recursively call itself to continue coloring
/// the ray until it hits the depth limit.
///
/// # Arguments
/// `ray` - The ray to be colored.
/// `spheres` - The spheres the ray could possibly intersect with.
/// `depth` - The maximum number of times recursion can occur.
///
fn get_ray_color(ray: Ray, scene: &Scene, depth: i32) -> Color {

    if depth <= 0 { return Color::new(0.0, 0.0, 0.0); }
    
    let min_dist = 0.001;
    let max_dist = 1000.0;

    match scene.get_intersect(ray, min_dist, max_dist) {
        Some(hit) => {
            let ray_color = get_ray_color(hit.scatter(), scene, depth - 1);
            let mut hit_color = hit.color;
            hit_color.r *= ray_color.r;
            hit_color.g *= ray_color.g;
            hit_color.b *= ray_color.b;
            return hit_color;
        },
        None => {
            let s = 0.5 * (1.0 + ray.direction.unit().y);
            let c1 = Color::new(0.5, 0.7, 1.0) * s;
            let c2 = Color::new(1.0, 1.0, 1.0) * (1.0 - s);
            return c1 + c2;
        }
    }
}

fn main() {

    let aspect_ratio = 16.0 / 9.0;
    let image_height = 1080;
    let image_width = (image_height as f64 * aspect_ratio) as i32;
    let vfov = 20.0;

    let look_from = Vector3::new(13.0, 2.0, 4.0);
    let look_to = Vector3::new(0.0, 0.5, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;

    let cam = Camera::new(look_from, look_to, up, vfov, aspect_ratio, aperature, dist_to_focus);

    let ground_texture = Monochrome::new(_GREY);
    let ground_material = Material::new(Box::new(ground_texture), None, None);
    let ground = Sphere::new(Point3::new(0.0, -1000.0, -1.0), 1000.0, ground_material);

    let mut scene = Scene::new(Rc::new(ground));

    let radius = 1.0;

    let left_texture = Monochrome::new(_LIGHT_RED);
    let left_material = Material::new(Box::new(left_texture), None, None);
    let left = Sphere::new(Point3::new(-3.0, radius, 0.0), radius, left_material);
    scene.push(Rc::new(left));

    let center_texture = Monochrome::new(_WHITE);
    let center_material = Material::new(Box::new(center_texture), None, Some(1.5));
    let center = Sphere::new(Point3::new(0.0, radius, 0.0), radius, center_material);
    scene.push(Rc::new(center));

    let right_texture = Monochrome::new(_LIGHT_BLUE);
    let right_material = Material::new(Box::new(right_texture), Some(0.05), None);
    let right = Sphere::new(Point3::new(3.0, radius, 0.0), radius, right_material);
    scene.push(Rc::new(right));

    let mut rng = rand::thread_rng();

    for x in -15..10 {
        for z in -10..5 {

            let radius = 0.15;
            let center = Point3::new(x as f64, radius, z as f64);

            let r = rng.gen();
            let color = Color::new(rng.gen(), rng.gen(), rng.gen());
            let texture = Monochrome::new(color);
            let mut material = Material::new(Box::new(texture), None, None);

            if 0.75 < r {

                material = Material::new(Box::new(texture), Some(rng.gen_range(0.0..1.0)), None);

            } else if 0.5 < r{

                material = Material::new(Box::new(texture), None, Some(rng.gen_range(1.0..2.0)));

            }

            let sphere = Sphere::new(center, radius, material);
            scene.push(Rc::new(sphere));

        }
    }

    let samples_per_pixel = 10;
    let max_bounce_depth = 10;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for row in (0..image_height).rev() {
        eprint!("\r{} scanlines remaining...      ", row);
        for col in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _sample in 0..samples_per_pixel {

                let rand_w: f64 = rng.gen_range(-0.5..0.5);
                let rand_h: f64 = rng.gen_range(-0.5..0.5);
                let w = (col as f64 + rand_w) / image_width as f64;
                let h = (row as f64 + rand_h) / image_height as f64;

                let ray = cam.get_ray(w, h);
                pixel_color += get_ray_color(ray, &scene, max_bounce_depth);
            }

            pixel_color /= samples_per_pixel as f64;
            pixel_color.gamma_correct();
            println!("{}", pixel_color);
        }
    }
    eprintln!();
}