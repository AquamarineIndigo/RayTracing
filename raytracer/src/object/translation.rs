use super::hittable::{HitRecord, Hittable};
use super::AxisAlignedBoundingBoxes;
use crate::basic::{Ray, Vec3, INFINITY};
use std::marker::{Send, Sync};
use std::sync::Arc;

#[derive(Clone)]
pub struct Translate {
    pub ptr: Arc<dyn Hittable + Send + Sync>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(ptr: Arc<dyn Hittable + 'static + Send + Sync>, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::set_with_time(r.origin - self.offset, r.direction, r.tm);
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }
        rec.p += self.offset;
        let normal = rec.normal;
        rec.set_face_normal(&moved_r, &normal);
        true
    }
    fn bounding_box(
        &self,
        time0: f64,
        time1: f64,
        output_box: &mut super::AxisAlignedBoundingBoxes,
    ) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }
        output_box.set(
            output_box.minimum + self.offset,
            output_box.maximum + self.offset,
        );
        true
    }
}

pub struct RotateY {
    pub ptr: Arc<dyn Hittable + Send + Sync>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AxisAlignedBoundingBoxes,
}

impl RotateY {
    pub fn new(ptr: Arc<dyn Hittable + 'static + Send + Sync>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        // println!("{angle} -> cos: {cos_theta}, sin: {sin_theta}");
        let mut min = Vec3::set(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::set(-INFINITY, -INFINITY, -INFINITY);
        let mut bbox = AxisAlignedBoundingBoxes::default();
        let hasbox = ptr.bounding_box(0.0, 1.0, &mut bbox);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.maximum.x_dir + ((1 - i) as f64) * bbox.minimum.x_dir;
                    let y = (j as f64) * bbox.maximum.y_dir + ((1 - j) as f64) * bbox.minimum.y_dir;
                    let z = (k as f64) * bbox.maximum.z_dir + ((1 - k) as f64) * bbox.minimum.z_dir;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = cos_theta * z - sin_theta * x;
                    let tester = Vec3::set(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        Self {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox: AxisAlignedBoundingBoxes::new(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool {
        output_box.copy_from_other(&self.bbox);
        self.hasbox
    }
    fn hit(
        &self,
        r: &crate::basic::ray::Ray,
        t_min: &f64,
        t_max: &f64,
        rec: &mut HitRecord,
    ) -> bool {
        let mut origin = r.origin;
        let mut direction = r.direction;
        origin[0] = self.cos_theta * r.origin[0] - self.sin_theta * r.origin[2];
        origin[2] = self.sin_theta * r.origin[0] + self.cos_theta * r.origin[2];
        direction[0] = self.cos_theta * r.direction[0] - self.sin_theta * r.direction[2];
        direction[2] = self.sin_theta * r.direction[0] + self.cos_theta * r.direction[2];
        let rotated = Ray::set_with_time(origin, direction, r.tm);

        if !self.ptr.hit(&rotated, t_min, t_max, rec) {
            return false;
        }
        let mut p = rec.p;
        let mut normal = rec.normal;
        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = self.cos_theta * rec.p[2] - self.sin_theta * rec.p[0];
        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = self.cos_theta * rec.normal[2] - self.sin_theta * rec.normal[0];
        rec.p.copy_vector(&p);
        rec.set_face_normal(&rotated, &normal);
        true
    }
}
