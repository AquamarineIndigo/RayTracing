use super::perlin::Perlin;
use crate::basic::vec3::{vec3_mul, Vec3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

// #[derive(Clone)]
// pub enum Textures {
// 	Solid(SolidColour),
// 	Checkered(Box<CheckeredTexture>),
// 	Noise(Box<NoiseTexture>),
// }
// #[derive(Clone)]
// pub enum TexturesCheckered {
// 	Solid(SolidColour),
// 	Noise(Box<NoiseTexture>),
// 	// Checkered(CheckeredTexture<TEven, TOdd>),
// }

#[derive(Copy, Clone)]
pub struct SolidColour {
    colour_value: Vec3,
}

impl SolidColour {
    pub fn new_from_vector(a: &Vec3) -> Self {
        Self { colour_value: *a }
    }
    pub fn new_from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            colour_value: Vec3::set(r, g, b),
        }
    }
    pub fn set_vector(&mut self, a: &Vec3) -> &Self {
        self.colour_value.copy_vector(a);
        self
    }
    pub fn set_rgb(&mut self, r: f64, g: f64, b: f64) -> &Self {
        self.colour_value.set_value(r, g, b);
        self
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.colour_value
    }
}

#[derive(Clone)]
pub struct CheckeredTexture<TEven, TOdd>
where
    TEven: Texture + Clone,
    TOdd: Texture + Clone,
{
    pub odd: TOdd,
    pub even: TEven,
}

impl<TEven: Texture + Clone, TOdd: Texture + Clone> CheckeredTexture<TEven, TOdd> {
    // pub fn new_from_colour(c1: &Vec3, c2: &Vec3) -> Self {
    // 	Self {
    // 		even: SolidColour::new_from_vector(c1),
    // 		odd: SolidColour::new_from_vector(c2),
    // 	}
    // }
    pub fn new(even: TEven, odd: TOdd) -> Self {
        Self { even, odd }
    }
}

impl<TEven: Texture + Clone, TOdd: Texture + Clone> Texture for CheckeredTexture<TEven, TOdd> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (p.x_dir * 10.0).sin() * (p.y_dir * 10.0).sin() * (p.z_dir * 10.0).sin();
        if sines < 0.0 {
            // match &self.odd {
            // 	TexturesCheckered::Solid(s) => {
            // 		s.value(u, v, p)
            // 	}
            // 	TexturesCheckered::Noise(n) => {
            // 		n.value(u, v, p)
            // 	}
            // }
            self.odd.value(u, v, p)
        } else {
            // match &self.even {
            // 	TexturesCheckered::Solid(s) => {
            // 		s.value(u, v, p)
            // 	}
            // 	TexturesCheckered::Noise(n) => {
            // 		n.value(u, v, p)
            // 	}
            // }
            self.even.value(u, v, p)
        }
    }
}

impl Default for SolidColour {
    fn default() -> Self {
        Self {
            colour_value: Vec3::set(0.0, 0.0, 0.0),
        }
    }
}

// impl From<Textures> for TexturesCheckered {
// 	fn from(value: Textures) -> Self {
// 		let ret = TexturesCheckered::Solid(SolidColour::default());
// 		// if let Textures::Solid(s) = value {
// 		// 	ret = TexturesCheckered::Solid(SolidColour::new_from_vector(&s.colour_value));
// 		// }
// 		match value {
// 			Textures::Solid(s) => {
// 				TexturesCheckered::Solid(SolidColour::new_from_vector(&s.colour_value))
// 			}
// 			Textures::Noise(n) => {
// 				TexturesCheckered::Noise(Box::new(NoiseTexture::new(n.scale)))
// 			}
// 			Textures::Checkered(_) => {ret},
// 		}
// 	}
// }

#[derive(Clone, Copy)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    const TURBULANCE_DEPTH: i32 = 7;
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        vec3_mul(
            // &((1.0 + self.noise.noise(&vec3_mul(&self.scale, p))) * 0.5),
            &(0.5
                * (1.0
                    + (self.scale * p.z_dir
                        + 10.0
                            * self
                                .noise
                                .turbulence(&vec3_mul(&self.scale, p), Self::TURBULANCE_DEPTH))
                    .sin())),
            &Vec3::set(1.0, 1.0, 1.0),
        )
    }
}
impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new(1.0)
    }
}
