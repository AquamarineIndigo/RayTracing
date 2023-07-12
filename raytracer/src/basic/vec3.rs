use super::random_range;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x_dir: f64,
    pub y_dir: f64,
    pub z_dir: f64,
}

pub fn vec3_add(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x_dir: a.x_dir + b.x_dir,
        y_dir: a.y_dir + b.y_dir,
        z_dir: a.z_dir + b.z_dir,
    }
}
pub fn vec3_mul(a: &f64, b: &Vec3) -> Vec3 {
    Vec3 {
        x_dir: a * b.x_dir,
        y_dir: a * b.y_dir,
        z_dir: a * b.z_dir,
    }
}
pub fn vec3_tri_add(a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3 {
    Vec3 {
        x_dir: a.x_dir + b.x_dir + c.x_dir,
        y_dir: a.y_dir + b.y_dir + c.y_dir,
        z_dir: a.z_dir + b.z_dir + c.z_dir,
    }
}
pub fn vec3_sub(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x_dir: a.x_dir - b.x_dir,
        y_dir: a.y_dir - b.y_dir,
        z_dir: a.z_dir - b.z_dir,
    }
}
pub fn vec3_dot(a: &Vec3, b: &Vec3) -> f64 {
    (a.x_dir * b.x_dir) + (a.y_dir * b.y_dir) + (a.z_dir * b.z_dir)
}
pub fn vec3_vec_mul(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x_dir: a.x_dir * b.x_dir,
        y_dir: a.y_dir * b.y_dir,
        z_dir: a.z_dir * b.z_dir,
    }
}
pub fn vec3_product(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x_dir: a.y_dir * b.z_dir - a.z_dir * b.y_dir,
        y_dir: a.z_dir * b.x_dir - a.x_dir * b.z_dir,
        z_dir: a.x_dir * b.y_dir - a.y_dir * b.x_dir,
    }
}

impl Vec3 {
    const NEAR_ZERO: f64 = 1e-8;
    pub fn set(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x_dir: x,
            y_dir: y,
            z_dir: z,
        }
    }
    // pub fn clone(&self) -> Vec3 {
    // 	Vec3 {
    // 		x_dir: self.x_dir,
    // 		y_dir: self.y_dir,
    // 		z_dir: self.z_dir,
    // 	}
    // }
    pub fn random_vector() -> Vec3 {
        Vec3::set(
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
        )
    }
    pub fn random_vector_range(min: &f64, max: &f64) -> Vec3 {
        Vec3::set(
            random_range(*min, *max),
            random_range(*min, *max),
            random_range(*min, *max),
        )
    }
    pub fn copy_vector(&mut self, other: &Vec3) -> &Self {
        [self.x_dir, self.y_dir, self.z_dir] = [other.x_dir, other.y_dir, other.z_dir];
        self
    }
    pub fn near_zero(&self) -> bool {
        (self.x_dir.abs() < Self::NEAR_ZERO)
            && (self.y_dir.abs() < Self::NEAR_ZERO)
            && (self.z_dir.abs() < Self::NEAR_ZERO)
    }
    pub fn set_value(&mut self, a: f64, b: f64, c: f64) -> &Self {
        [self.x_dir, self.y_dir, self.z_dir] = [a, b, c];
        self
    }
    pub fn length(&self) -> f64 {
        (self.x_dir.powi(2) + self.y_dir.powi(2) + self.z_dir.powi(2)).sqrt()
    }
}

pub fn generate_unit_vector(direction: &Vec3) -> Vec3 {
    let l_sqr = direction.x_dir.powi(2) + direction.y_dir.powi(2) + direction.z_dir.powi(2);
    let l = l_sqr.sqrt();
    Vec3 {
        x_dir: direction.x_dir / l,
        y_dir: direction.y_dir / l,
        z_dir: direction.z_dir / l,
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_vector();
        if p.length() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    generate_unit_vector(&random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if vec3_dot(normal, &in_unit_sphere) > 0.0 {
        in_unit_sphere
    } else {
        vec3_mul(&-1.0, &in_unit_sphere)
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2.0 * vec3_dot(v, n) * (*n)
}

pub fn refract(uv: &Vec3, n: &Vec3, eta_i_over_eta_t: &f64) -> Vec3 {
    let cos_theta: f64 = vec3_dot(&-(*uv), n).min(1.0);
    let r_out_perpendicular: Vec3 = (*eta_i_over_eta_t) * (*uv + cos_theta * (*n));
    let r_out_parallel: Vec3 = -(1.0 - vec3_dot(&r_out_perpendicular, &r_out_perpendicular))
        .abs()
        .sqrt()
        * (*n);
    r_out_perpendicular + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::set(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
        if vec3_dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x_dir: -self.x_dir,
            y_dir: -self.y_dir,
            z_dir: -self.z_dir,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec3_add(&rhs, &self)
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x_dir: self.x_dir + rhs,
            y_dir: self.y_dir + rhs,
            z_dir: self.z_dir + rhs,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x_dir: self.x_dir + rhs.x_dir,
            y_dir: self.y_dir + rhs.y_dir,
            z_dir: self.z_dir + rhs.z_dir,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = Self {
            x_dir: self.x_dir + rhs,
            y_dir: self.y_dir + rhs,
            z_dir: self.z_dir + rhs,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec3_sub(&self, &rhs)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            x_dir: self.x_dir - rhs,
            y_dir: self.y_dir - rhs,
            z_dir: self.z_dir - rhs,
        }
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x_dir: self.x_dir - rhs.x_dir,
            y_dir: self.y_dir - rhs.y_dir,
            z_dir: self.z_dir - rhs.z_dir,
        };
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = Self {
            x_dir: self.x_dir - rhs,
            y_dir: self.y_dir - rhs,
            z_dir: self.z_dir - rhs,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x_dir: self.x_dir * rhs.x_dir,
            y_dir: self.y_dir * rhs.y_dir,
            z_dir: self.z_dir * rhs.z_dir,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x_dir: self.x_dir * rhs,
            y_dir: self.y_dir * rhs,
            z_dir: self.z_dir * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x_dir: self * rhs.x_dir,
            y_dir: self * rhs.y_dir,
            z_dir: self * rhs.z_dir,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x_dir: self.x_dir * rhs,
            y_dir: self.y_dir * rhs,
            z_dir: self.z_dir * rhs,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x_dir: self.x_dir / rhs,
            y_dir: self.y_dir / rhs,
            z_dir: self.z_dir / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x_dir: self.x_dir / rhs,
            y_dir: self.y_dir / rhs,
            z_dir: self.z_dir / rhs,
        };
    }
}

impl Index<i32> for Vec3 {
    type Output = f64;
    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x_dir,
            1 => &self.y_dir,
            2 => &self.z_dir,
            _ => &0.0,
        }
    }
}
