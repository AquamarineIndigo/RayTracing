pub mod basic;
pub mod object;
use basic::vec3::{Vec3, vec3_add, vec3_mul,/* vec3_sub,vec3_dot, vec3_tri_add, random_unit_vector,*/  generate_unit_vector};
use basic::ray::Ray;
use object::Textures;
// use object::SolidColour;
use object::basic::vec3::vec3_vec_mul;
use object::basic::random_double;
// use object::basic::random_range;
use object::hittable::{HitRecord, Hittable};
use object::sphere::Sphere;
// use object::sphere::MovingSphere;
use object::hittable_list::{HittableList, Objects};
use console::style;
use image::{ImageBuffer, RgbImage};
// use rand::random;
// extern crate rayon;
// use rayon::prelude::*;
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};
// use object::texture::CheckeredTexture;
use object::texture::NoiseTexture;
use std::{fs::File, process::exit};
use crate::object::material::{Material, Lambertian, Metal, Materials/* , Dielectric*/};

use crate::basic::camera::{Camera, write_colour, TimeInterval, CameraCharacteristics};
use std::sync::Arc;
use std::thread;
use std::sync::mpsc;

// fn random_scene() -> HittableList {
// 	let mut world = HittableList { objects: Vec::new() };

// 	let checker = Textures::Checkered(CheckeredTexture::new(
// 		Textures::Solid(SolidColour::new_from_rgb(0.2, 0.3, 0.1)), 
// 		Textures::Solid(SolidColour::new_from_rgb(0.9, 0.9, 0.9)),
// 	));
// 	let mat_ground = Materials::LambertianMaterials(Lambertian::new_from_textures(&checker));
// 	world.add(Objects::SphereShape(Sphere::set(Vec3::set(0.0, -1000.0, 0.0), 1000.0, &mat_ground)));

// 	for a in -11..11 {
// 		for b in -11..11 {
// 			let choose_mat = random_double();
// 			let center = Vec3::set((a as f64) + 0.9* random_double(), 0.2, (b as f64) + 0.9 * random_double());
// 			if vec3_sub(&center, &Vec3::set(4.0, 0.2, 0.0)).length() > 0.9 {
// 				if choose_mat < 0.6 {
// 					// diffuse -> Lambertian
// 					let albedo = vec3_vec_mul(&Vec3::random_vector(), &Vec3::random_vector());
// 					let center2 = vec3_add(&center, &Vec3::set(0.0, random_range(0.0, 0.5), 0.0));
// 					let mat_sphere = Materials::LambertianMaterials(Lambertian::new_from_vector(&albedo));
// 					world.add(Objects::MovingSphere(
// 						MovingSphere::set(&center, &center2, 0.0, 1.0, 0.2, &mat_sphere))
// 					);
// 				} else if choose_mat < 0.9 {
// 					// metal
// 					let albedo = Vec3::random_vector_range(&0.5, &1.0);
// 					let fuzz = random_range(0.0, 0.5);
// 					let mat_sphere = Materials::MetalMaterials(Metal::new_from_vector(&albedo, fuzz));
// 					world.add(Objects::SphereShape(Sphere::set(center, 0.2, &mat_sphere)));
// 				} else {
// 					// glass -> Dielectric
// 					let mat_sphere = Materials::DielectricMaterials(Dielectric::set(1.5));
// 					world.add(Objects::SphereShape(Sphere::set(center, 0.2, &mat_sphere)));
// 					world.add(Objects::SphereShape(Sphere::set(center, -0.15, &mat_sphere))); // hollow
// 				}
// 			}
// 		}
// 	}
// 	let mat1 = Materials::DielectricMaterials(Dielectric::set(1.8));
// 	let mat2 = Materials::LambertianMaterials(Lambertian::set(0.4, 0.2, 0.1));
// 	let mat3 = Materials::MetalMaterials(Metal::set(0.7, 0.8, 0.7, 0.0));
// 	world.add(Objects::SphereShape(Sphere::set(Vec3::set(0.0, 1.0, 0.0), 1.0, &mat1)));
// 	// world.add(Objects::SphereShape(Sphere::set(Vec3::set(0.0, 1.0, 0.0), -0.8, &mat1))); // hollow
// 	world.add(Objects::SphereShape(Sphere::set(Vec3::set(-4.0, 1.0, 0.0), 1.0, &mat2)));
// 	world.add(Objects::SphereShape(Sphere::set(Vec3::set(4.0, 1.0, 0.0), 1.0, &mat3)));
// 	world
// }

