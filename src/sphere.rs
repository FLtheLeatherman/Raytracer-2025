use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::utility::PI;
use crate::vec3::Vec3;
use std::rc::Rc;
use std::sync::Arc;

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
    pub bbox: AABB,
}

impl Sphere {
    pub fn new(static_center: Vec3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius,
            mat: mat.clone(),
            bbox: AABB::new_points(&(static_center - rvec), &(static_center + rvec)),
        }
    }
    pub fn new_dyn(center1: Vec3, center2: Vec3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        let _center = Ray::new(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::new_points(&(_center.at(0.0) - rvec), &(_center.at(0.0) + rvec));
        let box2 = AABB::new_points(&(_center.at(1.0) - rvec), &(_center.at(1.0) + rvec));
        Sphere {
            center: _center,
            radius,
            mat: mat.clone(),
            bbox: AABB::new_aabb(&box1, &box2),
        }
    }
    pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        (phi / (2.0 * PI), theta / PI)
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
        (rec.u, rec.v) = Self::get_sphere_uv(&outward_normal);
        rec.mat = self.mat.clone();
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
