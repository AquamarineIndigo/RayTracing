use super::hittable::HitRecord;
use crate::basic::random_double;
use crate::basic::ray::Ray;
use crate::basic::vec3::{
    generate_unit_vector, random_in_unit_sphere, random_unit_vector, reflect, refract, vec3_add,
    vec3_dot, vec3_mul, Vec3,
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
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

impl Lambertian {
    // default material
    pub fn new_from_vector(a: &Vec3) -> Self {
        Lambertian { albedo: *a }
    }
    pub fn set(a: f64, b: f64, c: f64) -> Self {
        Self {
            albedo: Vec3::set(a, b, c),
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

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = vec3_add(&rec.normal, &random_unit_vector());
        if scatter_direction.near_zero() {
            scatter_direction.copy_vector(&rec.normal);
        }
        scattered.set_value(rec.p, scatter_direction, r_in.tm);
        attenuation.copy_vector(&self.albedo);
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
            vec3_add(&reflected, &vec3_mul(&self.fuzz, &random_in_unit_sphere())),
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

#[derive(Copy, Clone)]
pub enum Materials {
    MetalMaterials(Metal),
    LambertianMaterials(Lambertian),
    DielectricMaterials(Dielectric),
}

impl Material for Materials {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Materials::LambertianMaterials(l) => l.scatter(r_in, rec, attenuation, scattered),
            Materials::MetalMaterials(m) => m.scatter(r_in, rec, attenuation, scattered),
            Materials::DielectricMaterials(d) => d.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian {
            albedo: Vec3::set(0.0, 0.0, 0.0),
        }
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