fn two_perlin_spheres() -> HittableList {
	let mut world = HittableList { objects: Vec::new() };
	let pertext = Textures::Noise(Box::new(NoiseTexture::new()));
	let mat = Materials::LambertianMaterials(Box::new(Lambertian::new_from_textures(&pertext)));
	world.add(Objects::SphereShape(Sphere::set(Vec3::set(0.0, -1000.0, 0.0), 1000.0, &mat)));
	world.add(Objects::SphereShape(Sphere::set(Vec3::set(0.0, 2.0, 0.0), 2.0, &mat)));
	world
}

fn get_colour(r: &Ray, world: &HittableList, depth: &i32) -> Vec3 {
	if *depth < 0 {
		return Vec3::set(0.0, 0.0, 0.0);
	}
	let mut rec = HitRecord::new(&Materials::MetalMaterials(Metal::set(0.0, 0.0, 0.0, 0.0)));
	if world.hit(r, &0.001, &basic::INFINITY, &mut rec) {
		// let target: Vec3 = vec3_tri_add(&rec.p, &rec.normal, &random_unit_vector());
		// return vec3_mul(&0.5, &vec3_add(&rec.normal, &Vec3::set(1.0, 1.0, 1.0)));
		let mut scattered = Ray::default();
		let mut attenuation = Vec3::set(0.0, 0.0, 0.0);
		if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
			return vec3_vec_mul(&attenuation, &get_colour(&scattered, world, &(depth - 1)));
		} else {
			return Vec3::set(0.0, 0.0, 0.0);
		}
	}
	let unit_direction = generate_unit_vector(&r.direction);
	let t: f64 = 0.5 * (unit_direction.y_dir + 1.0);
	vec3_add(&vec3_mul(&(1.0 - t), &Vec3::set(1.0, 1.0, 1.0)), &vec3_mul(&t, &Vec3::set(0.5, 0.7, 1.0)))
}

fn get_id(i: &u32, j: &u32, width: &u32) -> usize {
	(j * width + i) as usize
}

