use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use stb_image::image::load_with_depth;
use std::rc::Rc;

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}
impl Quad {
    pub fn new(q: &Vec3, u: &Vec3, v: &Vec3, mat: impl Material + 'static) -> Self {
        let bbox_diagonal1 = AABB::new_points(q, &(*q + *u + *v));
        let bbox_diagonal2 = AABB::new_points(&(*q + *u), &(*q + *v));
        let n = u.cross(v);
        let normal = n.unit();
        Self {
            q: *q,
            u: *u,
            v: *v,
            w: n / n.dot(&n),
            mat: Rc::new(mat),
            bbox: AABB::new_aabb(&bbox_diagonal1, &bbox_diagonal2),
            normal,
            d: normal.dot(q),
        }
    }
    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}
impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&r.direction);
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.d - self.normal.dot(&r.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));
        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }
        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, self.normal);
        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
