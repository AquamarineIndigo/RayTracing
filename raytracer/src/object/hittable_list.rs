use crate::basic::ray; // as ray;
                       // use crate::basic::vec3;// as vector3;
use crate::object::hittable::{HitRecord, Hittable}; // as hitable;
                                                    // use crate::object::sphere;
use crate::object::material::Lambertian;
use std::sync::Arc;

use super::{surrounding_box, AxisAlignedBoundingBoxes};

// #[derive(Clone)]
// pub enum Objects  {
// 	SphereShape(sphere::Sphere),
// 	MovingSphere(sphere::MovingSphere),
// 	List(HittableList),
// }

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    // pub objects: vec!,
}

impl HittableList {
    // pub fn new() -> HittableList {
    // 	HittableList { objects: Vec::new() }
    // }
    pub fn add<T: Hittable + 'static + Send + Sync>(&mut self, obj: T) {
        self.objects.push(Arc::new(obj));
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new(&Lambertian::set(0.0, 0.0, 0.0));
        let mut hit_anything = false;
        let mut closest_so_far: f64 = *t_max;

        for i in self.objects.iter() {
            if i.hit(r, t_min, &closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.set_with_ptr(
                    temp_rec.t,
                    temp_rec.p,
                    temp_rec.u,
                    temp_rec.v,
                    Arc::clone(&temp_rec.material),
                );
                rec.front_face = temp_rec.front_face;
                rec.normal = temp_rec.normal;
                // rec.material = Arc::new(*((**i).material));
            }
        }
        hit_anything
    }

    fn bounding_box(
        &self,
        time0: f64,
        time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool {
        if self.empty() {
            return false;
        }
        let mut temp_box = AxisAlignedBoundingBoxes::default();
        let mut first_box = true;
        for i in self.objects.iter() {
            if !i.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            if first_box {
                output_box.copy_from_other(&temp_box);
                first_box = false;
            } else {
                output_box.copy_from_other(&surrounding_box(output_box, &temp_box));
            }
        }
        true
    }
}
