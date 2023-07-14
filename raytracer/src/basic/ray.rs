use super::vec3::{Vec3, vec3_mul, vec3_add};

#[derive(Copy, Clone)]
pub struct Ray {
	pub direction: Vec3,
	pub origin: Vec3,
}

impl Ray {
	pub fn set(origin: Vec3, direction: Vec3) -> Self{
		Ray {
			direction, //: direction.clone(),
			origin, //: origin.clone(),
		}
	}

	pub fn point_at_parameter(&self, t: &f64) -> Vec3 {
		// self.origin + (self.direction * t)
		vec3_add(&self.origin, &vec3_mul(t, &self.direction))
	}

	pub fn copy_ray(&mut self, other: &Ray) -> &Self {
		[self.direction, self.origin] = [other.direction, other.origin];
		self
	}
	pub fn set_value(&mut self, origin: Vec3, direction: Vec3) -> &Self{
		[self.direction, self.origin] = [direction, origin];
		self
	}
}

impl Default for Ray {
	fn default() -> Self {
		Self {
			direction: Vec3::set(0.0, 0.0, 0.0),
			origin: Vec3::set(0.0, 0.0, 0.0),
		}
	}
}