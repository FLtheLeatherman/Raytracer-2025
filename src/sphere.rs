use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::onb::ONB;
use crate::ray::Ray;
use crate::utility::{INFINITY, PI, random_double};
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
    pub bbox: AABB,
}

impl Sphere {
    pub fn new(static_center: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius,
            mat: mat.clone(),
            bbox: AABB::new_points(&(static_center - rvec), &(static_center + rvec)),
        }
    }
    pub fn new_dyn(center1: Vec3, center2: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
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
    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();
        Vec3::new(x, y, z)
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
        (rec.u, rec.v) = Self::get_sphere_uv(&outward_normal);
        rec.set_face_normal(r, outward_normal, rec.u, rec.v);
        rec.mat = self.mat.clone();
        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new(*origin, *direction),
            &Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }
        let dist_squared = (self.center.at(0.0) - *origin).squared_length();
        let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        return 1.0 / solid_angle;
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let direction = self.center.at(0.0) - *origin;
        let distance_squared = direction.squared_length();
        let uvw = ONB::new(&direction);
        uvw.transform(&Self::random_to_sphere(self.radius, distance_squared))
    }
}
