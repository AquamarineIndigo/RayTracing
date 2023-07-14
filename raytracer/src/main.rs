#[allow(dead_code)]
pub mod basic;
pub mod object;
use basic::ray::Ray;
use basic::vec3::{
    vec3_add, /*vec3_mul, vec3_sub,vec3_dot, vec3_tri_add, random_unit_vector,  generate_unit_vector*/
    Vec3,
};
use object::basic::{random_double, random_range};
use object::boxes::Boxes;
use object::constant_medium::ConstantMedium;
use object::sphere::MovingSphere;
use object::texture::NoiseTexture;
use object::{obj_file, BvhNode, ImageTexture, Sphere, Translate};
// use object::basic::random_range;
use console::style;
use image::{ImageBuffer, RgbImage};
use object::hittable::{HitRecord, Hittable};
use object::hittable_list::HittableList;
use object::material::{Dielectric, DiffuseLight, Isotropic};
use rand::seq::SliceRandom;
use rand::thread_rng;
// extern crate rayon;
// use rayon::prelude::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use object::translation::RotateY;
// use object::texture::CheckeredTexture;
// use object::texture::NoiseTexture;
use crate::object::material::{Lambertian, Metal /* , Dielectric, Material*/};
use std::{fs::File, process::exit};

use crate::basic::camera::{write_colour, Camera, CameraCharacteristics, TimeInterval};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
// use crate::object::aarect::{XZRect, XYRect, YZRect};
use crate::object::aarect::XZRect;

// fn final_scene() -> HittableList {
// 	let mut boxes1 = HittableList { objects: Vec::new() };
// 	let ground = Lambertian::new_from_colour(0.48, 0.83, 0.53);

// 	let boxes_per_side: i32 = 2;
// 	for i in 0..boxes_per_side {
// 		for j in 0..boxes_per_side {
// 			let w = 100.0;
// 			let x0 = -1000.0 + (i as f64) * w;
// 			let z0 = -1000.0 + (j as f64) * w;
// 			let y0 = 0.0;
// 			let x1 = x0 + w;
// 			let y1 = random_range(1.0, 101.0);
// 			let z1 = z0 + w;
// 			boxes1.add(Boxes::new(&Vec3::set(x0, y0, z0), &Vec3::set(x1, y1, z1), &ground));
// 		}
// 	}

// 	let mut world = HittableList { objects: Vec::new() };
// 	world.add(BvhNode::new_from_list(&mut boxes1, 0.0, 1.0));
// 	let light = DiffuseLight::new_from_colour(7.0, 7.0, 7.0);
// 	world.add(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, &light));
// 	let center1 = Vec3::set(400.0, 400.0, 200.0);
// 	let center2 = center1 + Vec3::set(30.0, 0.0, 0.0);
// 	let moving_sphere_material = Lambertian::new_from_colour(0.7, 0.3, 0.1);
// 	world.add(MovingSphere::set(&center1, &center2, 0.0, 1.0, 50.0, &moving_sphere_material));
// 	world.add(Sphere::set(Vec3::set(260.0, 150.0, 45.0), 50.0, &Dielectric::set(1.5)));
// 	world.add(Sphere::set(Vec3::set(0.0, 150.0, 145.0), 50.0, &Metal::set(0.8, 0.8, 0.9, 1.0)));

// 	let mut boundary = Sphere::set(Vec3::set(360.0, 150.0, 145.0), 70.0, &Dielectric::set(1.5));
// 	world.add(boundary.clone());
// 	world.add(ConstantMedium::new_from_colour(Arc::new(boundary.clone()), 0.2, &Vec3::set(0.2, 0.4, 0.9)));
// 	boundary = Sphere::set(Vec3::set(0.0, 0.0, 0.0), 5000.0, &Dielectric::set(1.5));
// 	world.add(ConstantMedium::new_from_colour(Arc::new(boundary), 0.0001, &Vec3::set(1.0, 1.0, 1.0)));

// 	let emat = Lambertian::new_from_textures(&ImageTexture::new("earthmap.jpg".to_string()));
// 	world.add(Sphere::set(Vec3::set(400.0, 200.0, 400.0), 100.0, &emat));
// 	let pertext = NoiseTexture::new(0.1);
// 	world.add(Sphere::set(Vec3::set(220.0, 280.0, 300.0), 80.0, &Lambertian::new_from_textures(&pertext)));

