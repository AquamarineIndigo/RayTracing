pub mod basic;
pub mod object;
use basic::ray::Ray;
use basic::vec3::{
    /*vec3_sub,vec3_tri_add, random_unit_vector,*/ generate_unit_vector, vec3_add,
    /*vec3_dot,*/ vec3_mul, Vec3,
};
use object::basic::vec3::vec3_vec_mul;
// use object::basic::vec3::vec3_tri_add;
use console::style;
use image::{ImageBuffer, RgbImage};
use object::hittable::{HitRecord, Hittable};
use object::hittable_list::{HittableList, Objects};
use object::sphere::Sphere;
// use rand::random;
// extern crate rayon;
// use rayon::prelude::*;
use crate::object::material::{Lambertian, Material, Materials /* , Dielectric*/, Metal};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::f64::consts::PI;
use std::{fs::File, process::exit};

use crate::basic::camera::{write_colour, Camera};
use crate::basic::random_double;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

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
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return vec3_vec_mul(&attenuation, &get_colour(&scattered, world, &(depth - 1)));
        } else {
            return Vec3::set(0.0, 0.0, 0.0);
        }
    }
    let unit_direction = generate_unit_vector(&r.direction);
    let t: f64 = 0.5 * (unit_direction.y_dir + 1.0);
    vec3_add(
        &vec3_mul(&(1.0 - t), &Vec3::set(1.0, 1.0, 1.0)),
        &vec3_mul(&t, &Vec3::set(0.5, 0.7, 1.0)),
    )
}

// #[derive(Copy, Clone)]
// struct Mem {
// 	i: u32,
// 	j: u32,
// 	lr: u8,
// 	lg: u8,
// 	lb: u8,
// }

// impl Mem {
// 	fn new() -> Self {
// 		Mem { i: 0, j: 0, lr: 0, lg: 0, lb: 0 }
// 	}
// }

fn get_id(i: &u32, j: &u32, width: &u32) -> usize {
    (j * width + i) as usize
}

fn main() {
    let path = "output/book1/image1-17.jpg";
    // let width: u32 = 800;
    const WIDTH: u32 = 1920;
    let quality = 255;
    // let aspect_ratio: f64 = 16.0 / 9.0;
    const ASPECTRATIO: f64 = 16.0 / 9.0;
    // let height: u32 = ((width as f64) / aspect_ratio) as u32;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECTRATIO) as u32;
    let sample_per_pixel = 100;
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let max_depth = 50;
    let r_ = (PI / 4.0).cos();

    let mut world = HittableList {
        objects: Vec::new(),
    };
    // let mat_ground = Materials::LambertianMaterials(Lambertian::set(0.8, 0.8, 0.0));
    // let mat_center = Materials::LambertianMaterials(Lambertian::set(0.1, 0.2, 0.5));
    // let mat_left = Materials::MetalMaterials(Metal::set(0.8, 0.8, 0.8, 0.3));
    let mat_right = Materials::LambertianMaterials(Lambertian::set(1.0, 0.0, 0.0));
    // let mat_center = Materials::DielectricMaterials(Dielectric::set(1.5));
    let mat_left = Materials::LambertianMaterials(Lambertian::set(0.0, 0.0, 1.0));

    // world.add(Objects::SphereShape(Sphere::set(Vec3::set(0.0, -100.5, -1.0), 100.0, &mat_ground)));
    // world.add(Objects::SphereShape(Sphere::set(Vec3::set(0.0, 0.0, -1.0), 0.5, &mat_center)));
    world.add(Objects::SphereShape(Sphere::set(
        Vec3::set(-r_, 0.0, -1.0),
        r_,
        &mat_left,
    )));
    // world.add(Objects::SphereShape(Sphere::set(Vec3::set(-1.0, 0.0, -1.0), -0.4, &mat_left))); // hollow
    world.add(Objects::SphereShape(Sphere::set(
        Vec3::set(r_, 0.0, -1.0),
        r_,
        &mat_right,
    )));

    let cam: Camera = Camera::new(90.0, ASPECTRATIO);

    // let mut pixel_rgb = Box::new([Mem::new(); ((HEIGHT * WIDTH) as usize)]);
    // for i in 0..WIDTH {
    // 	for j in 0..HEIGHT {
    // 		pixel_rgb[get_id(&i, &j, &WIDTH)].i = i;
    // 		pixel_rgb[get_id(&i, &j, &WIDTH)].j = j;
    // 	}
    // }
    // pixel_rgb.par_iter_mut()
    // 	.for_each(|p: &mut Mem| {
    // 		// let i: u32 = index % width;
    // 		// let j: u32 = index / width;
    // 		// let pixel = img.get_pixel_mut(i, height - j - 1);
    // 		let i: u32 = p.i;
    // 		let j: u32 = p.j;
    // 		let mut pixel_colour = Vec3::set(0.0, 0.0, 0.0);
    // 		for _s in 0..sample_per_pixel {
    // 			let u = (i as f64 + random_double()) / ((WIDTH - 1) as f64);
    // 			let v = (j as f64 + random_double()) / ((HEIGHT - 1) as f64);
    // 			let r: Ray = cam.get_ray(&u, &v);
    // 			pixel_colour = vec3_add(&pixel_colour, &get_colour(&r, &world.clone(), &max_depth));
    // 		}
    // 		let mut arr = [0; 3];
    // 		arr[..].copy_from_slice(&camera::write_colour(&pixel_colour, &sample_per_pixel)[..3]);
    // 		// *pixel = image::Rgb(arr);
    // 		// pixel_rgb[i as usize][j as usize][..].copy_from_slice(&camera::write_colour(&pixel_colour, &sample_per_pixel)[..3]);
    // 		// println!("Position: (x: {i}, y: {j})");
    // 		[p.lr, p.lg, p.lb] = arr;
    // 		progress.inc(1);
    // 	});
    // 	// .collect();

    // for i in 0..WIDTH {
    // 	for j in 0..HEIGHT {
    // 		*img.get_pixel_mut(i, HEIGHT - j - 1) =
    // 			image::Rgb([
    // 				pixel_rgb[get_id(&i, &j, &WIDTH)].lr,
    // 				pixel_rgb[get_id(&i, &j, &WIDTH)].lg,
    // 				pixel_rgb[get_id(&i, &j, &WIDTH)].lb
    // 			]);
    // 	}
    // }
    // progress.finish();

    const THREAD_NUMBER: i32 = 9;
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
                            pixel_colour = vec3_add(
                                &pixel_colour,
                                &get_colour(&r, &section_world.clone(), &max_depth),
                            );
                        }
                        let mut arr = vec![0, 0, 0];
                        arr[..]
                            .copy_from_slice(&write_colour(&pixel_colour, &sample_per_pixel)[..3]);
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
            *img.get_pixel_mut(i, HEIGHT - j - 1) = image::Rgb([
                output_pixel_color[get_id(&i, &j, &WIDTH)][0],
                output_pixel_color[get_id(&i, &j, &WIDTH)][1],
                output_pixel_color[get_id(&i, &j, &WIDTH)][2],
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
