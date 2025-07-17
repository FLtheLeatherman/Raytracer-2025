use crate::aabb::Aabb;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{INTERVAL_UNIVERSE, Interval};
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::utility::{INFINITY, random_double};
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}
impl ConstantMedium {
    // pub fn new(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
    //     Self {
    //         boundary: boundary.clone(),
    //         neg_inv_density: -1.0 / density,
    //         phase_function: Arc::new(Isotropic::new_tex(tex)),
    //     }
    // }
    pub fn new_color(boundary: Arc<dyn Hittable>, density: f64, albedo: &Color) -> Self {
        Self {
            boundary: boundary.clone(),
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(albedo)),
        }
    }
}
impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();
        if !self.boundary.hit(r, &INTERVAL_UNIVERSE, &mut rec1) {
            return false;
        }
        if !self
            .boundary
            .hit(r, &Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = self.phase_function.clone();
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
