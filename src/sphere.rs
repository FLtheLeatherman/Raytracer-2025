use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: impl Material + 'static) -> Sphere {
        Sphere {
            center,
            radius,
            mat: Arc::new(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}
