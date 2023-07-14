use super::material::Material;
use super::{AxisAlignedBoundingBoxes, HitRecord, Hittable};
use crate::basic::ray::Ray;
use crate::basic::vec3::{generate_unit_vector, vec3_dot, vec3_product, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct Triangle {
    pub mat_ptr: Arc<dyn Material + Send + Sync>,
    pub a: Vec3,
    pub vab: Vec3,
    pub vac: Vec3,
    pub b_inner_perp: Vec3,
    pub c_inner_perp: Vec3,
    pub normal: Vec3,
    pub bbox: AxisAlignedBoundingBoxes,
}

impl Triangle {
    pub fn new(
        a: Vec3,
        b: Vec3,
        c: Vec3,
        mat_ptr: Arc<dyn Material + Send + Sync + 'static>,
    ) -> Self {
        let vab = b - a;
        let vac = c - a;
        let prod_normal = vec3_product(&vab, &vac);
        let nl = prod_normal.length();
        let normal = generate_unit_vector(&prod_normal);
        let mut min = Vec3::set(0.0, 0.0, 0.0);
        let mut max = Vec3::set(0.0, 0.0, 0.0);
        for i in 0..3 {
            min[i] = a[i].min(b[i].min(c[i]));
            max[i] = a[i].max(b[i].max(c[i]));
        }
        Self {
            mat_ptr,
            a,
            vab,
            vac,
            normal,
            b_inner_perp: vec3_product(&normal, &vab) / nl,
            c_inner_perp: vec3_product(&vac, &normal) / nl,
            bbox: AxisAlignedBoundingBoxes::new(min, max),
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        if vec3_dot(&r.direction, &self.normal).abs() < 0.00001 {
            return false;
        }
        let voa = r.origin - self.a;
        let t = -vec3_dot(&voa, &self.normal) / vec3_dot(&r.direction, &self.normal);
        if t < *t_min || t > *t_max {
            return false;
        }
        let pt = r.point_at_parameter(t);
        let vap = pt - self.a;
        let u = vec3_dot(&self.b_inner_perp, &vap);
        let v = vec3_dot(&self.c_inner_perp, &vap);
        if (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v) && (0.0..=1.0).contains(&(u + v)) {
            rec.set_with_ptr(t, pt, u, v, Arc::clone(&self.mat_ptr));
            rec.set_face_normal(r, &self.normal);
            true
        } else {
            false
        }
    }
    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool {
        output_box.copy_from_other(&self.bbox);
        true
    }
}
