#[macro_use]
extern crate lazy_static;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::material::{Lambertian, Metal};
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use std::sync::Arc;
use vec3::Vec3;

fn main() {
    let mut world: HittableList = HittableList::new();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    let mut cam: Camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    let path = std::path::Path::new("output/book1/image14.png");
    cam.render(&world, path);
}
