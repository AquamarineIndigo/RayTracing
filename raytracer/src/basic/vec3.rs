use super::{random_double, random_range};

#[derive(Copy, Clone)]
pub struct Vec3 {
	pub x_dir: f64,
	pub y_dir: f64,
	pub z_dir: f64,
}

pub fn vec3_add(a: &Vec3, b: &Vec3) -> Vec3 {
	Vec3 {
		x_dir: a.x_dir + b.x_dir,
		y_dir: a.y_dir + b.y_dir,
		z_dir: a.z_dir + b.z_dir,
	}
}
pub fn vec3_mul(a: &f64, b: &Vec3) -> Vec3 {
	Vec3 {
		x_dir: a * b.x_dir,
		y_dir: a * b.y_dir,
		z_dir: a * b.z_dir,
	}
}
pub fn vec3_tri_add(a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3 {
	Vec3 {
		x_dir: a.x_dir + b.x_dir + c.x_dir,
		y_dir: a.y_dir + b.y_dir + c.y_dir,
		z_dir: a.z_dir + b.z_dir + c.z_dir,
	}
}
pub fn vec3_sub(a: &Vec3, b: &Vec3) -> Vec3 {
	Vec3 {
		x_dir: a.x_dir - b.x_dir,
		y_dir: a.y_dir - b.y_dir,
		z_dir: a.z_dir - b.z_dir,
	}
}
pub fn vec3_dot(a: &Vec3, b: &Vec3) -> f64 {
	(a.x_dir * b.x_dir) + (a.y_dir * b.y_dir) + (a.z_dir * b.z_dir)
}

impl Vec3 {
	pub fn set(x: f64, y: f64, z: f64) -> Vec3 {
		Vec3 {
			x_dir: x,
			y_dir: y,
			z_dir: z,
		}
	}
	pub fn clone(self: &Self) -> Vec3 {
		Vec3 {
			x_dir: self.x_dir,
			y_dir: self.y_dir,
			z_dir: self.z_dir,
		}
	}
	pub fn random_vector() -> Vec3 {
		Vec3::set(
			random_double(), random_double(), random_double()
		)
	}
	pub fn random_vector_range(min: &f64, max: &f64) -> Vec3 {
		Vec3::set(
			random_range(*min, *max), random_range(*min, *max), random_range(*min, *max)
		)
	}
}

pub fn generate_unit_vector(direction: &Vec3) -> Vec3 {
	let l_sqr = direction.x_dir.powi(2) + direction.y_dir.powi(2) + direction.z_dir.powi(2);
	let l = l_sqr.sqrt();
	Vec3 {
		x_dir: direction.x_dir / l,
		y_dir: direction.y_dir / l,
		z_dir: direction.z_dir / l,
	}
}

pub fn random_in_unit_sphere() -> Vec3 {
	loop {
		let p = Vec3::random_vector();
		if vec3_dot(&p, &p) >= 1.0 {
			continue;
		}
		return p;
	}
}