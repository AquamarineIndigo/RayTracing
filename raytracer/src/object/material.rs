use super::hittable::HitRecord;
// use super::texture::{Textures};//, CheckeredTexture};
// use super::texture::Texture;
use crate::basic::random_double;
use crate::basic::ray::Ray;
use crate::basic::vec3::{
    generate_unit_vector, random_in_unit_sphere, random_unit_vector, reflect, refract, vec3_dot,
    vec3_mul, Vec3,
};
// use super::SolidColour;
use super::{SolidColour, Texture};
// std::marker::Sized;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::set(0.0, 0.0, 0.0)
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian<T: Texture + Clone> {
    // pub albedo: Vec3,
    pub albedo: T,
}
#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ir: f64,
}
#[derive(Clone)]
pub struct DiffuseLight<T: Texture + Clone> {
    pub emit: T,
}
#[derive(Clone)]
pub struct Isotropic<T: Texture + Clone> {
    pub albedo: T,
}

impl<T: Texture + Clone> Lambertian<T> {
    #[allow(dead_code)]
    pub fn new_from_textures(a: &T) -> Self {
        Self { albedo: a.clone() }
    }
}
impl Lambertian<SolidColour> {
    // default material
    #[allow(dead_code)]
    pub fn new_from_vector(a: &Vec3) -> Self {
        Self {
            albedo: SolidColour::new_from_vector(a),
        }
    }
    pub fn new_from_colour(a: f64, b: f64, c: f64) -> Self {
        Self {
            albedo: SolidColour::new_from_rgb(a, b, c),
        }
    }
    #[allow(dead_code)]
    pub fn set(a: f64, b: f64, c: f64) -> Self {
        Self {
            albedo: SolidColour::new_from_rgb(a, b, c),
        }
    }
}
impl Metal {
    pub fn new_from_vector(a: &Vec3, f: f64) -> Self {
        Self {
            albedo: *a,
            fuzz: f,
        }
    }
    pub fn set(a: f64, b: f64, c: f64, f: f64) -> Self {
        Self {
            albedo: Vec3::set(a, b, c),
            fuzz: f,
        }
    }
}
impl Dielectric {
    pub fn set(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
    fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}
impl<T: Texture + Clone> DiffuseLight<T> {
    #[allow(dead_code)]
    pub fn new_from_textures(a: &T) -> Self {
        Self { emit: a.clone() }
    }
}
impl DiffuseLight<SolidColour> {
    pub fn new_from_vector(a: &Vec3) -> Self {
        Self {
            emit: SolidColour::new_from_vector(a),
        }
    }
    pub fn new_from_colour(a: f64, b: f64, c: f64) -> Self {
        Self {
            emit: SolidColour::new_from_rgb(a, b, c),
        }
    }
}
impl<T: Texture + Clone> Isotropic<T> {
    #[allow(dead_code)]
    pub fn new_from_textures(a: &T) -> Self {
        Self { albedo: a.clone() }
    }
}
impl Isotropic<SolidColour> {
    // default material
    #[allow(dead_code)]
    pub fn new_from_vector(a: &Vec3) -> Self {
        Self {
            albedo: SolidColour::new_from_vector(a),
        }
    }
    pub fn new_from_colour(a: f64, b: f64, c: f64) -> Self {
        Self {
            albedo: SolidColour::new_from_rgb(a, b, c),
        }
    }
}

impl<T: Texture + Clone + std::marker::Send + std::marker::Sync> Material for Lambertian<T> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction.copy_vector(&rec.normal);
        }
        scattered.set_value(rec.p, scatter_direction, r_in.tm);
        attenuation.copy_vector(&self.albedo.value(rec.u, rec.v, &rec.p));
        true
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&generate_unit_vector(&r_in.direction), &rec.normal);
        scattered.copy_ray(&Ray::set_with_time(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.tm,
        ));
        attenuation.copy_vector(&self.albedo);
        vec3_dot(&scattered.direction, &rec.normal) > 0.0
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        attenuation.set_value(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = {
            if rec.front_face {
                1.0 / self.ir
            } else {
                self.ir
            }
        };
        let unit_direction = generate_unit_vector(&r_in.direction);
        let dt = vec3_dot(&rec.normal, &vec3_mul(&-1.0, &unit_direction));
        let cos_theta = {
            if dt < 1.0 {
                dt
            } else {
                1.0
            }
        };
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let mut direction = Vec3::set(0.0, 0.0, 0.0);
        if (refraction_ratio * sin_theta > 1.0)
            || (Dielectric::reflectance(cos_theta, refraction_ratio) > random_double())
        {
            direction.copy_vector(&reflect(&unit_direction, &rec.normal));
        } else {
            direction.copy_vector(&refract(&unit_direction, &rec.normal, &refraction_ratio));
        }
        scattered.set_value(rec.p, direction, r_in.tm);
        true
    }
}
impl<T: Texture + Clone + std::marker::Send + std::marker::Sync> Material for DiffuseLight<T> {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
impl<T: Texture + Clone + std::marker::Send + std::marker::Sync> Material for Isotropic<T> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        scattered.set_value(rec.p, random_in_unit_sphere(), r_in.tm);
        attenuation.copy_vector(&self.albedo.value(rec.u, rec.v, &rec.p));
        true
    }
}
impl Default for Metal {
    fn default() -> Self {
        Metal {
            albedo: Vec3::set(0.0, 0.0, 0.0),
            fuzz: 0.0,
        }
    }
}
