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
use crate::utility::{random_double, random_double_range};
use hittable::Hittable;
use hittable_list::HittableList;
use material::Material;
use rand::random;
use sphere::Sphere;
use std::sync::Arc;
use utility::PI;
use vec3::Vec3;

fn main() {
    let mut world: HittableList = HittableList::new();
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center: Vec3 = Vec3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut cam: Camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        lookfrom,
        lookat,
        vup,
        0.6,
        10.0,
    );
    let path = std::path::Path::new("output/book1/image23.png");
    cam.render(&world, path);
}
