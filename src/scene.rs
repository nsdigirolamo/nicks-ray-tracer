use crate::color::*;
use crate::hit::Hit;
use crate::hittable::bvh_node::BvhNode;
use crate::hittable::bvh_node::construct_bvh_root;
use crate::hittable::Hittable;
use crate::hittable::sphere::Sphere;
use crate::material::Material;
use crate::texture::monochrome::Monochrome;
use crate::ray::Ray;
use crate::vector3::Point3;

use rand::Rng;
use std::rc::Rc;

/// Represents everything that could be seen in a rendered image.
pub struct Scene {
    /// All hittable objects within a scene.
    hittables: Vec<Rc<dyn Hittable>>,
    /// The root node of this scene's bounding volume hierarchy.
    bvh_root: BvhNode,
}

impl Scene {

    ///
    /// Returns a Scene with the given arguments.
    ///
    /// # Arguments
    /// * `hittables` - All hittable objects within a scene.
    ///
    pub fn new(initial_hittable: Rc<dyn Hittable>) -> Self {

        let mut hittables = vec![initial_hittable];
        Self {
            bvh_root: construct_bvh_root(&mut hittables, 0, 1),
            hittables: hittables,
        }
    }

    ///
    /// Adds a Rc<dyn Hittable> to the Scene's hittables field.
    ///
    /// # Arguments
    /// * `&mut self` - The Scene.
    /// * `hittable` - The hittable to add.
    ///
    pub fn push(&mut self, hittable: Rc<dyn Hittable>) {
        self.hittables.push(hittable);
        let length = self.hittables.len();
        self.bvh_root = construct_bvh_root(&mut self.hittables, 0, length);
    }

    ///
    /// Returns a Hit for the closest intersection between the given ray and all
    /// hittables within a scene. If no such intersection exists, return None.
    ///
    /// # Arguments
    /// * `&self` - The scene the ray exists in.
    /// * `ray` - The ray that will possibly intersect with a hittable in the scene.
    /// * `min_dist` - The minimum distance along the ray to look for an intersection.
    /// * `max_dist` - The maximum distance along the ray to look for an intersection.
    ///
    pub fn get_intersect(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<Hit> {

        self.bvh_root.get_hit(ray, min_dist, max_dist)

    }
}

///
/// Returns the final scene from the first book. Three large spheres surrounded
/// by many smaller spheres.
///
pub fn construct_book1_final() -> Scene {

    /*
        Here are the camera args that were used for this scene, in case it needs
        to be recreated exactly.

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
    */

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

    scene
}