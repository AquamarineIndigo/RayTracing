pub mod camera;
pub mod ray;
pub mod vec3;
// pub mod utility;
pub use ray::Ray;
use std::f64::consts::PI;
pub use vec3::Vec3;

pub const INFINITY: f64 = f64::INFINITY;
// pub const PI: f64 = 3.141_592_653_589_793;

pub fn degrees_to_radians(degrees: &f64) -> f64 {
    degrees * PI / 180.0
}

pub use rand::prelude::*;
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: &f64, min: &f64, max: &f64) -> f64 {
    if x < min {
        return *min;
    }
    if x > max {
        return *max;
    }
    *x
}
