use crate::interval::Interval;
use crate::vec3::Vec3;
use image::{ImageBuffer, RgbImage};

pub type Color = Vec3;

lazy_static! {
    pub static ref INTENSITY: Interval = Interval::new(0.0, 0.999);
}

pub fn write_color(x: u32, y: u32, pixel_color: &Color, img: &mut RgbImage) {
    let pixel = img.get_pixel_mut(x, y);
    let r = (INTENSITY.clamp(pixel_color.x) * 256.0) as u8;
    let g = (INTENSITY.clamp(pixel_color.y) * 256.0) as u8;
    let b = (INTENSITY.clamp(pixel_color.z) * 256.0) as u8;
    *pixel = image::Rgb([r, g, b]);
}
