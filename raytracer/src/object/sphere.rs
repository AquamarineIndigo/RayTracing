use crate::basic::ray;// as ray;
use crate::basic::vec3;// as vector3;
use crate::object::hittable;// as hitable;

#[derive(Copy, Clone)]
pub struct Sphere {
	pub center: vec3::Vec3,
	pub radius: f64,
}

impl Sphere {
	pub fn set(cen: vec3::Vec3, r: f64) -> Sphere {
		Sphere {
			center: cen, 
			radius: r,
		}
	}
}

impl hittable::Hittable for Sphere {
	fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut hittable::HitRecord) -> bool {
		let oc = vec3::vec3_sub(&r.origin, &self.center);
		let a: f64 = vec3::vec3_dot(&r.direction, &r.direction);
		let half_b: f64 = vec3::vec3_dot(&oc, &r.direction); // b -> half_b
		let c: f64 = vec3::vec3_dot(&oc, &oc) - self.radius*self.radius;
		let discriminant: f64 = half_b*half_b - a*c; 
		if discriminant < 0.0 {
			return false;
		}
		let sqrt_d = (half_b*half_b - a*c).sqrt();
		let mut root: f64 = (-half_b - sqrt_d) / a;
		if root < *t_min || *t_max < root {
			root = (-half_b + sqrt_d) / a;
			if root < *t_min || *t_max < root {
				return false;
			}
		}
		let point_at = r.point_at_parameter(&root);
		rec.get_value(
			root, 
			point_at.clone(), 
			vec3::vec3_mul(
				&(1.0 / self.radius), 
				&vec3::vec3_sub(&point_at, &self.center)
			)
		);
		let outward_normal: vec3::Vec3 = vec3::vec3_mul(&(1.0 / self.radius), &vec3::vec3_sub(&point_at, &self.center));
		rec.set_face_normal(&r, &outward_normal);
		return true;
	}
}
