use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use dyn_clone::DynClone;

pub trait Material: DynClone {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
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
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64
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
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction.dot(&rec.normal) > 0.0
    }
}
