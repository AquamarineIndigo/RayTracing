use super::vec3::{Vec3, vec3_sub, vec3_tri_add, vec3_mul};
use super::ray::Ray;
use super::clamp;

pub struct Camera {
	origin: Vec3,
	lower_left_corner: Vec3,
	horizontal: Vec3,
	vertical: Vec3,
}

impl Camera {
	pub fn new() -> Self {
		let aspect_ratio = 16.0 / 9.0;
		let viewpoint_height = 2.0;
		let viewpoint_width = aspect_ratio * viewpoint_height;
		let focal_length = 1.0;
		let origin_ = Vec3::set(0.0, 0.0, 0.0);
		let horizontal_ = Vec3::set(viewpoint_width.clone(), 0.0, 0.0);
		let vertical_ = Vec3::set(0.0, viewpoint_height.clone(), 0.0);
		Camera {
			origin: origin_,
			horizontal: horizontal_,
			vertical: vertical_,
			lower_left_corner: vec3_sub(
				&origin_, 
				&vec3_tri_add(&vec3_mul(&0.5, &horizontal_), &vec3_mul(&0.5, &vertical_), &Vec3::set(0.0, 0.0, focal_length))
			),
		}
	}
	pub fn get_ray(&self, u: &f64, v: &f64) -> Ray {
		Ray::set(
			self.origin.clone(), 
			vec3_tri_add(&self.lower_left_corner, &vec3_mul(&u, &self.horizontal), &vec3_mul(&v, &self.vertical)).clone(),
		)
	}
}

pub fn write_colour(pixel_colour: &Vec3, sample_per_pixel: &i32) -> Vec<u8> {
	let scale: f64 = 1.0 / (*sample_per_pixel as f64);
	let lr: f64 = pixel_colour.x_dir * scale;
	let lg: f64 = pixel_colour.y_dir * scale;
	let lb: f64 = pixel_colour.z_dir * scale;
	// lr *= scale;
	// lg *= scale;
	// lb *= scale;
	return vec![
		(clamp(&lr, &0.0, &0.999) * 256.0) as u8, 
		(clamp(&lg, &0.0, &0.999) * 256.0) as u8, 
		(clamp(&lb, &0.0, &0.999) * 256.0) as u8
	];
}