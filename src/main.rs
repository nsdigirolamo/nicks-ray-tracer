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
use crate::ray::Ray;
use crate::scene::Scene;
use crate::scene::construct_book1_final;
use crate::texture::checkered::Checkered;
use crate::texture::monochrome::Monochrome;
use crate::texture::noisy::Noisy;
use crate::vector3::Point3;
use crate::vector3::Vector3;

use noise::Perlin;
use rand::Rng;
use rand::rngs::ThreadRng;
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
fn get_ray_color(ray: Ray, scene: &Scene, depth: i32, rng: &ThreadRng) -> Color {

    if depth <= 0 { return Color::new(0.0, 0.0, 0.0); }
    
    let min_dist = 0.001;
    let max_dist = 1000.0;

    match scene.get_intersect(ray, min_dist, max_dist) {
        Some(hit) => {
            let ray_color = get_ray_color(hit.scatter(rng), scene, depth - 1, rng);
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
    let vfov = 50.0;

    let look_from = Vector3::new(0.0, 1.0, 5.0);
    let look_to = Vector3::new(0.0, 1.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 5.0;
    let aperature = 0.0;

    let cam = Camera::new(look_from, look_to, up, vfov, aspect_ratio, aperature, dist_to_focus);

    let texture1 = Noisy::new(Perlin::new(1), 10.0, 20, true, _LIGHT_BLUE);
    let material1 = Material::new(Box::new(texture1), Some(0.0), None);
    let sphere1 = Sphere::new(Point3::new(-2.1, 1.0, 0.0), 1.0, material1);

    let texture2 = Noisy::new(Perlin::new(2), 50.0, 10, false, _LIGHT_RED);
    let material2 = Material::new(Box::new(texture2), None, None);
    let sphere2 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material2);

    let texture3 = Noisy::new(Perlin::new(3), 10.0, 30, true, _LIGHT_GREEN);
    let material3 = Material::new(Box::new(texture3), None, Some(1.5));
    let sphere3 = Sphere::new(Point3::new(2.1, 1.0, 0.0), 1.0, material3);

    let texture4 = Monochrome::new(_GREY);
    let material4 = Material::new(Box::new(texture4), None, None);
    let sphere4 = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material4);

    let mut scene = Scene::new(Rc::new(sphere1));
    scene.push(Rc::new(sphere2));
    scene.push(Rc::new(sphere3));
    scene.push(Rc::new(sphere4));

    let samples_per_pixel = 200;
    let max_bounce_depth = 100;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut rng = rand::thread_rng();

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
                pixel_color += get_ray_color(ray, &scene, max_bounce_depth, &rng);
            }

            pixel_color /= samples_per_pixel as f64;
            pixel_color.gamma_correct();
            println!("{}", pixel_color);
        }
    }
    eprintln!();
}