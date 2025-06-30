use crate::utility::{random_double, random_int_range};
use crate::vec3::Vec3;

const POINT_COUNT: usize = 256;
pub struct Perlin {
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
    randfloat: [f64; POINT_COUNT],
}
impl Perlin {
    fn permute(p: &mut [i32; POINT_COUNT]) {
        for i in (0..POINT_COUNT).rev() {
            let target = random_int_range(0, i as i32);
            let tmp = p[i];
            p[i] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
    fn perlin_generate_perm(p: &mut [i32; POINT_COUNT]) {
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }
        Self::permute(p);
    }
    pub fn new() -> Self {
        let mut randfloat = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            randfloat[i] = random_double();
        }
        let mut perm_x = [0i32; POINT_COUNT];
        let mut perm_y = [0i32; POINT_COUNT];
        let mut perm_z = [0i32; POINT_COUNT];
        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);
        Self {
            perm_x,
            perm_y,
            perm_z,
            randfloat,
        }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;
        self.randfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}
