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
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let _i = i as f64 * u + (1.0 - i as f64) * (1.0 - u);
                    let _j = j as f64 * v + (1.0 - j as f64) * (1.0 - v);
                    let _k = k as f64 * w + (1.0 - k as f64) * (1.0 - w);
                    accum += _i * _j * _k * c[i as usize][j as usize][k as usize];
                }
            }
        }
        accum
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.randfloat[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }
        Self::trilinear_interp(c, u, v, w)
    }
}
