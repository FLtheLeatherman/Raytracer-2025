use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, r_tmin: f64, r_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            false,
        );
        let mut hit_anything = false;
        let mut closest_so_far = r_tmax;
        for object in &self.objects {
            if (*object).hit(r, r_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
