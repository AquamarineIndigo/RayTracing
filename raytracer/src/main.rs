pub mod basic;
pub mod object;
use basic::camera;
use basic::ray::Ray;
use basic::vec3::{
    generate_unit_vector, vec3_add, /*random_unit_vector, vec3_dot,*/ vec3_mul,
    /*vec3_sub, vec3_tri_add,*/ Vec3,
};
use object::basic::vec3::{vec3_sub, random_in_hemisphere};
// use object::basic::vec3::vec3_tri_add;
use console::style;
use image::{ImageBuffer, RgbImage};
use object::hittable::{HitRecord, Hittable};
use object::hittable_list::{HittableList, Objects};
use object::sphere::Sphere;
// use rand::random;
extern crate rayon;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::{fs::File, process::exit};

use crate::basic::camera::Camera;
use crate::basic::random_double;

// fn get_colour(r: &Ray, world: &Objects) -> Vec3 {
// 	let mut rec = HitRecord::new();
// 	match world {
// 		Objects::SphereShape(w) => {
// 			if w.hit(&r, &0.0, &basic::INFINITY, &mut rec) == true {
// 				return vec3_mul(&0.5, &vec3_add(&rec.normal, &Vec3::set(1.0, 1.0, 1.0)));
// 			}
// 		}
// 		Objects::List(w) => {
// 			if w.hit(&r, &0.0, &basic::INFINITY, &mut rec) == true {
// 				return vec3_mul(&0.5, &vec3_add(&rec.normal, &Vec3::set(1.0, 1.0, 1.0)));
// 			}
// 		}
// 	}
// 	let unit_direction = generate_unit_vector(&r.direction);
// 	let t: f64 = 0.5 * (unit_direction.y_dir + 1.0);
// 	vec3_add(&vec3_mul(&(1.0 - t), &Vec3::set(1.0, 1.0, 1.0)), &vec3_mul(&t, &Vec3::set(0.5, 0.7, 1.0)))
// }
fn get_colour(r: &Ray, world: &HittableList, depth: &i32) -> Vec3 {
    if *depth < 0 {
        return Vec3::set(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new();
    if world.hit(r, &0.0, &basic::INFINITY, &mut rec) {
        // let target: Vec3 = vec3_tri_add(&rec.p, &rec.normal, &random_unit_vector());
	let target: Vec3 = vec3_add(&rec.p, &random_in_hemisphere(&rec.normal));
        // return vec3_mul(&0.5, &vec3_add(&rec.normal, &Vec3::set(1.0, 1.0, 1.0)));
        return vec3_mul(
            &0.5,
            &get_colour(
                &Ray::set(rec.p, vec3_sub(&target, &rec.p)),
                world,
                &(depth - 1),
            ),
        );
    }
    let unit_direction = generate_unit_vector(&r.direction);
    let t: f64 = 0.5 * (unit_direction.y_dir + 1.0);
    vec3_add(
        &vec3_mul(&(1.0 - t), &Vec3::set(1.0, 1.0, 1.0)),
        &vec3_mul(&t, &Vec3::set(0.5, 0.7, 1.0)),
    )
}

#[derive(Copy, Clone)]
struct Mem {
    i: u32,
    j: u32,
    lr: u8,
    lg: u8,
    lb: u8,
}

impl Mem {
    fn new() -> Self {
        Mem {
            i: 0,
            j: 0,
            lr: 0,
            lg: 0,
            lb: 0,
        }
    }
}

fn get_id(i: &u32, j: &u32, width: &u32) -> usize {
    (j * width + i) as usize
}

fn main() {
    let path = "output/book1/image1-10.jpg";
    // let width: u32 = 800;
    const WIDTH: u32 = 1024;
    let quality = 255;
    // let aspect_ratio: f64 = 16.0 / 9.0;
    const ASPECTRATIO: f64 = 16.0 / 9.0;
    // let height: u32 = ((width as f64) / aspect_ratio) as u32;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECTRATIO) as u32;
    let sample_per_pixel = 100;
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let max_depth = 50;

    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.add(Objects::SphereShape(Sphere {
        center: Vec3::set(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Objects::SphereShape(Sphere {
        center: Vec3::set(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((HEIGHT * WIDTH) as u64)
    };

    let cam: Camera = Camera::new();

    // for j in 0..height { // rev?
    // 	for i in 0..width {
    // 		let pixel = img.get_pixel_mut(i, height - j - 1);
    // 		let mut pixel_colour = Vec3::set(0.0, 0.0, 0.0);
    // 		for _s in 0..sample_per_pixel {
    // 			let u = (i as f64 + random_double()) / ((width - 1) as f64);
    // 			let v = (j as f64 + random_double()) / ((height - 1) as f64);
    // 			let r: Ray = cam.get_ray(&u, &v);
    // 			pixel_colour = vec3_add(&pixel_colour, &get_colour(&r, &world.clone(), &max_depth));
    // 		}
    // 		let mut arr = [0; 3];
    // 		arr[..].copy_from_slice(&camera::write_colour(&pixel_colour, &sample_per_pixel)[..3]);
    // 		*pixel = image::Rgb(arr);
    // 		// println!("Position: (x: {i}, y: {j})");
    // 		progress.inc(1);
    // 	}
    // }

    // for j in 0..height { // rev?
    // 	for i in 0..width {
    // let _pix: Vec<_> =
    let mut pixel_rgb = [Mem::new(); ((HEIGHT * WIDTH) as usize)];
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            pixel_rgb[get_id(&i, &j, &WIDTH)].i = i as u32;
            pixel_rgb[get_id(&i, &j, &WIDTH)].j = j as u32;
        }
    }
    pixel_rgb.par_iter_mut().for_each(|p: &mut Mem| {
        // let i: u32 = index % width;
        // let j: u32 = index / width;
        // let pixel = img.get_pixel_mut(i, height - j - 1);
        let i: u32 = p.i;
        let j: u32 = p.j;
        let mut pixel_colour = Vec3::set(0.0, 0.0, 0.0);
        for _s in 0..sample_per_pixel {
            let u = (i as f64 + random_double()) / ((WIDTH - 1) as f64);
            let v = (j as f64 + random_double()) / ((HEIGHT - 1) as f64);
            let r: Ray = cam.get_ray(&u, &v);
            pixel_colour = vec3_add(&pixel_colour, &get_colour(&r, &world.clone(), &max_depth));
        }
        let mut arr = [0; 3];
        arr[..].copy_from_slice(&camera::write_colour(&pixel_colour, &sample_per_pixel)[..3]);
        // *pixel = image::Rgb(arr);
        // pixel_rgb[i as usize][j as usize][..].copy_from_slice(&camera::write_colour(&pixel_colour, &sample_per_pixel)[..3]);
        // println!("Position: (x: {i}, y: {j})");
        [p.lr, p.lg, p.lb] = arr;
        progress.inc(1);
    });
    // .collect();

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            *img.get_pixel_mut(i, HEIGHT - j - 1) = image::Rgb([
                pixel_rgb[get_id(&i, &j, &WIDTH)].lr,
                pixel_rgb[get_id(&i, &j, &WIDTH)].lg,
                pixel_rgb[get_id(&i, &j, &WIDTH)].lb,
            ]);
        }
    }
    progress.finish();
    println!("Output image is in \"{}\"", style(path).green());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Output image failed").red()),
    }
    exit(0);
}
