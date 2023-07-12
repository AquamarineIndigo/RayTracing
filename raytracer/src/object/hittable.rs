use super::basic::ray;
use super::basic::vec3;
use crate::object::aabb::AxisAlignedBoundingBoxes;
use crate::object::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new<T: Material + Clone + 'static>(mat: &T) -> Self {
        HitRecord {
            t: 0.0,
            p: vec3::Vec3::set(0.0, 0.0, 0.0),
            normal: vec3::Vec3::set(0.0, 0.0, 0.0),
            front_face: false,
            material: Arc::new(mat.clone()),
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn set<T: Material + Clone + 'static>(
        t_other: f64,
        p_other: vec3::Vec3,
        normal_other: vec3::Vec3,
        mat: &T,
    ) -> Self {
        HitRecord {
            t: t_other,
            p: p_other,
            normal: normal_other,
            front_face: false,
            material: Arc::new(mat.clone()),
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn get_value<T: Material + Clone + 'static>(
        &mut self,
        t_other: f64,
        p_other: vec3::Vec3,
        normal_other: vec3::Vec3,
        mat: &T,
    ) -> &Self {
        self.t = t_other;
        self.p = p_other;
        self.normal = normal_other;
        self.material = Arc::new(mat.clone());
        self
    }
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vec3::Vec3) {
        if vec3::vec3_dot(&r.direction, outward_normal) < 0.0 {
            self.front_face = true;
            self.normal = *outward_normal;
        } else {
            self.front_face = false;
            self.normal = vec3::vec3_mul(&(-1.0), outward_normal);
        }
    }
    // pub fn clone(&self) -> Self {
    // 	HitRecord {
    // 		t: self.t.clone(),
    // 		p: self.p.clone(),
    // 		normal: self.normal.clone(),
    // 		front_face: self.front_face.clone(),
    // 	}
    // }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(
        &self,
        time0: f64,
        time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool;
}

// impl Default for HitRecord {
// 	fn default() -> Self {
// 		Self::new()
// 	}
// }
