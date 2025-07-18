use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::utility::{INFINITY, random_double};
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    area: f64,
}
impl Quad {
    pub fn new(q: &Vec3, u: &Vec3, v: &Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox_diagonal1 = Aabb::new_points(q, &(*q + *u + *v));
        let bbox_diagonal2 = Aabb::new_points(&(*q + *u), &(*q + *v));
        let n = u.cross(v);
        let normal = n.unit();
        Self {
            q: *q,
            u: *u,
            v: *v,
            w: n / n.dot(&n),
            mat: mat.clone(),
            bbox: Aabb::new_aabb(&bbox_diagonal1, &bbox_diagonal2),
            normal,
            d: normal.dot(q),
            area: n.length(),
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
        rec.set_face_normal(r, self.normal, alpha, beta);
        if random_double() > self.mat.get_alpha(alpha, beta) {
            return false;
        }
        true
    }
    fn bounding_box(&self) -> Aabb {
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
        let distance_squared = rec.t * rec.t * direction.squared_length();
        let cosine = (direction.dot(&rec.normal) / direction.length()).abs();
        distance_squared / (cosine * self.area)
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let p = self.q + (self.u * random_double()) + (self.v * random_double());
        p - *origin
    }
}
pub fn make_box(a: &Vec3, b: &Vec3, mat: Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::new();
    let min = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);
    sides.add(Arc::new(Quad::new(
        &Vec3::new(min.x, min.y, max.z),
        &dx,
        &dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        &Vec3::new(max.x, min.y, max.z),
        &(-dz),
        &dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        &Vec3::new(max.x, min.y, min.z),
        &(-dx),
        &dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        &Vec3::new(min.x, min.y, min.z),
        &dz,
        &dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        &Vec3::new(min.x, max.y, max.z),
        &dx,
        &(-dz),
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        &Vec3::new(min.x, min.y, min.z),
        &dx,
        &dz,
        Arc::clone(&mat),
    )));
    Arc::new(sides)
}