fn main() {
	let path = "output/book2/image2-4.jpg";
	// let width: u32 = 800;
	const WIDTH: u32 = 1280;
	let quality = 255;
	// let aspect_ratio: f64 = 16.0 / 9.0;
	const ASPECTRATIO: f64 = 16.0 / 9.0;
	// let height: u32 = ((width as f64) / aspect_ratio) as u32;
	const HEIGHT: u32 = ((WIDTH as f64) / ASPECTRATIO) as u32;
	let sample_per_pixel = 50;
	let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
	let max_depth = 50;

	// let world = random_scene();
	// let look_from = Vec3::set(13.0, 2.0, 3.0);
	// let look_at = Vec3::set(0.0, 0.0, 0.0);
	// let vup = Vec3::set(0.0, 1.0, 0.0);
	// const APERTURE: f64 = 0.1;
	// let dist_to_focus = 10.0;
	// // let dist_to_focus = vec3_sub(&look_from, &look_at).length();

	// let cam: Camera = Camera::new(
	// 	&look_from, &look_at, &vup, 
	// 	CameraCharacteristics::new(20.0, ASPECTRATIO, APERTURE, dist_to_focus), 
	// 	TimeInterval::new(0.0, 1.0),
	// );
	
	let world = two_perlin_spheres();
	let look_from = Vec3::set(13.0, 2.0, 3.0);
	let look_at = Vec3::set(0.0, 0.0, 0.0);
	let vup = Vec3::set(0.0, 1.0, 0.0);
	const APERTURE: f64 = 0.0;
	let dist_to_focus = 10.0;
	// let dist_to_focus = vec3_sub(&look_from, &look_at).length();

	let cam: Camera = Camera::new(
		&look_from, &look_at, &vup, 
		CameraCharacteristics::new(20.0, ASPECTRATIO, APERTURE, dist_to_focus), 
		TimeInterval::new(0.0, 1.0),
	);

	const THREAD_NUMBER: i32 = 10;
	const SECTION_LINE_NUM: i32 = (HEIGHT as i32) / THREAD_NUMBER;

	// Progress bar
	let multiprogress = Arc::new(MultiProgress::new());
	multiprogress.set_move_cursor(true);

	// Thread
	let mut output_pixel_color = Vec::<Vec<u8>>::new();
	let mut thread_pool = Vec::<_>::new();

	for thread_id in 0..THREAD_NUMBER {
		// line
		// let line_id = random_line_id;
		let row_begin = thread_id * SECTION_LINE_NUM;
		let mut row_end = row_begin + SECTION_LINE_NUM;
		if thread_id == THREAD_NUMBER - 1 {
			row_end = HEIGHT as i32;
		}
		let section_world = world.clone();

		//progress
		let mp = multiprogress.clone();
		let progress_bar = mp.add(ProgressBar::new((row_end - row_begin) as u64));
		progress_bar.set_style(
			ProgressStyle::default_bar()
				.template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
				.progress_chars("=> ")
		);
	
		// thread code
		let (tx, rx) = mpsc::channel();
	
		thread_pool.push((
			thread::spawn(move || {
				let mut progress = 0;
				progress_bar.set_position(progress);
				let mut section_pixel_color = Vec::<Vec<u8>>::new();
				for j in row_begin..row_end {
					for i in 0..WIDTH {
						let mut pixel_colour = Vec3::set(0.0, 0.0, 0.0);
						for _s in 0..sample_per_pixel {
							let u = (i as f64 + random_double()) / ((WIDTH - 1) as f64);
							let v = (j as f64 + random_double()) / ((HEIGHT - 1) as f64);
							let r: Ray = cam.get_ray(&u, &v);
							pixel_colour = vec3_add(&pixel_colour, &get_colour(&r, &section_world.clone(), &max_depth));
						}
						let mut arr = vec![0, 0, 0];
						arr[..].copy_from_slice(&write_colour(&pixel_colour, &sample_per_pixel)[..3]);
						section_pixel_color.push(arr);
					}
					progress += 1;
					progress_bar.set_position(progress);
				}
				tx.send(section_pixel_color).unwrap();
				progress_bar.finish_with_message("Finished.");
			}),
			rx,
		));
	}
	multiprogress.join().unwrap();

	for _thread_id in 0..THREAD_NUMBER {
		let thread = thread_pool.remove(0);
		match thread.0.join() {
			Ok(_) => {
				let mut received = thread.1.recv().unwrap();
				output_pixel_color.append(&mut received);
			}
			Err(_) => {
				println!("Thread Failed to Parallel");
				exit(0);
			}
		}
	}

	for j in 0..HEIGHT {
		for i in 0..WIDTH {
			*img.get_pixel_mut(i, HEIGHT - j - 1) = 
				image::Rgb([
					output_pixel_color[get_id(&i, &j, &WIDTH)][0], 
					output_pixel_color[get_id(&i, &j, &WIDTH)][1], 
					output_pixel_color[get_id(&i, &j, &WIDTH)][2]
				]);
		}
	}

	println!("Output image is in \"{}\"", style(path).green());
	let output_image = image::DynamicImage::ImageRgb8(img);
	let mut output_file = File::create(path).unwrap();
	match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
		Ok(_) => {}
		Err(_) => println!("{}", style("Output image failed").red()),
	}
	// drop(pixel_rgb);
	exit(0);
}

// https://raytracing.github.io/