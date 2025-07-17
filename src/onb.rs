use crate::vec3::Vec3;

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: &Vec3) -> Self {
        let z = n.unit();
        let a = if z.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let y = z.cross(&a).unit();
        let x = z.cross(&y);
        Self { axis: [x, y, z] }
    }
    // pub fn u(&self) -> Vec3 {
    //     self.axis[0]
    // }
    // pub fn v(&self) -> Vec3 {
    //     self.axis[1]
    // }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    pub fn transform(&self, v: &Vec3) -> Vec3 {
        (self.axis[0] * v.x) + (self.axis[1] * v.y) + (self.axis[2] * v.z)
    }
}
