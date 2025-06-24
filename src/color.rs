use image::{ImageBuffer, RgbImage};
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(x: u32, y: u32, pixel_color: &Color, img: &mut RgbImage) {
    let pixel = img.get_pixel_mut(x, y);
    *pixel = image::Rgb([pixel_color.x as u8, pixel_color.y as u8, pixel_color.z as u8]);
}