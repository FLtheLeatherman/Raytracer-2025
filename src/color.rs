use crate::interval::Interval;
use crate::vec3::Vec3;
use image::{ImageBuffer, RgbImage};

pub type Color = Vec3;

lazy_static! {
    pub static ref INTENSITY: Interval = Interval::new(0.0, 0.999);
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}
pub fn write_color(x: u32, y: u32, pixel_color: &Color, img: &mut RgbImage) {
    let pixel = img.get_pixel_mut(x, y);
    let mut _r = pixel_color.x;
    let mut _g = pixel_color.y;
    let mut _b = pixel_color.z;
    if _r.is_nan() {
        _r = 0.0;
    }
    if _g.is_nan() {
        _g = 0.0;
    }
    if _b.is_nan() {
        _b = 0.0;
    }
    let r = (INTENSITY.clamp(linear_to_gamma(_r)) * 256.0) as u8;
    let g = (INTENSITY.clamp(linear_to_gamma(_g)) * 256.0) as u8;
    let b = (INTENSITY.clamp(linear_to_gamma(_b)) * 256.0) as u8;
    *pixel = image::Rgb([r, g, b]);
}
