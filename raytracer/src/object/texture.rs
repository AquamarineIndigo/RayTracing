use crate::basic::vec3::{vec3_mul, Vec3};

use super::perlin::Perlin;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Clone)]
pub enum Textures {
    Solid(SolidColour),
    Checkered(Box<CheckeredTexture>),
    Noise(Box<NoiseTexture>),
}
#[derive(Clone)]
pub enum TexturesCheckered {
    Solid(SolidColour),
    Noise(Box<NoiseTexture>),
    // Checkered(CheckeredTexture<TEven, TOdd>),
}

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
pub struct CheckeredTexture {
    pub odd: TexturesCheckered,
    pub even: TexturesCheckered,
}

impl CheckeredTexture {
    // pub fn new_from_colour(c1: &Vec3, c2: &Vec3) -> Self {
    // 	Self {
    // 		even: SolidColour::new_from_vector(c1),
    // 		odd: SolidColour::new_from_vector(c2),
    // 	}
    // }
    pub fn new(even: Textures, odd: Textures) -> Self {
        Self {
            even: even.into(),
            odd: odd.into(),
        }
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (p.x_dir * 10.0).sin() * (p.y_dir * 10.0).sin() * (p.z_dir * 10.0).sin();
        if sines < 0.0 {
            match &self.odd {
                TexturesCheckered::Solid(s) => s.value(u, v, p),
                TexturesCheckered::Noise(n) => n.value(u, v, p),
            }
        } else {
            match &self.even {
                TexturesCheckered::Solid(s) => s.value(u, v, p),
                TexturesCheckered::Noise(n) => n.value(u, v, p),
            }
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

impl From<Textures> for TexturesCheckered {
    fn from(value: Textures) -> Self {
        let ret = TexturesCheckered::Solid(SolidColour::default());
        // if let Textures::Solid(s) = value {
        // 	ret = TexturesCheckered::Solid(SolidColour::new_from_vector(&s.colour_value));
        // }
        match value {
            Textures::Solid(s) => {
                TexturesCheckered::Solid(SolidColour::new_from_vector(&s.colour_value))
            }
            Textures::Noise(_) => TexturesCheckered::Noise(Box::new(NoiseTexture::new())),
            Textures::Checkered(_) => ret,
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        vec3_mul(&self.noise.noise(p), &Vec3::set(1.0, 1.0, 1.0))
    }
}
impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new()
    }
}
