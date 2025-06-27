use rand::Rng;

pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + random_double() * (max - min)
}
pub fn random_int_range(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, max as f64 + 1.0) as i32
}
