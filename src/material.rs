/*
// Description: This file defines various material types and their behaviors.
//              The base material struct requires methods for scattering rays
//              and emitting light. Isotropic materials will scatter light
//              uniformly in all directions. Lambertian (matte) models diffuse 
//              the reflection, scattering light uniformly in many directions.
//              Metal surfaces reflect rays in a single direction with some
//              fuzz/randomness. Dielectric materials are transparent (glass/ 
//              water), reflecting and refracting based on index of refraction.
//              Diffuse light will emmit light based on texture. 
*/
use crate::{hittable::*, ray::*, texture::*, utils::*, vec3::*};
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        Color::new()
    }
}
#[derive(Clone)]
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(c: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from(c)),
        }
    }
    pub fn from_texture(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        true
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn from(a: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from(a)),
        }
    }
    pub fn from_texture(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Color::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}
impl Metal {
    pub fn from(a: Color, f: f32) -> Self {
        if f < 1. {
            Self { albedo: a, fuzz: f }
        } else {
            Self {
                albedo: a,
                fuzz: 1.,
            }
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.time(),
        );
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ir: f32,
}
impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::from(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(r_in.direction());

        let cos_theta = dot(unit_direction.inv(), rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double(0., 1.) {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            };

        *scattered = Ray::new(rec.p, direction, r_in.time());

        true
    }
}
fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    pub fn from(a: Arc<dyn Texture>) -> Self {
        Self { emit: a }
    }
    pub fn from_color(c: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::from(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: Ray,
        _rec: HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}