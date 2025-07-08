use crate::color::Color;
use crate::hittable::HitRecord;
use crate::interval::Interval;
use crate::onb::ONB;
use crate::pdf::{CosinePDF, PDF, SpherePDF};
use crate::ray::Ray;
use crate::rtw_stb_image::RtwImage;
use crate::texture::{CheckerTexture, SolidColor, Texture};
use crate::utility::{PI, random_double};
use crate::vec3::{Vec3, random_cosine_direction};
use dyn_clone::DynClone;
use rand::random;
use std::ops::Neg;
use std::ptr::{null, null_mut};
use std::slice::EscapeAscii;
use std::sync::Arc;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Arc<dyn PDF>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}
impl Default for ScatterRecord {
    fn default() -> Self {
        Self {
            attenuation: Color::default(),
            pdf_ptr: Arc::new(SpherePDF::new()),
            skip_pdf: false,
            skip_pdf_ray: Ray::default(),
        }
    }
}

pub trait Material: DynClone + Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool;
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
    fn get_normal(&self, u: f64, v: f64) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    fn get_alpha(&self, u: f64, v: f64) -> f64 {
        1.0
    }
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            tex: Arc::new(SolidColor::new_color(&albedo)),
        }
    }
    pub fn new_tex(tex: Arc<dyn Texture>) -> Lambertian {
        Lambertian { tex }
    }
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Arc::new(CosinePDF::new(&rec.normal));
        srec.skip_pdf = false;
        true
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = rec.normal.dot(&scattered.direction.unit());
        if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let mut reflected = Vec3::reflect(&r_in.direction, &rec.normal);
        reflected = reflected.unit() + (Vec3::random_unit_vector() * self.fuzz);
        srec.attenuation = self.albedo;
        srec.pdf_ptr = Arc::new(SpherePDF::new());
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new_time(rec.p, reflected, r_in.tm);
        true
    }
}

#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f64,
}
impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.pdf_ptr = Arc::new(SpherePDF::new());
        srec.skip_pdf = true;
        let mut ri: f64 = 0.0;
        if rec.front_face {
            ri = 1.0 / self.refraction_index
        } else {
            ri = self.refraction_index
        }
        let unit_direction = r_in.direction.unit();
        let cos_theta = unit_direction.neg().dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let mut direction = Vec3::default();
        if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, ri);
        }
        srec.skip_pdf_ray = Ray::new_time(rec.p, direction, r_in.tm);
        true
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(emit: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new_color(emit)),
        }
    }
    pub fn new_tex(tex: impl Texture + 'static) -> Self {
        Self { tex: Arc::new(tex) }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        false
    }
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Color {
        if !rec.front_face {
            return Color::new(0.0, 0.0, 0.0);
        }
        self.tex.value(u, v, p)
    }
}

#[derive(Clone)]
pub struct Isotropic {
    tex: Arc<dyn Texture>,
}
impl Isotropic {
    pub fn new(albedo: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new_color(albedo)),
        }
    }
    pub fn new_tex(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}
impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Arc::new(SpherePDF::new());
        srec.skip_pdf = false;
        true
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}

#[derive(Clone)]
pub struct MappedMaterial {
    base_material: Arc<dyn Material>,
    normal_map: Option<Arc<RtwImage>>,
    alpha_map: Option<Arc<RtwImage>>,
    light_map: Option<Arc<RtwImage>>,
    emissive_strength: f64,
}
impl MappedMaterial {
    pub fn new(base_material: Arc<dyn Material>) -> Self {
        Self {
            base_material: base_material.clone(),
            normal_map: None,
            alpha_map: None,
            light_map: None,
            emissive_strength: 0.0,
        }
    }
    pub fn set_normal(&mut self, normal_filename: &str) {
        self.normal_map = Option::from(Arc::new(RtwImage::new(normal_filename)));
    }
    pub fn set_alpha(&mut self, alpha_filename: &str) {
        self.alpha_map = Option::from(Arc::new(RtwImage::new(alpha_filename)));
    }
    pub fn set_light(&mut self, light_filename: &str, emissive_strength: f64) {
        self.light_map = Option::from(Arc::new(RtwImage::new(light_filename)));
        self.emissive_strength = emissive_strength;
    }
}
impl Material for MappedMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        self.base_material.scatter(r_in, rec, srec)
    }
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Color {
        match &self.light_map {
            Some(image_data) => {
                let u = Interval::new(0.0, 1.0).clamp(rec.u);
                let v = 1.0 - Interval::new(0.0, 1.0).clamp(rec.v);
                let i = (image_data.image_width as f64 * u) as usize;
                let j = (image_data.image_height as f64 * v) as usize;
                let pixel = image_data.pixel_data(i, j);
                let color = Vec3::new(
                    pixel[0] as f64 / 255.99,
                    pixel[1] as f64 / 255.99,
                    pixel[2] as f64 / 255.99,
                );
                color * self.emissive_strength
            }
            None => self.base_material.emitted(r_in, rec, u, v, p),
        }
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        self.base_material.scattering_pdf(r_in, rec, scattered)
    }
    fn get_normal(&self, u: f64, v: f64) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
        // match &self.normal_map {
        //     Some(image_data) => {
        //         let u = Interval::new(0.0, 1.0).clamp(u);
        //         let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);
        //         let i = (image_data.image_width as f64 * u) as usize;
        //         let j = (image_data.image_height as f64 * v) as usize;
        //         let pixel = image_data.pixel_data(i, j);
        //         Vec3::new(
        //             (pixel[0] as f64 / 255.99) * 2.0 - 1.0,
        //             (pixel[1] as f64 / 255.99) * 2.0 - 1.0,
        //             (pixel[2] as f64 / 255.99) * 2.0 - 1.0,
        //         )
        //     }
        //     None => Vec3::new(0.0, 0.0, 0.0),
        // }
    }
    fn get_alpha(&self, u: f64, v: f64) -> f64 {
        match &self.alpha_map {
            Some(image_data) => {
                let u = Interval::new(0.0, 1.0).clamp(u);
                let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);
                let i = (image_data.image_width as f64 * u) as usize;
                let j = (image_data.image_height as f64 * v) as usize;
                let pixel = image_data.pixel_data(i, j);
                let alpha = pixel[0] as f64 / 255.99;
                alpha
            }
            None => 1.0,
        }
    }
}
