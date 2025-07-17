use crate::utility::{random_double, random_int_range};
use crate::vec3::Vec3;

const POINT_COUNT: usize = 256;
pub struct Perlin {
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
    // randfloat: [f64; POINT_COUNT],
    randvec: [Vec3; POINT_COUNT],
}
impl Perlin {
    fn permute(p: &mut [i32; POINT_COUNT]) {
        for i in (0..POINT_COUNT).rev() {
            let target = random_int_range(0, i as i32);
            p.swap(i, target as usize);
        }
    }
    fn perlin_generate_perm(p: &mut [i32; POINT_COUNT]) {
        for (i, item) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *item = i as i32;
        }
        Self::permute(p);
    }
    pub fn new() -> Self {
        let mut randfloat = [0.0; POINT_COUNT];
        for item in randfloat.iter_mut().take(POINT_COUNT) {
            *item = random_double();
        }
        let mut perm_x = [0i32; POINT_COUNT];
        let mut perm_y = [0i32; POINT_COUNT];
        let mut perm_z = [0i32; POINT_COUNT];
        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);
        let mut randvec = [Vec3::default(); POINT_COUNT];
        for item in randvec.iter_mut().take(POINT_COUNT) {
            *item = Vec3::random_range(-1.0, 1.0).unit();
        }
        Self {
            perm_x,
            perm_y,
            perm_z,
            randvec,
        }
    }
    // fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    //     let mut accum = 0.0;
    //     for i in 0..2 {
    //         for j in 0..2 {
    //             for k in 0..2 {
    //                 let _i = i as f64 * u + (1.0 - i as f64) * (1.0 - u);
    //                 let _j = j as f64 * v + (1.0 - j as f64) * (1.0 - v);
    //                 let _k = k as f64 * w + (1.0 - k as f64) * (1.0 - w);
    //                 accum += _i * _j * _k * c[i as usize][j as usize][k as usize];
    //             }
    //         }
    //     }
    //     accum
    // }
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    let _i = i as f64 * uu + (1.0 - i as f64) * (1.0 - uu);
                    let _j = j as f64 * vv + (1.0 - j as f64) * (1.0 - vv);
                    let _k = k as f64 * ww + (1.0 - k as f64) * (1.0 - ww);
                    accum += _i * _j * _k * c[i as usize][j as usize][k as usize].dot(&weight_v);
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
        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.randvec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }
    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }
}
