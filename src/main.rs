#[macro_use]
extern crate lazy_static;

mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod obj;
mod onb;
mod pdf;
mod perlin;
mod quad;
mod ray;
mod rtw_stb_image;
mod sphere;
mod texture;
mod triangle;
mod utility;
mod vec3;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{RotateY, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, MappedMaterial, Metal};
use crate::obj::load_model;
use crate::quad::{Quad, make_box};
use crate::texture::{ImageTexture, NoiseTexture, SolidColor, UV};
use crate::triangle::Triangle;
use crate::utility::{degrees_to_radians, random_double, random_double_range};
use hittable::Hittable;
use hittable_list::HittableList;
use image::error::LimitErrorKind::DimensionError;
use sphere::Sphere;
use std::sync::Arc;
use std::time::Instant;
use tobj::LoadError::MaterialParseError;
use vec3::Vec3;

// fn bouncing_spheres() {
//     let mut world: HittableList = HittableList::new();
//     let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(ground_material),
//     )));
//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat = random_double();
//             let center: Vec3 = Vec3::new(
//                 a as f64 + 0.9 * random_double(),
//                 0.2,
//                 b as f64 + 0.9 * random_double(),
//             );
//             if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
//                 if choose_mat < 0.8 {
//                     let albedo = Color::random() * Color::random();
//                     let sphere_material = Lambertian::new(albedo);
//                     let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
//                     world.add(Arc::new(Sphere::new_dyn(
//                         center,
//                         center2,
//                         0.2,
//                         Arc::new(sphere_material),
//                     )));
//                 } else if choose_mat < 0.95 {
//                     let albedo = Color::random_range(0.5, 1.0);
//                     let fuzz = random_double_range(0.0, 0.5);
//                     let sphere_material = Metal::new(albedo, fuzz);
//                     world.add(Arc::new(Sphere::new(
//                         center,
//                         0.2,
//                         Arc::new(sphere_material),
//                     )));
//                 } else {
//                     let sphere_material = Dielectric::new(1.5);
//                     world.add(Arc::new(Sphere::new(
//                         center,
//                         0.2,
//                         Arc::new(sphere_material),
//                     )));
//                 }
//             }
//         }
//     }
//     let material1 = Dielectric::new(1.5);
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(0.0, 1.0, 0.0),
//         1.0,
//         Arc::new(material1),
//     )));
//     let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(-4.0, 1.0, 0.0),
//         1.0,
//         Arc::new(material2),
//     )));
//     let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(4.0, 1.0, 0.0),
//         1.0,
//         Arc::new(material3),
//     )));
//     let world = bvh::BvhNode::new_list(&mut world);
//     let lookfrom = Vec3::new(13.0, 2.0, 3.0);
//     let lookat = Vec3::new(0.0, 0.0, 0.0);
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let mut cam: Camera = Camera::new(
//         16.0 / 9.0,
//         400,
//         100,
//         50,
//         20.0,
//         lookfrom,
//         lookat,
//         vup,
//         0.6,
//         10.0,
//         Color::new(0.70, 0.80, 1.00),
//     );
//     let path = std::path::Path::new("output/book2/image1.png");
//     cam.initialize();
//     cam.render(&world, path);
// }
//
// fn checkered_spheres() {
//     let mut world = HittableList::new();
//     let checker =
//         CheckerTexture::new_color(0.32, &Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9));
//     let checker_copy =
//         CheckerTexture::new_color(0.32, &Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9));
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(0.0, -10.0, 0.0),
//         10.0,
//         Arc::new(Lambertian::new_tex(Arc::new(checker))),
//     )));
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(0.0, 10.0, 0.0),
//         10.0,
//         Arc::new(Lambertian::new_tex(Arc::new(checker_copy))),
//     )));
//     let lookfrom = Vec3::new(13.0, 2.0, 3.0);
//     let lookat = Vec3::new(0.0, 0.0, 0.0);
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let mut cam: Camera = Camera::new(
//         16.0 / 9.0,
//         400,
//         100,
//         50,
//         20.0,
//         lookfrom,
//         lookat,
//         vup,
//         0.0,
//         10.0,
//         Color::new(0.70, 0.80, 1.00),
//     );
//     let path = std::path::Path::new("output/book2/image3.png");
//     cam.initialize();
//     cam.render(&world, path);
// }
// fn earth() {
//     let mut world = HittableList::new();
//     let earth_texture = ImageTexture::new("earthmap.jpg");
//     let earth_suface = Lambertian::new_tex(Arc::new(earth_texture));
//     let globe = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, Arc::new(earth_suface));
//     world.add(Arc::new(globe));
//     let lookfrom = Vec3::new(0.0, 0.0, 12.0);
//     let lookat = Vec3::new(0.0, 0.0, 0.0);
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let mut cam: Camera = Camera::new(
//         16.0 / 9.0,
//         400,
//         100,
//         50,
//         20.0,
//         lookfrom,
//         lookat,
//         vup,
//         0.0,
//         10.0,
//         Color::new(0.70, 0.80, 1.00),
//     );
//     let path = std::path::Path::new("output/book2/image5.png");
//     cam.initialize();
//     cam.render(&world, path);
// }
// fn perlin_spheres() {
//     let mut world = HittableList::new();
//     let pertext = NoiseTexture::new(4.0);
//     let pertext_clone = NoiseTexture::new(4.0);
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(Lambertian::new_tex(Arc::new(pertext))),
//     )));
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(0.0, 2.0, 0.0),
//         2.0,
//         Arc::new(Lambertian::new_tex(Arc::new(pertext_clone))),
//     )));
//     let lookfrom = Vec3::new(13.0, 2.0, 3.0);
//     let lookat = Vec3::new(0.0, 0.0, 0.0);
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let mut cam: Camera = Camera::new(
//         16.0 / 9.0,
//         400,
//         100,
//         50,
//         20.0,
//         lookfrom,
//         lookat,
//         vup,
//         0.0,
//         10.0,
//         Color::new(0.70, 0.80, 1.00),
//     );
//     let path = std::path::Path::new("output/book2/image15.png");
//     cam.initialize();
//     cam.render(&world, path);
// }
// fn quads() {
//     let mut world = HittableList::new();
//     let left_red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
//     let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
//     let right_blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
//     let upper_orange = Lambertian::new(Color::new(1.0, 0.5, 0.0));
//     let lower_teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(-3.0, -2.0, 5.0),
//         &Vec3::new(0.0, 0.0, -4.0),
//         &Vec3::new(0.0, 4.0, 0.0),
//         Arc::new(left_red),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(-2.0, -2.0, 0.0),
//         &Vec3::new(4.0, 0.0, 0.0),
//         &Vec3::new(0.0, 4.0, 0.0),
//         Arc::new(back_green),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(3.0, -2.0, 1.0),
//         &Vec3::new(0.0, 0.0, 4.0),
//         &Vec3::new(0.0, 4.0, 0.0),
//         Arc::new(right_blue),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(-2.0, 3.0, 1.0),
//         &Vec3::new(4.0, 0.0, 0.0),
//         &Vec3::new(0.0, 0.0, 4.0),
//         Arc::new(upper_orange),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(-2.0, -3.0, 5.0),
//         &Vec3::new(4.0, 0.0, 0.0),
//         &Vec3::new(0.0, 0.0, -4.0),
//         Arc::new(lower_teal),
//     )));
//     let lookfrom = Vec3::new(0.0, 0.0, 9.0);
//     let lookat = Vec3::new(0.0, 0.0, 0.0);
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let mut cam: Camera = Camera::new(
//         1.0,
//         400,
//         100,
//         50,
//         80.0,
//         lookfrom,
//         lookat,
//         vup,
//         0.0,
//         10.0,
//         Color::new(0.70, 0.80, 1.00),
//     );
//     let path = std::path::Path::new("output/book2/image16.png");
//     cam.initialize();
//     cam.render(&world, path);
// }
// fn simple_lights() {
//     let mut world = HittableList::new();
//     let pertext = NoiseTexture::new(4.0);
//     let pertext_clone = NoiseTexture::new(4.0);
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(Lambertian::new_tex(Arc::new(pertext))),
//     )));
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(0.0, 2.0, 0.0),
//         2.0,
//         Arc::new(Lambertian::new_tex(Arc::new(pertext_clone))),
//     )));
//     let difflight = DiffuseLight::new(&Color::new(4.0, 4.0, 4.0));
//     let difflight_clone = DiffuseLight::new(&Color::new(4.0, 4.0, 4.0));
//     world.add(Arc::new(Sphere::new(
//         Vec3::new(0.0, 7.0, 0.0),
//         2.0,
//         Arc::new(difflight),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(3.0, 1.0, -2.0),
//         &Vec3::new(2.0, 0.0, 0.0),
//         &Vec3::new(0.0, 2.0, 0.0),
//         Arc::new(difflight_clone),
//     )));
//     let lookfrom = Vec3::new(26.0, 3.0, 6.0);
//     let lookat = Vec3::new(0.0, 2.0, 0.0);
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let mut cam: Camera = Camera::new(
//         16.0 / 9.0,
//         400,
//         100,
//         50,
//         20.0,
//         lookfrom,
//         lookat,
//         vup,
//         0.0,
//         10.0,
//         Color::new(0.0, 0.0, 0.0),
//     );
//     let path = std::path::Path::new("output/book2/image18.png");
//     cam.initialize();
//     cam.render(&world, path);
// }
// fn cornell_box() {
//     let mut world = HittableList::new();
//     let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
//     let light = DiffuseLight::new(&Color::new(15.0, 15.0, 15.0));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(555.0, 0.0, 0.0),
//         &Vec3::new(0.0, 555.0, 0.0),
//         &Vec3::new(0.0, 0.0, 555.0),
//         Arc::new(green),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(0.0, 0.0, 0.0),
//         &Vec3::new(0.0, 555.0, 0.0),
//         &Vec3::new(0.0, 0.0, 555.0),
//         Arc::new(red),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(343.0, 554.0, 332.0),
//         &Vec3::new(-130.0, 0.0, 0.0),
//         &Vec3::new(0.0, 0.0, -105.0),
//         Arc::new(light),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(0.0, 0.0, 0.0),
//         &Vec3::new(555.0, 0.0, 0.0),
//         &Vec3::new(0.0, 0.0, 555.0),
//         Arc::new(white),
//     )));
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(555.0, 555.0, 555.0),
//         &Vec3::new(-555.0, 0.0, 0.0),
//         &Vec3::new(0.0, 0.0, -555.0),
//         Arc::new(white),
//     )));
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(0.0, 0.0, 555.0),
//         &Vec3::new(555.0, 0.0, 0.0),
//         &Vec3::new(0.0, 555.0, 0.0),
//         Arc::new(white),
//     )));
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     let box1 = make_box(
//         &Vec3::new(0.0, 0.0, 0.0),
//         &Vec3::new(165.0, 330.0, 165.0),
//         Arc::new(white),
//     );
//     let box1 = Arc::new(RotateY::new(box1, 15.0));
//     let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
//     world.add(box1);
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     let box2 = make_box(
//         &Vec3::new(0.0, 0.0, 0.0),
//         &Vec3::new(165.0, 165.0, 165.0),
//         Arc::new(white),
//     );
//     let box2 = Arc::new(RotateY::new(box2, -18.0));
//     let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
//     world.add(box2);
//     let lookfrom = Vec3::new(278.0, 278.0, -800.0);
//     let lookat = Vec3::new(278.0, 278.0, 0.0);
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let mut cam: Camera = Camera::new(
//         1.0,
//         600,
//         200,
//         50,
//         40.0,
//         lookfrom,
//         lookat,
//         vup,
//         0.0,
//         10.0,
//         Color::new(0.0, 0.0, 0.0),
//     );
//     let path = std::path::Path::new("output/book2/image21.png");
//     cam.initialize();
//     cam.render(&world, path);
// }
// fn cornell_smoke() {
//     let mut world = HittableList::new();
//     let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
//     let light = DiffuseLight::new(&Color::new(7.0, 7.0, 7.0));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(555.0, 0.0, 0.0),
//         &Vec3::new(0.0, 555.0, 0.0),
//         &Vec3::new(0.0, 0.0, 555.0),
//         Arc::new(green),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(0.0, 0.0, 0.0),
//         &Vec3::new(0.0, 555.0, 0.0),
//         &Vec3::new(0.0, 0.0, 555.0),
//         Arc::new(red),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(113.0, 554.0, 127.0),
//         &Vec3::new(330.0, 0.0, 0.0),
//         &Vec3::new(0.0, 0.0, 305.0),
//         Arc::new(light),
//     )));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(0.0, 0.0, 0.0),
//         &Vec3::new(555.0, 0.0, 0.0),
//         &Vec3::new(0.0, 0.0, 555.0),
//         Arc::new(white),
//     )));
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(0.0, 555.0, 0.0),
//         &Vec3::new(555.0, 0.0, 0.0),
//         &Vec3::new(0.0, 0.0, 555.0),
//         Arc::new(white),
//     )));
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     world.add(Arc::new(Quad::new(
//         &Vec3::new(0.0, 0.0, 555.0),
//         &Vec3::new(555.0, 0.0, 0.0),
//         &Vec3::new(0.0, 555.0, 0.0),
//         Arc::new(white),
//     )));
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     let box1 = make_box(
//         &Vec3::new(0.0, 0.0, 0.0),
//         &Vec3::new(165.0, 330.0, 165.0),
//         Arc::new(white),
//     );
//     let box1 = Arc::new(RotateY::new(box1, 15.0));
//     let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
//     // world.add(box1.clone());
//     let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
//     let box2 = make_box(
//         &Vec3::new(0.0, 0.0, 0.0),
//         &Vec3::new(165.0, 165.0, 165.0),
//         Arc::new(white),
//     );
//     let box2 = Arc::new(RotateY::new(box2, -18.0));
//     let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
//     // world.add(box2.clone());
//     world.add(Arc::new(ConstantMedium::new_color(
//         box1,
//         0.01,
//         &Color::new(0.0, 0.0, 0.0),
//     )));
//     world.add(Arc::new(ConstantMedium::new_color(
//         box2,
//         0.01,
//         &Color::new(1.0, 1.0, 1.0),
//     )));
//     let lookfrom = Vec3::new(278.0, 278.0, -800.0);
//     let lookat = Vec3::new(278.0, 278.0, 0.0);
//     let vup = Vec3::new(0.0, 1.0, 0.0);
//     let mut cam: Camera = Camera::new(
//         1.0,
//         600,
//         200,
//         50,
//         40.0,
//         lookfrom,
//         lookat,
//         vup,
//         0.0,
//         10.0,
//         Color::new(0.0, 0.0, 0.0),
//     );
//     let path = std::path::Path::new("output/book2/image22.png");
//     cam.initialize();
//     cam.render(&world, path);
// }
fn book2_final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_size = 20;
    for i in 0..boxes_per_size {
        for j in 0..boxes_per_size {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(make_box(
                &Vec3::new(x0, y0, z0),
                &Vec3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    let mut world = HittableList::new();
    world.add(Arc::new(BvhNode::new_list(&mut boxes1)));
    let light = Arc::new(DiffuseLight::new(&Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        &Vec3::new(123.0, 554.0, 147.0),
        &Vec3::new(300.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 265.0),
        light,
    )));
    let mut lights = HittableList::new();
    let empty_material = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
    lights.add(Arc::new(Quad::new(
        &Vec3::new(123.0, 554.0, 147.0),
        &Vec3::new(300.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 265.0),
        empty_material,
    )));
    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_dyn(
        center1,
        center2,
        50.0,
        sphere_material,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));
    let boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.0001,
        &Color::new(1.0, 1.0, 1.0),
    )));
    let emat = Arc::new(Lambertian::new_tex(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_tex(pertext)),
    )));
    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::new_list(&mut boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));
    let lookfrom = Vec3::new(478.0, 278.0, -600.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut cam: Camera = Camera::new(
        1.0,
        image_width,
        samples_per_pixel,
        max_depth as i32,
        40.0,
        lookfrom,
        lookat,
        vup,
        0.0,
        10.0,
        Color::new(0.0, 0.0, 0.0),
    );
    let path = std::path::Path::new("output/book2/image23_with_pdf.png");
    cam.initialize();
    let world_arc: Arc<dyn Hittable> = Arc::new(world);
    let lights_arc: Arc<dyn Hittable> = Arc::new(lights);
    cam.render(&world_arc, &lights_arc, path);
}
fn book3_cornell_box() {
    let mut world = HittableList::new();
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(&Color::new(15.0, 15.0, 15.0));
    world.add(Arc::new(Quad::new(
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        &Vec3::new(0.0, 555.0, 0.0),
        Arc::new(green),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(0.0, 0.0, 555.0),
        &Vec3::new(0.0, 0.0, -555.0),
        &Vec3::new(0.0, 555.0, 0.0),
        Arc::new(red),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(213.0, 554.0, 227.0),
        &Vec3::new(130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 105.0),
        Arc::new(light),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        Arc::new(white),
    )));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    world.add(Arc::new(Quad::new(
        &Vec3::new(0.0, 0.0, 555.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -555.0),
        Arc::new(white),
    )));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    world.add(Arc::new(Quad::new(
        &Vec3::new(555.0, 0.0, 555.0),
        &Vec3::new(-555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        Arc::new(white),
    )));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    // let aluminum = Metal::new(Color::new(0.8, 0.85, 0.88), 0.0);
    let box1 = make_box(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 330.0, 165.0),
        Arc::new(white),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);
    // let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    // let box2 = make_box(
    //     &Vec3::new(0.0, 0.0, 0.0),
    //     &Vec3::new(165.0, 165.0, 165.0),
    //     Arc::new(white),
    // );
    // let box2 = Arc::new(RotateY::new(box2, -18.0));
    // let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    // world.add(box2);
    let glass = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        Arc::new(glass),
    )));
    let empty_material = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        &Vec3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        empty_material.clone(),
    )));
    lights.add(Arc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        empty_material,
    )));
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut cam: Camera = Camera::new(
        1.0,
        600,
        1000,
        50,
        40.0,
        lookfrom,
        lookat,
        vup,
        0.0,
        10.0,
        Color::new(0.0, 0.0, 0.0),
    );
    let path = std::path::Path::new("output/book3/image15.png");
    cam.initialize();
    let world_arc: Arc<dyn Hittable> = Arc::new(world);
    let lights_arc: Arc<dyn Hittable> = Arc::new(lights);
    cam.render(&world_arc, &lights_arc, path);
}
fn obj_test() {
    let mut world = load_model("cornell_box.obj", 1.0);
    let light = DiffuseLight::new(&Color::new(15.0, 15.0, 15.0));
    world.add(Arc::new(Quad::new(
        &Vec3::new(213.0, 548.0, 227.0),
        &Vec3::new(130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 105.0),
        Arc::new(light),
    )));
    let mut lights = HittableList::new();
    let light = DiffuseLight::new(&Color::new(7.0, 7.0, 7.0));
    lights.add(Arc::new(Quad::new(
        &Vec3::new(213.0, 548.0, 227.0),
        &Vec3::new(130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 105.0),
        Arc::new(light),
    )));
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut cam: Camera = Camera::new(
        1.0,
        600,
        10,
        50,
        40.0,
        lookfrom,
        lookat,
        vup,
        0.0,
        10.0,
        Color::new(0.0, 0.0, 0.0),
    );
    let path = std::path::Path::new("output/test_obj.png");
    cam.initialize();
    let world_arc: Arc<dyn Hittable> = Arc::new(world);
    let lights_arc: Arc<dyn Hittable> = Arc::new(lights);
    cam.render(&world_arc, &lights_arc, path);
}
fn normal_mapping_test() {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let blue = Lambertian::new(Color::new(0.05, 0.05, 0.45));
    let light = DiffuseLight::new(&Color::new(15.0, 15.0, 15.0));
    world.add(Arc::new(Quad::new(
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        Arc::new(green),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        Arc::new(red),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        Arc::new(light),
    )));
    let light = DiffuseLight::new(&Color::new(15.0, 15.0, 15.0));
    lights.add(Arc::new(Quad::new(
        &Vec3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        Arc::new(light),
    )));
    let mut white_image1 = MappedMaterial::new(Arc::new(white));
    white_image1.set_normal("normal_mapping1.jpg");
    world.add(Arc::new(Quad::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        Arc::new(white_image1),
    )));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    world.add(Arc::new(Quad::new(
        &Vec3::new(555.0, 555.0, 555.0),
        &Vec3::new(-555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -555.0),
        Arc::new(white),
    )));
    let mut blue_image2 = MappedMaterial::new(Arc::new(blue));
    blue_image2.set_normal("normal_mapping2.jpg");
    world.add(Arc::new(Quad::new(
        &Vec3::new(0.0, 0.0, 555.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        Arc::new(blue_image2),
    )));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let box1 = make_box(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 330.0, 165.0),
        Arc::new(white),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let box2 = make_box(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 165.0, 165.0),
        Arc::new(white),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut cam: Camera = Camera::new(
        1.0,
        600,
        100,
        50,
        40.0,
        lookfrom,
        lookat,
        vup,
        0.0,
        10.0,
        Color::new(0.0, 0.0, 0.0),
    );
    let path = std::path::Path::new("output/test_normal_mapping.png");
    cam.initialize();
    let world_arc: Arc<dyn Hittable> = Arc::new(world);
    let lights_arc: Arc<dyn Hittable> = Arc::new(lights);
    cam.render(&world_arc, &lights_arc, path);
}
fn all_mapping_test() {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let blue = Lambertian::new(Color::new(0.05, 0.05, 0.45));
    let light = DiffuseLight::new(&Color::new(15.0, 15.0, 15.0));
    world.add(Arc::new(Quad::new(
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        Arc::new(green),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        Arc::new(red),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        Arc::new(light),
    )));
    let light = DiffuseLight::new(&Color::new(3.0, 3.0, 3.0));
    lights.add(Arc::new(Quad::new(
        &Vec3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        Arc::new(light),
    )));
    let mut white_image1 = MappedMaterial::new(Arc::new(white));
    white_image1.set_normal("normal_mapping1.jpg");
    world.add(Arc::new(Quad::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        Arc::new(white_image1),
    )));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    world.add(Arc::new(Quad::new(
        &Vec3::new(555.0, 555.0, 555.0),
        &Vec3::new(-555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -555.0),
        Arc::new(white),
    )));
    let mut blue_image = MappedMaterial::new(Arc::new(blue));
    blue_image.set_alpha("alpha_mapping1.png");
    world.add(Arc::new(Quad::new(
        &Vec3::new(555.0, 0.0, 554.9),
        &Vec3::new(-555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        Arc::new(blue_image),
    )));
    let genshin = Lambertian::new_tex(Arc::new(ImageTexture::new("genshin.jpg")));
    world.add(Arc::new(Quad::new(
        &Vec3::new(555.0, 0.0, 555.0),
        &Vec3::new(-555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        Arc::new(genshin),
    )));
    let tmp = Metal::new(Color::new(1.0, 1.0, 1.0), 0.5);
    let mut light2 = MappedMaterial::new(Arc::new(tmp));
    light2.set_light("light_mapping2.png", 2.0);
    world.add(Arc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        Arc::new(light2),
    )));
    // let mut world = bvh::BvhNode::new_list(&mut world);
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut cam: Camera = Camera::new(
        1.0,
        600,
        1000,
        50,
        40.0,
        lookfrom,
        lookat,
        vup,
        0.0,
        10.0,
        Color::new(0.0, 0.0, 0.0),
    );
    let path = std::path::Path::new("output/test_mapping_all.png");
    cam.initialize();
    let world_arc: Arc<dyn Hittable> = Arc::new(world);
    let lights_arc: Arc<dyn Hittable> = Arc::new(lights);
    cam.render(&world_arc, &lights_arc, path);
}
fn final_scene() {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    // 灯
    let light = Arc::new(DiffuseLight::new(&Color::new(18.0, 18.0, 18.0)));
    world.add(Arc::new(Quad::new(
        &Vec3::new(163.0, 654.0, 177.0),
        &Vec3::new(230.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 205.0),
        light.clone(),
    )));
    let empty_material = Arc::new(Lambertian::new(Color::default()));
    lights.add(Arc::new(Quad::new(
        &Vec3::new(163.0, 654.0, 177.0),
        &Vec3::new(230.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 205.0),
        empty_material.clone(),
    )));

    // 地板
    let white = Arc::new(Lambertian::new(Color::new(
        188.0 / 255.99,
        122.0 / 255.99,
        122.0 / 255.99,
    )));
    let mut _brown = MappedMaterial::new(white.clone());
    _brown.set_normal("normal_mapping3.jpg");
    let _brown = Arc::new(_brown);
    world.add(Arc::new(Quad::new(
        &Vec3::new(-100.0, 0.0, 100.0),
        &Vec3::new(755.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 755.0),
        _brown.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(-100.0 - 755.0, 0.0, 100.0),
        &Vec3::new(755.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 755.0),
        _brown.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(-100.0 + 755.0, 0.0, 100.0),
        &Vec3::new(755.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 755.0),
        _brown.clone(),
    )));

    // 墙
    let white = Arc::new(Lambertian::new(Color::new(0.4, 0.02, 0.07)));
    let mut _brown = MappedMaterial::new(white.clone());
    _brown.set_normal("normal_mapping1.jpg");
    let _brown = Arc::new(_brown);
    world.add(Arc::new(Quad::new(
        &Vec3::new(-100.0, 0.0, 100.0 + 755.0),
        &Vec3::new(755.0, 0.0, 0.0),
        &Vec3::new(0.0, 755.0, 0.0),
        _brown.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(-100.0 - 755.0, 0.0, 100.0 + 755.0),
        &Vec3::new(755.0, 0.0, 0.0),
        &Vec3::new(0.0, 755.0, 0.0),
        _brown.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(-100.0 + 755.0, 0.0, 100.0 + 755.0),
        &Vec3::new(755.0, 0.0, 0.0),
        &Vec3::new(0.0, 755.0, 0.0),
        _brown.clone(),
    )));

    // 墙上线索
    let clue1 = ImageTexture::new("1.png");
    let clue1 = Lambertian::new_tex(Arc::new(clue1));
    let clue3 = ImageTexture::new("3.png");
    let clue3 = Lambertian::new_tex(Arc::new(clue3));
    let clue6 = ImageTexture::new("6.png");
    let clue6 = Lambertian::new_tex(Arc::new(clue6));
    world.add(Arc::new(Quad::new(
        &Vec3::new(-275.0, 150.0, 100.0 + 754.9),
        &Vec3::new(-100.0, 0.0, 0.0),
        &Vec3::new(0.0, 150.0, 0.0),
        Arc::new(clue1),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(-305.0, 325.0, 100.0 + 754.9),
        &Vec3::new(-100.0, 0.0, 0.0),
        &Vec3::new(0.0, 150.0, 0.0),
        Arc::new(clue3),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(-355.0, 200.0, 100.0 + 754.9),
        &Vec3::new(-100.0, 0.0, 0.0),
        &Vec3::new(0.0, 150.0, 0.0),
        Arc::new(clue6),
    )));

    // 水坑
    let tmp = Arc::new(Metal::new(Color::new(0.73, 0.73, 0.73), 0.01));
    let mut tmp = MappedMaterial::new(tmp);
    tmp.set_alpha("alpha_mapping2.png");
    let tmp = Arc::new(tmp);
    world.add(Arc::new(Quad::new(
        &Vec3::new(-100.0, 0.0, 100.0),
        &Vec3::new(755.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 755.0),
        tmp.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(-100.0 - 755.0, 0.0, 100.0),
        &Vec3::new(755.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 755.0),
        tmp.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Vec3::new(-100.0 + 755.0, 0.0, 100.0),
        &Vec3::new(755.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 755.0),
        tmp.clone(),
    )));

    // Raytracer 标识
    let white = Arc::new(Dielectric::new(1.0));
    let mut _light = MappedMaterial::new(white.clone());
    _light.set_light("light_mapping3.png", 1.5);
    let dengpai = Quad::new(
        &Vec3::new(
            80.0 + 159.37,
            50.0,
            465.0 + 205.0 * degrees_to_radians(18.0).sin() - 9.75,
        ),
        &Vec3::new(
            205.0 * -degrees_to_radians(18.0).cos(),
            0.0,
            205.0 * -degrees_to_radians(18.0).sin(),
        ),
        &Vec3::new(0.0, 70.0, 0.0),
        Arc::new(_light),
    );
    world.add(Arc::new(dengpai));

    // 阿米娅
    let amiya = load_model("amiya.obj", 2.8);
    let amiya = RotateY::new(Arc::new(amiya), 210.0);
    let amiya = Translate::new(Arc::new(amiya), Vec3::new(475.0, 0.0, 580.0));
    world.add(Arc::new(amiya));

    // 夕泡泡
    let longpao = load_model("arknights_dusk_plush_doll.obj", 170.0);
    let longpao = RotateY::new(Arc::new(longpao), 135.0);
    let longpao = Translate::new(Arc::new(longpao), Vec3::new(100.0, 165.0, 545.0));
    world.add(Arc::new(longpao));

    // 斯卡蒂
    let sakaban = load_model("skadi.obj", 2.0);
    let sakaban = RotateY::new(Arc::new(sakaban), 240.0);
    let sakaban = Translate::new(Arc::new(sakaban), Vec3::new(750.0, 50.0, 420.0));
    world.add(Arc::new(sakaban));

    // 无人机
    let drone = load_model("drone.obj", 30.0);
    let drone = RotateY::new(Arc::new(drone), 70.0);
    let drone = Translate::new(Arc::new(drone), Vec3::new(780.0, 300.0, 550.0));
    world.add(Arc::new(drone));

    // 玻璃砖
    let white = Dielectric::new(1.5);
    let box2 = make_box(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(200.0, 165.0, 165.0),
        Arc::new(white),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(
        box2,
        Vec3::new(80.0 - 33.8, 0.0, 465.0 - 9.06),
    ));
    world.add(box2);

    // 雨
    let mut rain = HittableList::new();
    for i in 0..100 {
        let center: Vec3 = Vec3::new(
            random_double_range(-500.0, 1055.0),
            random_double_range(100.0, 555.0),
            random_double_range(100.0, 855.0),
        );
        let albedo = Color::new(0.73, 0.73, 0.73);
        let sphere_material = Lambertian::new(albedo);
        let center2 = center + Vec3::new(0.0, random_double_range(20.0, 40.0), 0.0);
        rain.add(Arc::new(Sphere::new_dyn(
            center,
            center2,
            2.5,
            Arc::new(sphere_material),
        )));
    }
    let rain = BvhNode::new_list(&mut rain);
    world.add(Arc::new(rain));

    // 右侧若干球
    let glass = Arc::new(Dielectric::new(1.5));
    let mut light = MappedMaterial::new(glass.clone());
    light.set_light("light_mapping2.png", 1.0);
    world.add(Arc::new(Sphere::new(
        Vec3::new(-130.0, 87.13 + 65.0, 340.0 - 75.05),
        50.0,
        Arc::new(light),
    )));
    let metal = Arc::new(Metal::new(Color::new(0.1, 0.5, 0.1), 0.4));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-130.0, 65.0, 340.0),
        65.0,
        metal.clone(),
    )));
    let metal = Arc::new(Metal::new(Color::new(0.1, 0.1, 0.5), 0.4));
    world.add(Arc::new(Sphere::new(
        Vec3::new(
            -130.0 - 130.0 * degrees_to_radians(60.0).cos(),
            65.0,
            340.0 - 130.0 * degrees_to_radians(60.0).sin(),
        ),
        65.0,
        metal.clone(),
    )));
    let metal = Arc::new(Metal::new(Color::new(0.5, 0.5, 0.1), 0.4));
    world.add(Arc::new(Sphere::new(
        Vec3::new(
            -130.0 + 130.0 * degrees_to_radians(60.0).cos(),
            65.0,
            340.0 - 130.0 * degrees_to_radians(60.0).sin(),
        ),
        65.0,
        metal.clone(),
    )));

    // 斯卡蒂左侧球
    let moon = Lambertian::new(Color::new(0.0, 0.0, 0.0));
    let mut moon = MappedMaterial::new(Arc::new(moon));
    moon.set_light("moon.png", 1.0);
    let moon = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 40.0, Arc::new(moon)));
    let moon = Arc::new(RotateY::new(moon, 180.0));
    let moon = Arc::new(Translate::new(moon, Vec3::new(880.0, 40.0, 350.0)));
    world.add(moon);
    world.add(Arc::new(Sphere::new(
        Vec3::new(800.0, 36.0, 300.0),
        36.0,
        glass.clone(),
    )));

    // 斯卡蒂右侧
    let mat = Arc::new(Lambertian::new(Color::new(0.6, 0.07, 0.45)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(675.0, 15.0, 320.0),
        15.0,
        mat.clone(),
    )));
    let mat = Arc::new(DiffuseLight::new(&Color::new(1.0, 0.7, 0.8)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(660.0, 15.0, 350.0),
        15.0,
        mat.clone(),
    )));
    let mat = Arc::new(Lambertian::new(Color::new(0.6, 0.07, 0.07)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(615.0, 15.0, 335.0),
        15.0,
        mat.clone(),
    )));
    let mat = Arc::new(Lambertian::new(Vec3::new(0.6, 0.85, 0.9)));
    let box1 = make_box(
        &Vec3::new(-15.0, -15.0, -15.0),
        &Vec3::new(15.0, 15.0, 15.0),
        mat.clone(),
    );
    let box1 = RotateY::new(box1, 150.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(645.0, 15.0, 275.0));
    world.add(Arc::new(box1));
    let mat = Arc::new(Metal::new(Vec3::new(0.9, 0.85, 0.2), 0.7));
    let box1 = make_box(
        &Vec3::new(-15.0, -15.0, -15.0),
        &Vec3::new(15.0, 15.0, 15.0),
        mat.clone(),
    );
    let box1 = RotateY::new(box1, 210.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(630.0, 15.0, 385.0));
    world.add(Arc::new(box1));

    let lookfrom = Vec3::new(278.0, 600.0, -600.0);
    let lookat = Vec3::new(278.0, 278.0, 260.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut cam: Camera = Camera::new(
        16.0 / 9.0,
        1600,
        5000,
        50,
        40.0,
        lookfrom,
        lookat,
        vup,
        0.0,
        1500.0,
        Color::new(0.01, 0.01, 0.01),
    );
    let path = std::path::Path::new("output/final_scene.png");
    cam.initialize();
    let world_arc: Arc<dyn Hittable> = Arc::new(world);
    let lights_arc: Arc<dyn Hittable> = Arc::new(lights);
    cam.render(&world_arc, &lights_arc, path);
}
fn main() {
    let start = Instant::now();
    let a = 0;
    match a {
        0 => final_scene(),
        1 => obj_test(),
        2 => normal_mapping_test(),
        3 => all_mapping_test(),
        9 => book2_final_scene(800, 10000, 40),
        10 => book3_cornell_box(),
        _ => (),
    }
    let duration = start.elapsed();
    println!("代码执行耗时: {:?}", duration);
}
