use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            tm: 0.0,
        }
    }
    pub fn new_time(origin: Vec3, direction: Vec3, tm: f64) -> Self {
        Self {
            origin,
            direction,
            tm,
        }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
