#[allow(dead_code)]
pub mod basic;
pub mod object;
use basic::ray::Ray;
use basic::vec3::{
    vec3_add, /*vec3_mul, vec3_sub,vec3_dot, vec3_tri_add, random_unit_vector,  generate_unit_vector*/
    Vec3,
};
use object::Translate;
// use object::ImageTexture;
// use object::Textures;
// use object::SolidColour;
// use object::basic::vec3::vec3_vec_mul;
use object::basic::random_double;
use object::boxes::Boxes;
use object::constant_medium::ConstantMedium;
// use object::basic::random_range;
use object::hittable::{HitRecord, Hittable};
use object::material::DiffuseLight;
// use object::sphere::Sphere;
// use object::sphere::MovingSphere;
use console::style;
use image::{ImageBuffer, RgbImage};
use object::hittable_list::HittableList;
// use rand::random;
// extern crate rayon;
// use rayon::prelude::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use object::translation::RotateY;
// use object::texture::CheckeredTexture;
// use object::texture::NoiseTexture;
use crate::object::material::{Lambertian, Metal /* , Dielectric, Material*/};
use std::{fs::File, process::exit};

use crate::basic::camera::{write_colour, Camera, CameraCharacteristics, TimeInterval};
use crate::object::aarect::{XYRect, XZRect, YZRect};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

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

// fn two_perlin_spheres() -> HittableList {
// 	let mut world = HittableList { objects: Vec::new() };
// 	let pertext = NoiseTexture::new(4.0);
// 	let mat = Lambertian::new_from_textures(&pertext);
// 	world.add(Sphere::set(Vec3::set(0.0, -1000.0, 0.0), 1000.0, &mat));
// 	world.add(Sphere::set(Vec3::set(0.0, 2.0, 0.0), 2.0, &mat));
// 	world
// }

// fn earth() -> HittableList {
// 	let earth_texture = ImageTexture::new("earthmap.jpg".to_string());
// 	let earth_surface = Lambertian::new_from_textures(&earth_texture);
// 	let mut world = HittableList { objects: Vec::new() };
// 	world.add(Sphere::set(Vec3::set(0.0, 0.0, 0.0), 2.0, &earth_surface));
// 	world
// }

// fn simple_light() -> HittableList {
// 	let mut world = HittableList { objects: Vec::new() };
// 	let pertext = NoiseTexture::new(4.0);
// 	// let mat = Lambertian::set(0.4, 0.2, 0.1);
// 	let mat = Lambertian::new_from_textures(&pertext);
// 	world.add(Sphere::set(Vec3::set(0.0, -1000.0, 0.0), 1000.0, &mat));
// 	world.add(Sphere::set(Vec3::set(0.0, 2.0, 0.0), 2.0, &mat));

// 	let diffuse_light = DiffuseLight::new_from_colour(&Vec3::set(4.0, 4.0, 4.0));
// 	world.add(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, &diffuse_light));
// 	world.add(Sphere::set(Vec3::set(0.0, 7.0, 0.0), 2.0, &diffuse_light));
// 	world
// }

