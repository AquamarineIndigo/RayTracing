use super::basic::ray;
use super::basic::vec3;
use crate::object::material::{/*Lambertian,*/ Materials};

#[derive(Copy, Clone)]
pub struct HitRecord {
	pub t: f64,
	pub p: vec3::Vec3,
	pub normal: vec3::Vec3,
	pub front_face: bool,
	pub material: Materials,
}

impl HitRecord {
	pub fn new(mat: &Materials) -> HitRecord {
		HitRecord {
			t: 0.0, 
			p: vec3::Vec3::set(0.0, 0.0, 0.0), 
			normal: vec3::Vec3::set(0.0, 0.0, 0.0), 
			front_face: false,
			material: *mat,
		}
	}
	pub fn set(t_other: f64, p_other: vec3::Vec3, normal_other: vec3::Vec3, mat: &Materials) -> HitRecord{
		HitRecord {
			t: t_other,
			p: p_other,
			normal: normal_other,
			front_face: false,
			material: *mat,
		}
	}
	pub fn get_value(&mut self, t_other: f64, p_other: vec3::Vec3, normal_other: vec3::Vec3, mat: &Materials) -> &Self {
		self.t = t_other;
		self.p = p_other;
		self.normal = normal_other;
		self.material = *mat;
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

pub trait Hittable {
	fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool;
}


// impl Default for HitRecord {
// 	fn default() -> Self {
// 		Self::new()
// 	}
// }