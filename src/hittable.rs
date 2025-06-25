use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
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

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}
