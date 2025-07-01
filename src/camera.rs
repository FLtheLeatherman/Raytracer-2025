use crate::color::{Color, write_color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utility::{INFINITY, degrees_to_radians, random_double};
use crate::vec3::Vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use lazy_static::initialize;
use rand::random;
use rayon::prelude::*;

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
    vfov: f64,
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    background: Color,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: i32,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
        background: Color,
    ) -> Camera {
        Camera {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Vec3::default(),
            pixel00_loc: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            samples_per_pixel,
            pixel_sample_scale: 1.0 / samples_per_pixel as f64,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_angle,
            focus_dist,
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
            background,
        }
    }
    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;
        self.center = self.lookfrom;
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = (self.image_width as f64 / self.image_height as f64) * viewport_height;
        self.w = (self.lookfrom - self.lookat).unit();
        self.u = self.vup.cross(&self.w).unit();
        self.v = self.w.cross(&self.u);
        let viewport_u = self.u * viewport_width;
        let viewport_v = self.v * (-viewport_height);
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
        let viewport_upper_left =
            self.center - (self.w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_unit_vector();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));
        let mut ray_origin = Vec3::default();
        if self.defocus_angle <= 0.0 {
            ray_origin = self.center;
        } else {
            ray_origin = self.defocus_disk_sample();
        }
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();
        Ray::new_time(ray_origin, ray_direction, ray_time)
    }
    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable, rate: f64) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec: HitRecord = HitRecord::default();
        if !world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);
        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }
        let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world, rate);
        color_from_emission + color_from_scatter
    }
    pub fn render(&self, world: &(dyn Hittable + Sync), path: &std::path::Path) {
        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        let pixels: Vec<(u32, u32, Color)> = (0..self.image_height)
            .into_par_iter()
            .flat_map(|j| {
                (0..self.image_width).into_par_iter().map(move |i| {
                    let rate = 0.5;
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _sample in 0..self.samples_per_pixel {
                        // 这里 self 会被自动地、不可变地借用 (&self)
                        // world 会被自动地、不可变地借用 (&world)
                        let r = self.get_ray(i, j);
                        pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world, rate);
                    }
                    (i, j, pixel_color)
                })
            })
            .collect();
        for (i, j, pixel_color) in pixels {
            write_color(i, j, &(pixel_color * self.pixel_sample_scale), &mut img);
        }
        // let progress = if option_env!("CI").unwrap_or_default() == "true" {
        //     ProgressBar::hidden()
        // } else {
        //     ProgressBar::new((self.image_height * self.image_width) as u64)
        // };
        // for j in 0..self.image_height {
        //     for i in 0..self.image_width {
        //         let rate = 0.5;
        //         let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        //         for sample in 0..self.samples_per_pixel {
        //             let r = self.get_ray(i, j);
        //             pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world, rate);
        //         }
        //         write_color(i, j, &(pixel_color * self.pixel_sample_scale), &mut img);
        //         progress.inc(1);
        //     }
        // }
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
        img.save(path).expect("Cannot save the image to the file");
    }
}
