use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: AABB::default(),
        }
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
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
}
