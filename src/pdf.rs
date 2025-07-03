use crate::hittable::Hittable;
use crate::onb::ONB;
use crate::utility::PI;
use crate::vec3::{Vec3, random_cosine_direction};
use std::sync::Arc;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct SpherePDF {}
impl SpherePDF {
    pub fn new() -> Self {
        Self {}
    }
}
impl PDF for SpherePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }
    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

pub struct CosinePDF {
    uvw: ONB,
}
impl CosinePDF {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: ONB::new(w) }
    }
}
impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = direction.unit().dot(&self.uvw.w());
        (cosine_theta / PI).max(0.0)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.transform(&random_cosine_direction())
    }
}

pub struct HittablePDF {
    objects: Arc<dyn Hittable>,
    origin: Vec3,
}
impl HittablePDF {
    pub fn new(objects: Arc<dyn Hittable>, origin: &Vec3) -> Self {
        Self {
            objects,
            origin: *origin,
        }
    }
}
impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        todo!()
    }

    fn generate(&self) -> Vec3 {
        todo!()
    }
}
