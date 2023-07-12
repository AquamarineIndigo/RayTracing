use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::AxisAlignedBoundingBoxes;
use crate::basic::ray::Ray;
use crate::basic::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct XYRect {
    pub mp: Arc<dyn Material + Send + Sync>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl XYRect {
    pub fn new<T: Material + Clone + 'static + Send + Sync>(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        mat: &T,
    ) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp: Arc::new(mat.clone()),
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin.z_dir) / r.direction.z_dir;
        if t < *t_min || t > *t_max {
            return false;
        }
        let x = r.origin.x_dir + t * r.direction.x_dir;
        let y = r.origin.y_dir + t * r.direction.y_dir;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        let outward_normal = Vec3::set(0.0, 0.0, 1.0);
        rec.set_with_ptr(
            t,
            r.point_at_parameter(t),
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            Arc::clone(&self.mp),
        );
        rec.set_face_normal(r, &outward_normal);
        // rec.material = self.mp.clone();
        true
    }
    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool {
        output_box.set(
            Vec3::set(self.x0, self.y0, self.k - 0.0001),
            Vec3::set(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}
