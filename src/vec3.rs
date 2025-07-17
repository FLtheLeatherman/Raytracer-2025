use crate::utility::{PI, random_double, random_double_range};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
    pub fn axis(&self, n: u32) -> f64 {
        if n == 0 {
            self.x
        } else if n == 1 {
            self.y
        } else {
            self.z
        }
    }
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    pub fn unit(&self) -> Vec3 {
        Vec3::new(
            self.x / self.length(),
            self.y / self.length(),
            self.z / self.length(),
        )
    }
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let lensq = p.squared_length();
            if lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f64::abs(self.x) < s && f64::abs(self.y) < s && f64::abs(self.z) < s
    }
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        (*v) - (*n) * 2.0 * Self::dot(v, n)
    }
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = 1.0_f64.min(Self::dot(&uv.neg(), n));
        let r_out_perp = (*uv + (*n) * cos_theta) * etai_over_etat;
        let r_out_parallel = (*n) * -f64::abs(1.0 - r_out_perp.squared_length()).sqrt();
        r_out_perp + r_out_parallel
    }
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }
    pub fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();
    Vec3::new(x, y, z)
}
