#[macro_use]
extern crate lazy_static;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utility;
mod vec3;

use crate::camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let mut cam: Camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    let path = std::path::Path::new("output/book1/image8.png");
    cam.render(&world, path);
}