// fn cornell_box() -> HittableList {
// 	let mut world = HittableList { objects: Vec::new() };
// 	let red =	Lambertian::new_from_colour(0.65, 0.05, 0.05);
// 	let white =	Lambertian::new_from_colour(0.73, 0.73, 0.73);
// 	let green =	Lambertian::new_from_colour(0.12, 0.45, 0.15);
// 	let light =	DiffuseLight::new_from_colour(15.0, 15.0, 15.0);
// 	world.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white));
// 	world.add(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, &light));
// 	world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white));
// 	world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white));
// 	world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &green));
// 	world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red));
// 	// world.add(Boxes::new(&Vec3::set(130.0, 0.0, 65.0), &Vec3::set(295.0, 165.0, 230.0), &white));
// 	// world.add(Boxes::new(&Vec3::set(265.0, 0.0, 295.0), &Vec3::set(430.0, 330.0, 460.0), &white));
// 	let box1 = Boxes::new(&Vec3::set(0.0, 0.0, 0.0), &Vec3::set(165.0, 330.0, 165.0), &white);
// 	let box1 = RotateY::new(Arc::new(box1), 15.0);
// 	let box1 = Translate::new(Arc::new(box1), Vec3::set(265.0, 0.0, 295.0));
// 	world.add(box1);
// 	let box2 = Boxes::new(&Vec3::set(0.0, 0.0, 0.0), &Vec3::set(165.0, 165.0, 165.0), &white);
// 	let box2 = RotateY::new(Arc::new(box2), -18.0);
// 	let box2 = Translate::new(Arc::new(box2), Vec3::set(130.0, 0.0, 65.0));
// 	world.add(box2);
// 	world
// }
fn cornell_smoke() -> HittableList {
    let mut world = HittableList {
        objects: Vec::new(),
    };
    let red = Lambertian::new_from_colour(0.65, 0.05, 0.05);
    let white = Lambertian::new_from_colour(0.73, 0.73, 0.73);
    let green = Lambertian::new_from_colour(0.12, 0.45, 0.15);
    let light = DiffuseLight::new_from_colour(15.0, 15.0, 15.0);
    world.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white));
    world.add(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, &light));
    world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white));
    world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white));
    world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &green));
    world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red));
    // world.add(Boxes::new(&Vec3::set(130.0, 0.0, 65.0), &Vec3::set(295.0, 165.0, 230.0), &white));
    // world.add(Boxes::new(&Vec3::set(265.0, 0.0, 295.0), &Vec3::set(430.0, 330.0, 460.0), &white));
    let box1 = Boxes::new(
        &Vec3::set(0.0, 0.0, 0.0),
        &Vec3::set(165.0, 330.0, 165.0),
        &white,
    );
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::set(265.0, 0.0, 295.0));
    world.add(ConstantMedium::new_from_colour(
        Arc::new(box1),
        0.01,
        &Vec3::set(0.0, 0.0, 0.0),
    ));
    let box2 = Boxes::new(
        &Vec3::set(0.0, 0.0, 0.0),
        &Vec3::set(165.0, 165.0, 165.0),
        &white,
    );
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), Vec3::set(130.0, 0.0, 65.0));
    world.add(ConstantMedium::new_from_colour(
        Arc::new(box2),
        0.01,
        &Vec3::set(1.0, 1.0, 1.0),
    ));
    world
}

fn get_colour(r: &Ray, background: &Vec3, world: &HittableList, depth: &i32) -> Vec3 {
    if *depth < 0 {
        return Vec3::set(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new(&Metal::set(0.0, 0.0, 0.0, 0.0));
    if world.hit(r, &0.001, &basic::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::set(0.0, 0.0, 0.0);
        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            emitted + attenuation * get_colour(&scattered, background, world, &(depth - 1))
        } else {
            emitted
        }
    } else {
        *background
    }
}

fn get_id(i: &u32, j: &u32, width: &u32) -> usize {
    (j * width + i) as usize
}

fn main() {
    let path = "output/book2/image2-17.jpg";
    // let width: u32 = 800;
    const WIDTH: u32 = 1024;
    let quality = 255;
    // let aspect_ratio: f64 = 16.0 / 9.0;
    // const ASPECTRATIO: f64 = 16.0 / 9.0;
    const ASPECTRATIO: f64 = 1.0;
    // let height: u32 = ((width as f64) / aspect_ratio) as u32;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECTRATIO) as u32;
    let sample_per_pixel = 200;
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

    let world = cornell_smoke();
    // let background = Vec3::set(0.7, 0.8, 1.0);
    let background = Vec3::set(0.0, 0.0, 0.0);
    let look_from = Vec3::set(278.0, 278.0, -800.0);
    let look_at = Vec3::set(278.0, 278.0, 0.0);
    let vup = Vec3::set(0.0, 1.0, 0.0);
    const APERTURE: f64 = 0.0;
    let dist_to_focus = 10.0;
    // let dist_to_focus = vec3_sub(&look_from, &look_at).length();

    let cam: Camera = Camera::new(
        &look_from,
        &look_at,
        &vup,
        CameraCharacteristics::new(40.0, ASPECTRATIO, APERTURE, dist_to_focus),
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
                            let r: Ray = cam.get_ray(u, v);
                            pixel_colour = vec3_add(
                                &pixel_colour,
                                &get_colour(
                                    &r,
                                    &background.clone(),
                                    &section_world.clone(),
                                    &max_depth,
                                ),
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

// https://raytracing.github.io/
