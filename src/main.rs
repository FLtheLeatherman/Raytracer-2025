mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utility;
mod vec3;

use color::Color;
use color::write_color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use ray::Ray;
use sphere::Sphere;
use utility::INFINITY;
use vec3::Vec3;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        0.0,
        false,
    );
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return Color::new(1.0, 0.0, 0.0);
        // return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction: Vec3 = r.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}
fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    if image_height < 1 {
        image_height = 1;
    }
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    // world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = (image_width as f64 / image_height as f64) * viewport_height;
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel100_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel100_loc + pixel_delta_u * (i as f64) + pixel_delta_v * (j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &world);
            write_color(i, j, &pixel_color, &mut img);
        }
    }
    let path = std::path::Path::new("output/book1/image3.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    img.save(path).expect("Cannot save the image to the file");
}
