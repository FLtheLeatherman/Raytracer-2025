use crate::utility;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: utility::INFINITY,
            max: -utility::INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }
    pub fn new_interval(a: &Interval, b: &Interval) -> Interval {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

lazy_static! {
    pub static ref INTERVAL_EMPTY: Interval = Interval::new(utility::INFINITY, -utility::INFINITY);
    pub static ref INTERVAL_UNIVERSE: Interval =
        Interval::new(-utility::INFINITY, utility::INFINITY);
}
