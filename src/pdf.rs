use crate::hittable::Hittable;
use crate::onb::ONB;
use crate::utility::{random_double, PI};
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
    pub fn new(objects: &Arc<dyn Hittable>, origin: &Vec3) -> Self {
        Self {
            objects: objects.clone(),
            origin: *origin,
        }
    }
}
impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

pub struct MixturePDF {
    p: [Arc<dyn PDF>; 2],
}
impl MixturePDF {
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> Self {
        Self {
            p: [p0, p1],
        }
    }
}
impl PDF for MixturePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
