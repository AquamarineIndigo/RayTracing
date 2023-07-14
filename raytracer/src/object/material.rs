use super::hittable::HitRecord;
use crate::basic::ray::Ray;
use crate::basic::vec3::{
    generate_unit_vector, random_unit_vector, reflect, vec3_add, vec3_dot, Vec3,
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
    pub fn new_from_vector(a: &Vec3) -> Self {
        Metal { albedo: *a }
    }
    pub fn set(a: f64, b: f64, c: f64) -> Self {
        Self {
            albedo: Vec3::set(a, b, c),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = vec3_add(&rec.normal, &random_unit_vector());
        if scatter_direction.near_zero() {
            scatter_direction.copy_vector(&rec.normal);
        }
        scattered.copy_ray(&Ray::set(rec.p, scatter_direction));
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
        scattered.copy_ray(&Ray::set(rec.p, reflected));
        attenuation.copy_vector(&self.albedo);
        vec3_dot(&scattered.direction, &rec.normal) > 0.0
    }
}

#[derive(Copy, Clone)]
pub enum Materials {
    MetalMaterials(Metal),
    LambertianMaterials(Lambertian),
}

impl Material for Materials {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        // if let Materials::MetalMaterials(m) | Materials::LambertianMaterials(m) = self {
        // 	m.scatter(r_in, rec, attenuation, scattered)
        // } else {
        // 	false
        // }
        match self {
            Materials::LambertianMaterials(m) => m.scatter(r_in, rec, attenuation, scattered),
            Materials::MetalMaterials(m) => m.scatter(r_in, rec, attenuation, scattered),
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
        }
    }
}
