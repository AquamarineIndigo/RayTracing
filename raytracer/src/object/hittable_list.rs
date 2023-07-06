use crate::basic::ray;// as ray;
// use crate::basic::vec3;// as vector3;
use crate::object::hittable::{HitRecord, Hittable};// as hitable;
use crate::object::sphere;

#[derive(Clone)]
pub enum Objects {
	SphereShape(sphere::Sphere),
	List(HittableList),
}

#[derive(Clone)]
pub struct HittableList {
	pub objects: Vec<Objects>,
	// pub objects: vec!,
}

impl HittableList {
	// pub fn new() -> HittableList {
	// 	HittableList { objects: Vec::new() }
	// }
	pub fn add(&mut self, obj: Objects) {
		self.objects.push(obj);
	}
	pub fn clear(&mut self) {
		self.objects.clear();
	}
}

impl Hittable for HittableList {
	fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
		let mut temp_rec:HitRecord = HitRecord::new();
		let mut hit_anything = false;
		let mut closest_so_far: f64 = t_max.clone();

		for i in &self.objects {
			match i {
				Objects::SphereShape(s) => {
					if s.hit(&r, &t_min, &closest_so_far, &mut temp_rec) == true {
						hit_anything = true;
						closest_so_far = temp_rec.t;
						*rec = temp_rec.clone();
					}
				}
				_ => {}
			}
			
		}
		return hit_anything;
	}
}
