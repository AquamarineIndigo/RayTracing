// use crate::basic::random_double;
use crate::basic::random_integer;
use crate::basic::Vec3;
// use std::sync::Arc;
use crate::basic::vec3::{generate_unit_vector, vec3_dot};

const POINT_COUNT: usize = 256;
#[derive(Clone, Copy)]
pub struct Perlin {
    // ranfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
    random_vectors: [Vec3; POINT_COUNT],
}

impl Perlin {
    fn permute(p: &mut [i32; POINT_COUNT], n: usize) {
        for i in (0..(n)).rev() {
            let target = random_integer(0, i as i32) as usize;
            // let tmp = p[i];
            // p[i] = p[target];
            // p[target] = tmp;
            p.swap(i, target);
        }
    }

    fn perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut p = [0_i32; POINT_COUNT];
        for (i, item) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *item = i as i32;
        }
        Perlin::permute(&mut p, POINT_COUNT);
        p
    }
    pub fn new() -> Self {
        // let mut ranfloat = [0.0; POINT_COUNT];
        // for mut _item in ranfloat.iter_mut().take(POINT_COUNT) {
        // 	*_item = random_double();
        // }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        // println!("{perm_x}, {perm_y}, {perm_z}");
        let mut random_vectors = [Vec3::set(0.0, 0.0, 0.0); POINT_COUNT];
        for item in random_vectors.iter_mut().take(POINT_COUNT) {
            item.copy_vector(&generate_unit_vector(&Vec3::random_vector_range(
                &-1.0, &1.0,
            )));
        }
        Self {
            // ranfloat,
            perm_x,
            perm_y,
            perm_z,
            random_vectors,
        }
    }
    // fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    // 	let mut accumulate = 0.0;
    // 	// for i in 0..2 {
    // 	// 	for j in 0..2 {
    // 	// 		for k in 0..2 {
    // 	for (i, it1) in c.iter().enumerate() {
    // 		for (j, it2) in it1.iter().enumerate() {
    // 			for (k, it3) in it2.iter().enumerate() {
    // 				accumulate += ((i as f64) * u + (1.0-(i as f64)) * (1.0-u)) *
    // 					((j as f64) * v + (1.0-(j as f64)) * (1.0-v)) *
    // 					((k as f64) * w + (1.0-(k as f64)) * (1.0-w)) * (*it3);
    // 			}
    // 		}
    // 	}
    // 	accumulate
    // }
    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let u_ = u * u * (3.0 - 2.0 * u);
        let v_ = v * v * (3.0 - 2.0 * v);
        let w_ = w * w * (3.0 - 2.0 * w);
        let mut accumulate = 0.0;
        for (i, it1) in c.iter().enumerate() {
            for (j, it2) in it1.iter().enumerate() {
                for (k, it3) in it2.iter().enumerate() {
                    let weight_v = Vec3::set(u - (i as f64), v - (j as f64), w - (k as f64));
                    accumulate += ((i as f64) * u_ + (1.0 - (i as f64)) * (1.0 - u_))
                        * ((j as f64) * v_ + (1.0 - (j as f64)) * (1.0 - v_))
                        * ((k as f64) * w_ + (1.0 - (k as f64)) * (1.0 - w_))
                        * vec3_dot(it3, &weight_v);
                }
            }
        }
        accumulate
    }
    pub fn noise(&self, point: &Vec3) -> f64 {
        //  No Smoothing
        // let i = ((4.0 * point.x_dir) as i32) & 255;
        // let j = ((4.0 * point.y_dir) as i32) & 255;
        // let k = ((4.0 * point.z_dir) as i32) & 255;
        // self.ranfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]

        // Smoothing
        // let u = point.x_dir - point.x_dir.floor();
        // let v = point.y_dir - point.y_dir.floor();
        // let w = point.z_dir - point.z_dir.floor();
        // let u = u * u * (3.0 - 2.0*u);
        // let v = v * v * (3.0 - 2.0*v);
        // let w = w * w * (3.0 - 2.0*w);
        // let i = point.x_dir.floor() as i32;
        // let j = point.y_dir.floor() as i32;
        // let k = point.z_dir.floor() as i32;
        // let mut c = [[[0.0; 2]; 2]; 2];
        // // for di in 0..2 {
        // // 	for dj in 0..2 {
        // // 		for dk in 0..2 {
        // for (di, it1) in c.iter_mut().enumerate() {
        // 	for (dj, it2) in it1.iter_mut().enumerate() {
        // 		for (dk, it3) in it2.iter_mut().enumerate() {
        // 			*it3 = self.ranfloat[(
        // 				self.perm_x[((i+di as i32)&255)as usize] ^
        // 				self.perm_y[((j+dj as i32)&255) as usize] ^
        // 				self.perm_z[((k+dk as i32)&255) as usize]
        // 			) as usize];
        // 		}
        // 	}
        // }
        // Self::trilinear_interp(&c, u, v, w)

        // More Reasonable Smoothing
        let u = point.x_dir - point.x_dir.floor();
        let v = point.y_dir - point.y_dir.floor();
        let w = point.z_dir - point.z_dir.floor();
        let i = point.x_dir.floor() as i32;
        let j = point.y_dir.floor() as i32;
        let k = point.z_dir.floor() as i32;
        let mut c = [[[Vec3::set(0.0, 0.0, 0.0); 2]; 2]; 2];
        for (di, it1) in c.iter_mut().enumerate() {
            for (dj, it2) in it1.iter_mut().enumerate() {
                for (dk, it3) in it2.iter_mut().enumerate() {
                    it3.copy_vector(
                        &self.random_vectors[(self.perm_x[((i + di as i32) & 255) as usize]
                            ^ self.perm_y[((j + dj as i32) & 255) as usize]
                            ^ self.perm_z[((k + dk as i32) & 255) as usize])
                            as usize],
                    );
                }
            }
        }
        Self::perlin_interp(&c, u, v, w)
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