// 	let mut boxes2 = HittableList { objects: Vec::new() };
// 	let white = Lambertian::new_from_colour(0.73, 0.73, 0.73);
// 	let ns = 10;
// 	for _j in 0..ns {
// 		boxes2.add(Sphere::set(Vec3::random_vector_range(&0.0, &165.0), 10.0, &white));
// 	}
// 	world.add(
// 		Translate::new(
// 			Arc::new(RotateY::new(
// 				Arc::new(BvhNode::new_from_list(&mut boxes2, 0.0, 1.0)), 15.0
// 			)),
// 			Vec3::set(-100.0, 270.0, 395.0)
// 		)
// 	);
// 	world
// }

fn cats() -> HittableList {
    let mut boxes1 = HittableList {
        objects: Vec::new(),
    };
    let ground = Lambertian::new_from_colour(0.48, 0.83, 0.53);

    let boxes_per_side: i32 = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_range(20.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Boxes::new(
                &Vec3::set(x0, y0, z0),
                &Vec3::set(x1, y1, z1),
                &ground,
            ));
        }
    }

    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.add(BvhNode::new_from_list(&mut boxes1, 0.0, 1.0));
    let light = DiffuseLight::new_from_colour(14.0, 15.0, 15.0);
    world.add(XZRect::new(100.0, 500.0, -150.0, 450.0, 754.0, &light));
    let center1 = Vec3::set(70.0, 500.0, -200.0);
    let center2 = center1 + Vec3::set(60.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new_from_colour(0.7, 0.3, 0.1);
    world.add(MovingSphere::set(
        &center1,
        &center2,
        0.0,
        1.0,
        50.0,
        &moving_sphere_material,
    ));
    world.add(Sphere::set(
        Vec3::set(390.0, 150.0, -240.0),
        50.0,
        &Dielectric::set(1.5),
    ));
    world.add(Sphere::set(
        Vec3::set(0.0, 150.0, 30.0),
        50.0,
        &Metal::set(0.8, 0.8, 1.0, 0.2),
    ));

    let more_light = DiffuseLight::new_from_colour(5.7, 0.1, 0.0);
    world.add(Sphere::set(
        Vec3::set(300.0, 450.0, 200.0),
        35.0,
        &more_light,
    ));

    let mut boundary = Sphere::set(Vec3::set(560.0, 150.0, -260.0), 70.0, &Dielectric::set(1.5));
    world.add(boundary.clone());
    world.add(ConstantMedium::new_from_colour(
        Arc::new(boundary.clone()),
        0.2,
        &Vec3::set(0.2, 0.6, 0.9),
    ));
    boundary = Sphere::set(Vec3::set(0.0, 0.0, 0.0), 5000.0, &Dielectric::set(1.5));
    world.add(ConstantMedium::new_from_colour(
        Arc::new(boundary),
        0.0001,
        &Vec3::set(1.0, 1.0, 1.0),
    ));

    let emat = Lambertian::new_from_textures(&ImageTexture::new("objs/earthmap.jpg".to_string()));
    // let emat = Lambertian::new_from_textures(&NoiseTexture::new(0.8));
    world.add(Sphere::set(Vec3::set(50.0, 350.0, -100.0), 100.0, &emat));
    let pertext = NoiseTexture::new(0.1);
    world.add(Sphere::set(
        Vec3::set(220.0, 530.0, -40.0),
        80.0,
        &Lambertian::new_from_textures(&pertext),
    ));

    let mut cat = obj_file::obj_file(
        "objs/Cat.obj".to_string(),
        Arc::new(Metal::new_from_vector(&Vec3::set(0.89, 0.45, 0.39), 0.0)),
        0.3,
    );
    world.add(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_from_vector(&mut cat[..], 0.0, 1.0)),
            16.0,
        )),
        Vec3::set(300.0, 101.0, -400.0),
    ));
    let mut rose = obj_file::obj_file(
        "objs/Rose.obj".to_string(),
        // Lambertian::new_from_colour(1.00, 0.68, 0.73),
        Arc::new(Isotropic::new_from_textures(&ImageTexture::new(
            "objs/rose_colour.jpg".to_string(),
        ))),
        2.5,
    );
    world.add(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_from_vector(&mut rose[..], 0.0, 1.0)),
            45.0,
        )),
        Vec3::set(460.0, 90.0, -250.0),
    ));
    let mut airplane = obj_file::obj_file(
        "objs/Airplane.obj".to_string(),
        Arc::new(Metal::new_from_vector(&Vec3::set(0.68, 1.00, 0.73), 0.0)),
        0.12,
    );
    world.add(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_from_vector(&mut airplane[..], 0.0, 1.0)),
            45.0,
        )),
        Vec3::set(500.0, 430.0, 200.0),
    ));
    let mut koenigsegg = obj_file::obj_file(
        "objs/Koenigsegg.obj".to_string(),
        Arc::new(Metal::new_from_vector(&Vec3::set(0.38, 0.45, 0.43), 0.0)),
        17.0,
    );
    world.add(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_from_vector(&mut koenigsegg[..], 0.0, 1.0)),
            105.0,
        )),
        Vec3::set(300.0, 105.0, 250.0),
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
    let path = "output/book2/image2-final_scene.jpg";
    // let width: u32 = 800;
    const WIDTH: u32 = 800;
    let quality = 200;
    // let aspect_ratio: f64 = 16.0 / 9.0;
    // const ASPECTRATIO: f64 = 16.0 / 9.0;
    const ASPECTRATIO: f64 = 1.0;
    // let height: u32 = ((width as f64) / aspect_ratio) as u32;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECTRATIO) as u32;
    let sample_per_pixel = 5000;
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let max_depth = 50;

    let world = cats();
    // let background = Vec3::set(0.7, 0.8, 1.0);
    let background = Vec3::set(0.0, 0.0, 0.0);
    let look_from = Vec3::set(678.0, 278.0, -800.0);
    let look_at = Vec3::set(278.0, 278.0, 0.0);
    let vup = Vec3::set(0.0, 1.0, 0.0);
    const APERTURE: f64 = 0.0;
    let dist_to_focus = 10.0;
    // let dist_to_focus = vec3_sub(&look_from, &look_at).length();

    let cam: Camera = Camera::new(
        &look_from,
        &look_at,
        &vup,
        CameraCharacteristics::new(50.0, ASPECTRATIO, APERTURE, dist_to_focus),
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
    let mut rng = thread_rng();
    let mut rows = [0; HEIGHT as usize];
    for i in 0..HEIGHT {
        rows[i as usize] = i;
    }
    rows.shuffle(&mut rng);

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
				.progress_chars("#| ")
		);

        // thread code
        let (tx, rx) = mpsc::channel();

        thread_pool.push((
            thread::spawn(move || {
                let mut progress = 0;
                progress_bar.set_position(progress);
                let mut section_pixel_color = Vec::<Vec<u8>>::new();
                for j_r in row_begin..row_end {
                    for i in 0..WIDTH {
                        let j: u32 = rows[j_r as usize];
                        let mut pixel_colour = Vec3::set(0.0, 0.0, 0.0);
                        for _s in 0..sample_per_pixel {
                            let u = (i as f64 + random_double()) / ((WIDTH - 1) as f64);
                            let v = (j as f64 + random_double()) / ((HEIGHT - 1) as f64);
                            let r: Ray = cam.get_ray(u, v);
                            pixel_colour = vec3_add(
                                &pixel_colour,
                                &get_colour(&r, &background, &section_world.clone(), &max_depth),
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

    for j_r in 0..HEIGHT {
        for i in 0..WIDTH {
            let j = rows[j_r as usize];
            *img.get_pixel_mut(i, HEIGHT - j - 1) = image::Rgb([
                output_pixel_color[get_id(&i, &j_r, &WIDTH)][0],
                output_pixel_color[get_id(&i, &j_r, &WIDTH)][1],
                output_pixel_color[get_id(&i, &j_r, &WIDTH)][2],
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
// https://free3d.com/3d-models/?page=2
