use super::Hittable; //, HittableList};
use super::{surrounding_box, AxisAlignedBoundingBoxes};
use crate::basic::random_integer;
use std::{cmp::Ordering, sync::Arc};
// use std::ptr::null;
use crate::basic::ray::Ray;

#[derive(Clone)]
pub struct BvhNode {
    pub boxes: AxisAlignedBoundingBoxes,
    pub left: Arc<BvhNode>,
    pub right: Arc<BvhNode>,
}

impl BvhNode {
    pub fn new_from_vector(
        &mut self,
        src_objects: &[Arc<BvhNode>],
        start: usize,
        end: usize,
        _time0: f64,
        _time1: f64,
    ) -> &Self {
        let mut objects = src_objects.to_owned();
        let axis = random_integer(0, 2);
        let object_span = end - start;
        if object_span == 1 {
            self.left = objects[start].clone();
            self.right = objects[start].clone();
        } else if object_span == 2 {
            if box_compare(&objects[start], &objects[start + 1], axis) == Ordering::Less {
                self.left = objects[start].clone();
                self.right = objects[start + 1].clone();
            } else {
                self.left = objects[start + 1].clone();
                self.right = objects[start].clone();
            }
        } else {
            objects.sort_by(|a, b| box_compare(a, b, axis));
            let mid = start + object_span / 2;
            self.left = Arc::new(
                (*self)
                    .new_from_vector(&objects, start, mid, _time0, _time1)
                    .clone(),
            );
            self.right = Arc::new(
                (*self)
                    .new_from_vector(&objects, mid, end, _time0, _time1)
                    .clone(),
            );
        }
        let mut box_l = AxisAlignedBoundingBoxes::default();
        let mut box_r = AxisAlignedBoundingBoxes::default();
        if !(*self.left).bounding_box(0.0, 0.0, &mut box_l)
            || !(*self.right).bounding_box(0.0, 0.0, &mut box_r)
        {
            println!("No Bounding Box in BVHNode Constructor");
        }
        self.boxes = surrounding_box(&box_l, &box_r);
        self
    }
    // pub fn new_from_list(list: &HittableList, time0:f64, time1: f64) -> Self {
    // 	Self {
    // 		boxes:
    // 	}
    // }
}

// impl Default for BvhNode {
// 	fn default() -> Self {
// 		let nullptr: *const BvhNode = null();
// 		Self {
// 			boxes: AxisAlignedBoundingBoxes::default(),
// 			left: Arc::new(),
// 			right: Arc::new(),
// 		}
// 	}
// }

impl Hittable for BvhNode {
    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool {
        output_box.copy_from_other(&self.boxes);
        true
    }
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut super::HitRecord) -> bool {
        if !self.boxes.hit(r, *t_min, *t_max) {
            return false;
        }
        let hit_left: bool = (*self.left).hit(r, t_min, t_max, rec);
        let t_clone = rec.t;
        let hit_right: bool =
            (*self.right).hit(r, t_min, if hit_left { &t_clone } else { t_max }, rec);
        hit_left || hit_right
    }
}

pub fn box_compare(a: &Arc<BvhNode>, b: &Arc<BvhNode>, axis: i32) -> Ordering {
    let mut box_a = AxisAlignedBoundingBoxes::default();
    let mut box_b = AxisAlignedBoundingBoxes::default();
    if !(*a).bounding_box(0.0, 0.0, &mut box_a) || !(*b).bounding_box(0.0, 0.0, &mut box_b) {
        println!("No Bounding Box in BVHNode Constructor");
    }
    match axis {
        0 => {
            if box_a.minimum.x_dir < box_b.minimum.x_dir {
                Ordering::Less
            } else if box_a.minimum.x_dir > box_b.minimum.x_dir {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        1 => {
            if box_a.minimum.y_dir < box_b.minimum.y_dir {
                Ordering::Less
            } else if box_a.minimum.y_dir > box_b.minimum.y_dir {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        _ => {
            if box_a.minimum.z_dir < box_b.minimum.z_dir {
                Ordering::Less
            } else if box_a.minimum.z_dir > box_b.minimum.z_dir {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}
