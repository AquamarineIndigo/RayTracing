use super::vec3 as vector3;

#[derive(Copy, Clone)]
pub struct Ray {
	pub direction: vector3::Vec3,
	pub origin: vector3::Vec3,
}

impl Ray {
	pub fn set(origin: vector3::Vec3, direction: vector3::Vec3) -> Self{
		Ray {
			direction, //: direction.clone(),
			origin, //: origin.clone(),
		}
	}

	pub fn point_at_parameter(self: &Self, t: &f64) -> vector3::Vec3 {
		// self.origin + (self.direction * t)
		vector3::vec3_add(&self.origin, &vector3::vec3_mul(&t, &self.direction))
	}
}