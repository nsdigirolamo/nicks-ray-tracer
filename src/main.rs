mod camera;
mod color;
mod hit;
mod material;
mod ray;
mod sphere;
mod vector3;

use crate::camera::Camera;
use crate::color::*;
use crate::hit::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::Point3;
use crate::vector3::Vector3;

use rand::Rng;

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
    let vfov = 20.0;

    let look_from = Vector3::new(13.0, 2.0, 4.0);
    let look_to = Vector3::new(0.0, 0.5, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    
    let cam = Camera::new(look_from, look_to, up, vfov, aspect_ratio, aperature, dist_to_focus);

    let mut spheres = Vec::new();

    let ground_mat = Material::new(_GREY, None, None);
    let ground = Sphere::new(Point3::new(0.0, -1000.0, -1.0), 1000.0, ground_mat);
    spheres.push(ground);
    
    let radius = 1.0;

    let left_mat = Material::new(_LIGHT_RED, None, None);
    let left = Sphere::new(Point3::new(-3.0, radius, 0.0), radius, left_mat);
    spheres.push(left);

    let center_mat = Material::new(_WHITE, None, Some(1.5));
    let center = Sphere::new(Point3::new(0.0, radius, 0.0), radius, center_mat);
    spheres.push(center);

    let right_mat = Material::new(_LIGHT_BLUE, Some(0.05), None);
    let right = Sphere::new(Point3::new(3.0, radius, 0.0), radius, right_mat);
    spheres.push(right);

    let mut rng = rand::thread_rng();

    for x in -15..10 {
        for z in -10..5 {

            let radius = 0.15;
            let center = Point3::new(x as f64, radius, z as f64);
        
            let r = rng.gen();
            let c = Color::new(rng.gen(), rng.gen(), rng.gen());
            let mut material = Material::new(c, None, None);

            if 0.75 < r {

                material = Material::new(c, Some(rng.gen_range(0.0..1.0)), None);

            } else if 0.5 < r{

                material = Material::new(c, None, Some(rng.gen_range(1.0..2.0)));

            }

            let sphere = Sphere::new(center, radius, material);
            spheres.push(sphere);

        }
    }

    let samples_per_pixel = 1000;
    let max_bounce_depth = 500;

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
                pixel_color += get_ray_color(ray, &spheres, max_bounce_depth);
            }

            pixel_color /= samples_per_pixel as f64;
            pixel_color.gamma_correct();
            println!("{}", pixel_color);
        }
    }
    eprintln!();
}