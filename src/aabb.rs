use crate::interval::{INTERVAL_EMPTY, INTERVAL_UNIVERSE, Interval};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops::Add;

#[derive(Copy, Clone, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
        let mut _self = Aabb {
            x: *x,
            y: *y,
            z: *z,
        };
        _self.pad_to_minimums();
        _self
    }
    pub fn new_points(a: &Vec3, b: &Vec3) -> Self {
        let mut _self = Self {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        };
        _self.pad_to_minimums();
        _self
    }
    pub fn new_aabb(box0: &Aabb, box1: &Aabb) -> Self {
        Aabb {
            x: Interval::new_interval(&box0.x, &box1.x),
            y: Interval::new_interval(&box0.y, &box1.y),
            z: Interval::new_interval(&box0.z, &box1.z),
        }
    }
    pub fn axis_interval(&self, n: u32) -> &Interval {
        if n == 1 {
            &self.y
        } else if n == 2 {
            &self.z
        } else {
            &self.x
        }
    }
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let ray_orig = r.origin;
        let ray_dir = r.direction;
        let mut tmp_ray = *ray_t;
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.axis(axis);
            let t0 = (ax.min - ray_orig.axis(axis)) * adinv;
            let t1 = (ax.max - ray_orig.axis(axis)) * adinv;
            if t0 < t1 {
                if t0 > tmp_ray.min {
                    tmp_ray.min = t0;
                }
                if t1 < tmp_ray.max {
                    tmp_ray.max = t1;
                }
            } else {
                if t1 > tmp_ray.min {
                    tmp_ray.min = t1;
                }
                if t0 < tmp_ray.max {
                    tmp_ray.max = t0;
                }
            }
            if tmp_ray.max <= tmp_ray.min {
                return false;
            }
        }
        true
    }
    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }
}
impl Add<Vec3> for Aabb {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::new(&(self.x + rhs.x), &(self.y + rhs.y), &(self.z + rhs.z))
    }
}
impl Add<Aabb> for Vec3 {
    type Output = Aabb;
    fn add(self, rhs: Aabb) -> Self::Output {
        rhs + self
    }
}
lazy_static! {
    pub static ref AABB_EMPTY: Aabb = Aabb::new(&INTERVAL_EMPTY, &INTERVAL_EMPTY, &INTERVAL_EMPTY);
    pub static ref AABB_UNIVERSE: Aabb =
        Aabb::new(&INTERVAL_UNIVERSE, &INTERVAL_UNIVERSE, &INTERVAL_UNIVERSE);
}
