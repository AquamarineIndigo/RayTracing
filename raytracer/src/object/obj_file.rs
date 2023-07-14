use super::material::Material;
use super::{BvhNode, Hittable, HittableList};
// use crate::basic::ray::Ray;
use super::Triangle;
use crate::basic::vec3::Vec3;
use std::sync::Arc;
use tobj::{load_obj, LoadOptions};

pub fn obj_file(
    filename: String,
    mat: Arc<dyn Material + 'static + Send + Sync>,
    size: f64,
) -> Vec<Arc<dyn Hittable + Send + Sync>> {
    let obj = load_obj(
        filename,
        &LoadOptions {
            single_index: false,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    );
    let (models, _) = obj.expect("Failed to Load OBJ file");
    let mut objects: Vec<Arc<dyn Hittable + Send + Sync>> = Vec::new();
    for m in models.iter() {
        let indices = &m.mesh.indices;
        let positions = &m.mesh.positions;
        let mut faces = HittableList {
            objects: Vec::new(),
        };
        let mut points: Vec<Vec3> = Vec::new();
        for p in (0..positions.len()).step_by(3) {
            points.push(Vec3::set(
                positions[p] * size,
                positions[p + 1] * size,
                positions[p + 2] * size,
            ));
        }
        for f in (0..indices.len()).step_by(3) {
            faces.add(Triangle::new(
                points[indices[f] as usize],
                points[indices[f + 1] as usize],
                points[indices[f + 2] as usize],
                Arc::clone(&mat),
            ))
        }
        objects.push(Arc::new(BvhNode::new_from_list(&mut faces, 0.0, 1.0)));
    }
    objects
}
