use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(static_center: Vec3, radius: f64, mat: impl Material + 'static) -> Sphere {
        Sphere {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius,
            mat: Arc::new(mat),
        }
    }
    pub fn new_dyn(
        center1: Vec3,
        center2: Vec3,
        radius: f64,
        mat: impl Material + 'static,
    ) -> Sphere {
        Sphere {
            center: Ray::new(center1, center2 - center1),
            radius,
            mat: Arc::new(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.tm);
        let oc = current_center - r.origin;
        let a = Vec3::dot(&r.direction, &r.direction);
        let h = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}
