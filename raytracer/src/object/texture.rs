use crate::basic::vec3::Vec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Copy, Clone)]
pub enum Textures {
    Solid(SolidColour),
    Checkered(CheckeredTexture),
}
#[derive(Copy, Clone)]
pub enum TexturesCheckered {
    Solid(SolidColour),
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

#[derive(Copy, Clone)]
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
            let TexturesCheckered::Solid(s) = self.odd;
            s.value(u, v, p)
        } else {
            let TexturesCheckered::Solid(s) = self.even;
            s.value(u, v, p)
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

// impl <TEven: Texture, TOdd: Texture> Default for CheckeredTexture <TEven, TOdd> {
// 	fn default() -> Self {
// 		Self {
// 			odd: Textures::Solid(SolidColour::default()),
// 			even: Textures::Solid(SolidColour::default()),
// 		}
// 	}
// }

// impl Into<TexturesCheckered> for Textures {
// 	fn into(self) -> TexturesCheckered {
// 		let mut ret = TexturesCheckered::Solid(SolidColour::default());
// 		// match self {
// 		// 	Self::Solid(s) => {
// 		// 		ret = TexturesCheckered::Solid(SolidColour::new_from_vector(&s.colour_value));
// 		// 	}
// 		// 	_ => {}
// 		// }
// 		if let Self::Solid(s) = self {
// 			ret = TexturesCheckered::Solid(SolidColour::new_from_vector(&s.colour_value));
// 		}
// 		ret
// 	}
// }

impl From<Textures> for TexturesCheckered {
    fn from(value: Textures) -> Self {
        let mut ret = TexturesCheckered::Solid(SolidColour::default());
        if let Textures::Solid(s) = value {
            ret = TexturesCheckered::Solid(SolidColour::new_from_vector(&s.colour_value));
        }
        ret
    }
}
