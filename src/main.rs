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
    let R = (PI / 4.0).cos();
    let material_left = Lambertian::new(Color::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Color::new(1.0, 0.0, 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(-R, -0.0, -1.0),
        R,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(R, -0.0, -1.0),
        R,
        material_right,
    )));
    let mut cam: Camera = Camera::new(16.0 / 9.0, 400, 100, 50, 90.0);
    let path = std::path::Path::new("output/book1/image19.png");
    cam.render(&world, path);
}
