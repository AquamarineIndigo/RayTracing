use std::mem::swap;

use crate::basic::ray::Ray;
use crate::basic::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct AxisAlignedBoundingBoxes {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AxisAlignedBoundingBoxes {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn set(&mut self, a: Vec3, b: Vec3) -> &Self {
        self.minimum.copy_vector(&a);
        self.maximum.copy_vector(&b);
        self
    }

    pub fn copy_from_other(&mut self, other: &AxisAlignedBoundingBoxes) -> &Self {
        self.minimum.copy_vector(&other.minimum);
        self.maximum.copy_vector(&other.maximum);
        self
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let invd = 1.0 / r.direction[i];
            let mut t0 = (self.minimum[i] - r.origin[i]) * invd;
            let mut t1 = (self.maximum[i] - r.origin[i]) * invd;
            if invd < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(
    box0: &AxisAlignedBoundingBoxes,
    box1: &AxisAlignedBoundingBoxes,
) -> AxisAlignedBoundingBoxes {
    let small = Vec3::set(
        if box0.minimum.x_dir < box1.minimum.x_dir {
            box0.minimum.x_dir
        } else {
            box1.minimum.x_dir
        },
        if box0.minimum.y_dir < box1.minimum.y_dir {
            box0.minimum.y_dir
        } else {
            box1.minimum.y_dir
        },
        if box0.minimum.z_dir < box1.minimum.z_dir {
            box0.minimum.z_dir
        } else {
            box1.minimum.z_dir
        },
    );
    let big = Vec3::set(
        if box0.maximum.x_dir > box1.maximum.x_dir {
            box0.maximum.x_dir
        } else {
            box1.maximum.x_dir
        },
        if box0.maximum.y_dir > box1.maximum.y_dir {
            box0.maximum.y_dir
        } else {
            box1.maximum.y_dir
        },
        if box0.maximum.z_dir > box1.maximum.z_dir {
            box0.maximum.z_dir
        } else {
            box1.maximum.z_dir
        },
    );
    AxisAlignedBoundingBoxes::new(small, big)
}

impl Default for AxisAlignedBoundingBoxes {
    fn default() -> Self {
        Self {
            minimum: Vec3::set(0.0, 0.0, 0.0),
            maximum: Vec3::set(0.0, 0.0, 0.0),
        }
    }
}
