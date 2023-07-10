use crate::basic::{random_double, random_integer, Vec3};
// use std::sync::Arc;

const POINT_COUNT: usize = 256;
#[derive(Clone, Copy)]
pub struct Perlin {
    ranfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
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
        let mut ranfloat = [0.0; POINT_COUNT];
        for mut _item in ranfloat.iter_mut().take(POINT_COUNT) {
            *_item = random_double();
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        // println!("{perm_x}, {perm_y}, {perm_z}");
        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, point: &Vec3) -> f64 {
        let i = ((4.0 * point.x_dir) as i32) & 255;
        let j = ((4.0 * point.y_dir) as i32) & 255;
        let k = ((4.0 * point.z_dir) as i32) & 255;
        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
