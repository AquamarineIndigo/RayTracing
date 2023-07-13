use crate::basic::Vec3;

use super::hittable::{HitRecord, Hittable};
use super::{HittableList, Material, XYRect, XZRect, YZRect};
// use std::sync::Arc;
use std::marker::{Send, Sync};

#[derive(Clone)]
pub struct Boxes {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
}

impl Boxes {
    pub fn new<T: Material + Send + Sync + Clone + 'static>(p0: &Vec3, p1: &Vec3, mat: &T) -> Self {
        let mut sides = HittableList {
            objects: Vec::new(),
        };
        sides.add(XYRect::new(
            p0.x_dir, p1.x_dir, p0.y_dir, p1.y_dir, p1.z_dir, mat,
        ));
        sides.add(XYRect::new(
            p0.x_dir, p1.x_dir, p0.y_dir, p1.y_dir, p0.z_dir, mat,
        ));

        sides.add(XZRect::new(
            p0.x_dir, p1.x_dir, p0.z_dir, p1.z_dir, p1.y_dir, mat,
        ));
        sides.add(XZRect::new(
            p0.x_dir, p1.x_dir, p0.z_dir, p1.z_dir, p0.y_dir, mat,
        ));

        sides.add(YZRect::new(
            p0.y_dir, p1.y_dir, p0.z_dir, p1.z_dir, p1.x_dir, mat,
        ));
        sides.add(YZRect::new(
            p0.y_dir, p1.y_dir, p0.z_dir, p1.z_dir, p0.x_dir, mat,
        ));

        Self {
            box_min: *p0,
            box_max: *p1,
            sides,
        }
    }
}

impl Hittable for Boxes {
    fn hit(
        &self,
        r: &crate::basic::ray::Ray,
        t_min: &f64,
        t_max: &f64,
        rec: &mut HitRecord,
    ) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }
    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
        output_box: &mut super::AxisAlignedBoundingBoxes,
    ) -> bool {
        output_box.set(self.box_min, self.box_max);
        true
    }
}
