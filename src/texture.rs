use crate::color::Color;
use crate::vec3::Vec3;
use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
pub struct SolidColor {
    pub albedo: Color,
}
impl SolidColor {
    pub fn new_color(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }
}
impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.albedo
    }
}
pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn new_color(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Rc::new(SolidColor::new_color(c1)),
            odd: Rc::new(SolidColor::new_color(c2)),
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let x_integer = (p.x * self.inv_scale).floor() as i32;
        let y_integer = (p.y * self.inv_scale).floor() as i32;
        let z_integer = (p.z * self.inv_scale).floor() as i32;
        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
