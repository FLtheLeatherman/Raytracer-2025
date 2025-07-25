use crate::color::Color;
use crate::interval::Interval;
use crate::perlin::Perlin;
use crate::rtw_stb_image::RtwImage;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct UV {
    pub u: Vec3,
    pub v: Vec3,
}
impl UV {
    pub fn new(u: Vec3, v: Vec3) -> Self {
        Self { u, v }
    }
    pub fn get_uv(&self, coef: Vec3) -> (f64, f64) {
        (self.u.dot(&coef), self.v.dot(&coef))
    }
}
impl Default for UV {
    fn default() -> Self {
        Self {
            u: Vec3::new(0.0, 1.0, 0.0),
            v: Vec3::new(0.0, 0.0, 1.0),
        }
    }
}

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}
pub struct SolidColor {
    pub albedo: Color,
}
impl SolidColor {
    pub fn new_color(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
    // pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
    //     Self {
    //         albedo: Color::new(red, green, blue),
    //     }
    // }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        self.albedo
    }
}
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}
impl CheckerTexture {
    // pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
    //     Self {
    //         inv_scale: 1.0 / scale,
    //         even,
    //         odd,
    //     }
    // }
    // pub fn new_color(scale: f64, c1: &Color, c2: &Color) -> Self {
    //     Self {
    //         inv_scale: 1.0 / scale,
    //         even: Arc::new(SolidColor::new_color(c1)),
    //         odd: Arc::new(SolidColor::new_color(c2)),
    //     }
    // }
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

pub struct ImageTexture {
    image: RtwImage,
}
impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::new(filename),
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Vec3) -> Color {
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        u = Interval::new(0.0, 1.0).clamp(u);
        v = 1.0 - Interval::new(0.0, 1.0).clamp(v);
        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;
        let pixel = self.image.pixel_data(i, j);
        let color_scale = 1.0 / 255.0;
        Color::new(
            (color_scale * pixel[0] as f64) * (color_scale * pixel[0] as f64),
            (color_scale * pixel[1] as f64) * (color_scale * pixel[1] as f64),
            (color_scale * pixel[2] as f64) * (color_scale * pixel[2] as f64),
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}
impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(p, 7)))
    }
}
