use crate::aabb::AABB;
use crate::color::Color;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::utility::{INFINITY, degrees_to_radians};
use crate::vec3::Vec3;
use console::StyledObject;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
        mat: Arc<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            mat: mat.clone(),
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        if Vec3::dot(&r.direction, &outward_normal) < 0.0 {
            self.front_face = true;
            self.normal = outward_normal;
        } else {
            self.front_face = false;
            self.normal = -outward_normal;
        }
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat: Arc::new(Lambertian::new(Color::default())),
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}
impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            object: object.clone(),
            offset,
            bbox: object.bounding_box() + offset,
        }
    }
}
impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new_time(r.origin - self.offset, r.direction, r.tm);
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }
        rec.p = rec.p + self.offset;
        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}
impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = object.bounding_box();
        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;
                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(new_x, y, new_z);
                    min.x = min.x.min(tester.x);
                    min.y = min.y.min(tester.y);
                    min.z = min.z.min(tester.z);
                    max.x = max.x.max(tester.x);
                    max.y = max.y.max(tester.y);
                    max.z = max.z.max(tester.z);
                }
            }
        }
        bbox = AABB::new_points(&min, &max);
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}
impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let origin = Vec3::new(
            self.cos_theta * r.origin.x - self.sin_theta * r.origin.z,
            r.origin.y,
            self.sin_theta * r.origin.x + self.cos_theta * r.origin.z,
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction.x - self.sin_theta * r.direction.z,
            r.direction.y,
            self.sin_theta * r.direction.x + self.cos_theta * r.direction.z,
        );
        let rotated_r = Ray::new_time(origin, direction, r.tm);
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }
        rec.p = Vec3::new(
            self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
            rec.p.y,
            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
        );
        rec.normal = Vec3::new(
            self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
            rec.normal.y,
            -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
        );
        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
