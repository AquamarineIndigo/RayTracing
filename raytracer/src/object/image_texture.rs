// use image::{ImageBuffer, RgbImage};
use image::{GenericImageView, ImageBuffer, Rgb};

use crate::basic::{clamp, Vec3};

use super::Texture;

#[derive(Clone)]
pub struct ImageTexture {
    // data: Vec<[u8; 3]>,
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    // bytes_per_scanline: u32,
}
// fn get_id(i: u32, j: u32, width: u32) -> usize {
// 	(j * width + i) as usize
// }

impl ImageTexture {
    // const BYTES_PER_PIXEL: u32 = 3;
    pub fn new(s: String) -> Self {
        // let picture = image::open(s).unwrap();
        // let (width, height) = picture.dimensions();
        // let mut data: Vec<[u8; 3]> = Vec::new();
        // for y in 0..height {
        // 	for x in 0..width {
        // 		let pixel = picture.get_pixel(x, y);
        // 		data.push([pixel[0], pixel[1], pixel[2]]);
        // 	}
        // }
        // Self {
        // 	data, width , height,
        // 	// bytes_per_scanline: width * Self::BYTES_PER_PIXEL,
        // }
        let im = image::open(s).unwrap();
        let (width, height) = im.dimensions();
        Self {
            data: im.into_rgb8(),
            width,
            height,
            // bytes_per_scanline: width * Self::BYTES_PER_PIXEL,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        // if self.data.is_empty() {
        // return Vec3::set(0.0, 1.0, 1.0);
        // }
        let u = 1.0 - clamp(&u, &0.0, &1.0);
        let v = 1.0 - clamp(&v, &0.0, &1.0); // Flip of coordinates
        let mut i = (u * (self.width as f64)) as u32;
        let mut j = (v * (self.height as f64)) as u32;
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let pixel = self.data[(i, j)].0;
        Vec3::set(
            (pixel[0] as f64) / 255.0,
            (pixel[1] as f64) / 255.0,
            (pixel[2] as f64) / 255.0,
        )
    }
}
