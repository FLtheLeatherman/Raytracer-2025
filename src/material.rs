use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::utility::random_double;
use crate::vec3::Vec3;
use dyn_clone::DynClone;
use rand::random;
use std::ops::Neg;
use std::rc::Rc;

pub trait Material: DynClone {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone)]
pub struct Lambertian {
    tex: Rc<dyn Texture>,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            tex: Rc::new(SolidColor::new_color(&albedo)),
        }
    }
    pub fn new_tex(tex: Rc<dyn Texture>) -> Lambertian {
        Lambertian { tex }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if (scatter_direction.near_zero()) {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new_time(rec.p, scatter_direction, r_in.tm);
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vec3::reflect(&r_in.direction, &rec.normal);
        reflected = reflected.unit() + (Vec3::random_unit_vector() * self.fuzz);
        *scattered = Ray::new_time(rec.p, reflected, r_in.tm);
        *attenuation = self.albedo;
        scattered.direction.dot(&rec.normal) > 0.0
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
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
        *scattered = Ray::new_time(rec.p, direction, r_in.tm);
        true
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Rc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(emit: &Color) -> Self {
        Self {
            tex: Rc::new(SolidColor::new_color(emit)),
        }
    }
    pub fn new_tex(tex: impl Texture + 'static) -> Self {
        Self { tex: Rc::new(tex) }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.tex.value(u, v, p)
    }
}
