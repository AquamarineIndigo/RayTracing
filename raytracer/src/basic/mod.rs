pub mod vec3;
pub mod ray;
pub mod camera;
// pub mod utility;
pub use vec3::Vec3;
pub use ray::Ray;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: &f64) -> f64 {
	return degrees * PI / 180.0;
}

pub use rand::prelude::*;
pub fn random_double() -> f64 {
	let mut rng = rand::thread_rng();
	let r = rng.gen_range(0.0..1.0);
	return r;
}

pub fn random_range(min: f64, max: f64) -> f64 {
	min + (max - min) * random_double()
}

pub fn clamp(x: &f64, min: &f64, max: &f64) -> f64 {
	if x < min {
		return *min;
	}
	if x > max {
		return *max;
	}
	return *x;
}