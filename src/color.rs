use crate::vec3::Vec3;
use image::{ImageBuffer, RgbImage};

pub type Color = Vec3;

pub fn write_color(x: u32, y: u32, pixel_color: &Color, img: &mut RgbImage) {
    let pixel = img.get_pixel_mut(x, y);
    *pixel = image::Rgb([
        (pixel_color.x * 255.999) as u8,
        (pixel_color.y * 255.999) as u8,
        (pixel_color.z * 255.999) as u8,
    ]);
}
