use super::{surrounding_box, AxisAlignedBoundingBoxes};
use super::{Hittable, HittableList}; //, HittableList};
use crate::basic::random_integer;
use std::{cmp::Ordering, sync::Arc};
// use std::ptr::null;
use crate::basic::ray::Ray;

#[derive(Clone)]
pub struct BvhNode {
    pub boxes: AxisAlignedBoundingBoxes,
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
}

impl BvhNode {
    pub fn new_from_vector(
        objects: &mut [Arc<dyn Hittable + 'static + Send + Sync>],
        time0: f64,
        time1: f64,
    ) -> Self {
        // let mut objects = src_objects.to_owned();
        let axis = random_integer(0, 2);
        let object_span = objects.len();
        if object_span == 1 {
            let mut boxes = AxisAlignedBoundingBoxes::default();
            objects[0].bounding_box(time0, time1, &mut boxes);
            Self {
                left: Some(objects[0].clone()),
                right: None,
                boxes,
            }
        // self.left = objects[0].clone();
        // self.right = objects[0].clone();
        } else if object_span == 2 {
            let left = objects[0].clone();
            let right = objects[1].clone();
            let mut box_l = AxisAlignedBoundingBoxes::default();
            let mut box_r = AxisAlignedBoundingBoxes::default();
            if !left.bounding_box(0.0, 0.0, &mut box_l) || !right.bounding_box(0.0, 0.0, &mut box_r)
            {
                println!("No Bounding Box in BVHNode Constructor");
            }
            let boxes = surrounding_box(&box_l, &box_r);

            if box_compare(&objects[0], &objects[1], axis) == Ordering::Less {
                Self {
                    left: Some(left),
                    right: Some(right),
                    boxes,
                }
            } else {
                Self {
                    left: Some(right),
                    right: Some(left),
                    boxes,
                }
            }
        } else {
            objects.sort_by(|a, b| box_compare(a, b, axis));
            let mid = object_span / 2;
            let left = Arc::new(BvhNode::new_from_vector(&mut objects[0..mid], time0, time1));
            let right = Arc::new(BvhNode::new_from_vector(
                &mut objects[mid..object_span],
                time0,
                time1,
            ));
            let mut box_l = AxisAlignedBoundingBoxes::default();
            let mut box_r = AxisAlignedBoundingBoxes::default();
            if !left.bounding_box(0.0, 0.0, &mut box_l) || !right.bounding_box(0.0, 0.0, &mut box_r)
            {
                println!("No Bounding Box in BVHNode Constructor");
            }
            let boxes = surrounding_box(&box_l, &box_r);
            Self {
                left: Some(left),
                right: Some(right),
                boxes,
            }
        }
    }
    pub fn new_from_list(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        Self::new_from_vector(&mut list.objects[..], time0, time1)
    }
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
        let mut hit_left = false;
        let mut closest_so_far = *t_max;
        if self.left.is_some() {
            hit_left = self
                .left
                .as_ref()
                .unwrap()
                .hit(r, t_min, &closest_so_far, rec);
            if hit_left {
                closest_so_far = rec.t;
            }
        }
        let mut hit_right = false;
        if self.right.is_some() {
            hit_right = self
                .right
                .as_ref()
                .unwrap()
                .hit(r, t_min, &closest_so_far, rec);
        }
        hit_left || hit_right
    }
}

pub fn box_compare(
    a: &Arc<dyn Hittable + 'static + Send + Sync>,
    b: &Arc<dyn Hittable + 'static + Send + Sync>,
    axis: i32,
) -> Ordering {
    let mut box_a = AxisAlignedBoundingBoxes::default();
    let mut box_b = AxisAlignedBoundingBoxes::default();
    if !(*a).bounding_box(0.0, 0.0, &mut box_a) || !(*b).bounding_box(0.0, 0.0, &mut box_b) {
        println!("No Bounding Box in BVHNode Constructor");
    }
    f64::partial_cmp(&box_a.minimum[axis], &box_b.minimum[axis]).unwrap()
    // match axis {
    // 	0 => {
    // 		// if box_a.minimum.x_dir < box_b.minimum.x_dir {
    // 		// 	Ordering::Less
    // 		// } else if box_a.minimum.x_dir > box_b.minimum.x_dir {
    // 		// 	Ordering::Greater
    // 		// } else {
    // 		// 	Ordering::Equal
    // 		// }
    // 		f64::partial_cmp(&box_a.minimum.x_dir, &box_b.minimum.x_dir)
    // 	}
    // 	1 => {
    // 		// if box_a.minimum.y_dir < box_b.minimum.y_dir {
    // 		// 	Ordering::Less
    // 		// } else if box_a.minimum.y_dir > box_b.minimum.y_dir {
    // 		// 	Ordering::Greater
    // 		// } else {
    // 		// 	Ordering::Equal
    // 		// }
    // 		f64::partial_cmp(&box_a.minimum.y_dir, &box_b.minimum.y_dir)
    // 	}
    // 	_ => {
    // 		// if box_a.minimum.z_dir < box_b.minimum.z_dir {
    // 		// 	Ordering::Less
    // 		// } else if box_a.minimum.z_dir > box_b.minimum.z_dir {
    // 		// 	Ordering::Greater
    // 		// } else {
    // 		// 	Ordering::Equal
    // 		// }
    // 		f64::partial_cmp(&box_a.minimum.z_dir, &box_b.minimum.z_dir)
    // 	}
    // }.unwrap()
}
