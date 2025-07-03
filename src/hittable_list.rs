use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::random_int_range;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: AABB::default(),
        }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::new_aabb(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            if (*object).hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in self.objects.iter() {
            sum += weight * object.pdf_value(origin, direction);
        }
        sum
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        self.objects[random_int_range(0, self.objects.len() as i32 - 1) as usize].random(origin)
    }
}
