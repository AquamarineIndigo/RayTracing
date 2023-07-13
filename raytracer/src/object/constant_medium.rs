use super::hittable::{HitRecord, Hittable};
use super::material::Isotropic;
use super::{AxisAlignedBoundingBoxes, Lambertian, Material, Texture};
use crate::basic::{random_double, Vec3, INFINITY};
use std::f64::consts::E;
use std::marker::{Send, Sync};
use std::sync::Arc;

#[derive(Clone)]
pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable + Send + Sync>,
    pub phase_function: Arc<dyn Material + Send + Sync>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new_from_textures<T: Texture + Send + Sync + Clone + 'static>(
        b: Arc<dyn Hittable + Send + Sync>,
        d: f64,
        a: &T,
    ) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new_from_textures(a)),
        }
    }
    pub fn new_from_colour(b: Arc<dyn Hittable + Send + Sync>, d: f64, a: &Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Lambertian::new_from_vector(a)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::basic::ray::Ray,
        t_min: &f64,
        t_max: &f64,
        rec: &mut HitRecord,
    ) -> bool {
        let mut rec1 = HitRecord::new(&Lambertian::set(0.0, 0.0, 0.0));
        let mut rec2 = HitRecord::new(&Lambertian::set(0.0, 0.0, 0.0));
        if !self.boundary.hit(r, &-INFINITY, &INFINITY, &mut rec1) {
            return false;
        }
        if !self
            .boundary
            .hit(r, &(rec1.t + 0.0001), &INFINITY, &mut rec2)
        {
            return false;
        }
        if rec1.t < *t_min {
            rec1.t = *t_min;
        }
        if rec2.t > *t_max {
            rec2.t = *t_max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().log(E);
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.get_value_ptr(
            rec1.t + hit_distance / ray_length,
            r.point_at_parameter(rec.t),
            Vec3::set(1.0, 0.0, 0.0),
            self.phase_function.clone(),
        );
        rec.front_face = true;
        true
    }

    fn bounding_box(
        &self,
        time0: f64,
        time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
