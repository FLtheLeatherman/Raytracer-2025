use crate::vec3::Vec3;

pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn new(n: &Vec3) -> Self {
        let z = n.unit();
        let mut a = Vec3::default();
        if z.x.abs() > 0.9 {
            a = Vec3::new(0.0, 1.0, 0.0);
        } else {
            a = Vec3::new(1.0, 0.0, 0.0);
        }
        let y = z.cross(&a).unit();
        let x = z.cross(&y);
        Self { axis: [x, y, z] }
    }
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    pub fn transform(&self, v: &Vec3) -> Vec3 {
        (self.axis[0] * v.x) + (self.axis[1] * v.y) + (self.axis[2] * v.z)
    }
}
