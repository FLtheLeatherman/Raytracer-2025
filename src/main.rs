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
use crate::material::{Dielectric, Lambertian, Metal};
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use std::sync::Arc;
use utility::PI;
use vec3::Vec3;

fn main() {
    let mut world: HittableList = HittableList::new();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    let material_left = Dielectric::new(1.50);
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    let material_bubble = Dielectric::new(1.00 / 1.50);
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    let lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut cam: Camera = Camera::new(16.0 / 9.0, 400, 100, 50, 20.0, lookfrom, lookat, vup);
    let path = std::path::Path::new("output/book1/image21.png");
    cam.render(&world, path);
}
