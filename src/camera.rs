use crate::color::{Color, write_color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utility::{INFINITY, random_double};
use crate::vec3::Vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use lazy_static::initialize;

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    pixel_sample_scale: f64,
    max_depth: i32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: i32,
    ) -> Camera {
        Camera {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            samples_per_pixel,
            pixel_sample_scale: 1.0 / samples_per_pixel as f64,
            max_depth,
        }
    }
    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.center = Vec3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = (self.image_width as f64 / self.image_height as f64) * viewport_height;
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
        let camera_center = Vec3::new(0.0, 0.0, 0.0);
        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;
    }
    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec: HitRecord = HitRecord::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            false,
        );
        if world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_unit_vector();
            return Self::ray_color(&Ray::new(rec.p, direction), depth - 1, world) * 0.5;
        }
        let unit_direction: Vec3 = r.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
    pub fn render(&mut self, world: &dyn Hittable, path: &std::path::Path) {
        self.initialize();
        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + Self::ray_color(&r, self.max_depth, world);
                }
                write_color(i, j, &(pixel_color * self.pixel_sample_scale), &mut img);
                progress.inc(1);
            }
        }
        progress.finish();
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
        img.save(path).expect("Cannot save the image to the file");
    }
}
