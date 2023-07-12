use super::clamp;
use super::random_range;
use super::ray::Ray;
use super::vec3::{generate_unit_vector, random_in_unit_disk, vec3_product, Vec3};
// use crate::basic::degrees_to_radians;

#[derive(Copy, Clone)]
pub struct VecUVW {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl VecUVW {
    pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Self {
        Self { u, v, w }
    }
    pub fn set(&mut self, u: &Vec3, v: &Vec3, w: &Vec3) -> &Self {
        [self.u, self.v, self.w] = [*u, *v, *w];
        self
    }
}

pub struct TimeInterval {
    pub time0: f64,
    pub time1: f64,
}

impl TimeInterval {
    pub fn new(time0: f64, time1: f64) -> Self {
        Self { time0, time1 }
    }
    pub fn set(&mut self, time0: f64, time1: f64) -> &Self {
        [self.time0, self.time1] = [time0, time1];
        self
    }
}

pub struct CameraCharacteristics {
    pub vfov: f64,         // vertical field-of-view in degrees
    pub aspect_ratio: f64, // usually 16:9
    pub aperture: f64,
    pub focus_dist: f64,
}

impl CameraCharacteristics {
    pub fn new(vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        Self {
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    // w: Vec3,
    // uvw: VecUVW,
    lens_radius: f64,
    time0: f64,
    time1: f64,
    // t01: TimeInterval,
}

impl Camera {
    pub fn new(
        look_from: &Vec3,
        look_at: &Vec3,
        vup: &Vec3,
        // vfov: f64,  // vfov -> vertical field-of-view in degrees
        // aspect_ratio: f64,
        // aperture: f64,
        // focus_dist: f64,
        para: CameraCharacteristics,
        time01: TimeInterval,
        // time0: f64,
        // time1: f64,
    ) -> Self {
        let theta = para.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewpoint_height = 2.0 * h;
        let viewpoint_width = para.aspect_ratio * viewpoint_height;

        let w_ = generate_unit_vector(&(*look_from - *look_at));
        let u_ = generate_unit_vector(&vec3_product(vup, &w_));
        let v_ = vec3_product(&w_, &u_);
        let horizontal_ = viewpoint_width * para.focus_dist * u_;
        let vertical_ = viewpoint_height * para.focus_dist * v_;
        Camera {
            origin: *look_from,
            horizontal: horizontal_,
            vertical: vertical_,
            lower_left_corner: *look_from
                - horizontal_ / 2.0
                - vertical_ / 2.0
                - para.focus_dist * w_,
            u: u_,
            v: v_,
            // VecUVW::new(u_, v_, w_),
            lens_radius: para.aperture / 2.0,
            time0: time01.time0,
            time1: time01.time1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = rd.x_dir * self.u + rd.y_dir * self.v;
        Ray::set_with_time(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random_range(self.time0, self.time1),
        )
    }
}
// impl Default for Camera {
// 	fn default() -> Self {
// 		Self::new(90.0, 16.0 / 9.0)
// 	}
// }

pub fn write_colour(pixel_colour: &Vec3, sample_per_pixel: &i32) -> Vec<u8> {
    let scale: f64 = 1.0 / (*sample_per_pixel as f64);
    let lr: f64 = (pixel_colour.x_dir * scale).sqrt();
    let lg: f64 = (pixel_colour.y_dir * scale).sqrt();
    let lb: f64 = (pixel_colour.z_dir * scale).sqrt();
    // lr *= scale;
    // lg *= scale;
    // lb *= scale;
    vec![
        (clamp(&lr, &0.0, &0.999) * 256.0) as u8,
        (clamp(&lg, &0.0, &0.999) * 256.0) as u8,
        (clamp(&lb, &0.0, &0.999) * 256.0) as u8,
    ]
}
