use std::f64::consts::PI;

use crate::basic::ray; // as ray;
use crate::basic::vec3;
use crate::basic::vec3::vec3_add;
use crate::basic::vec3::vec3_mul;
use crate::basic::vec3::vec3_sub;
// as vector3;
use crate::basic::vec3::Vec3;
use crate::object::aabb::{surrounding_box, AxisAlignedBoundingBoxes};
use crate::object::hittable; // as hitable;
use crate::object::material::Materials;

#[derive(Clone)]
pub struct Sphere {
    pub center: vec3::Vec3,
    pub radius: f64,
    pub material: Materials,
}

impl Sphere {
    pub fn set(cen: vec3::Vec3, r: f64, mat: &Materials) -> Self {
        Self {
            center: cen,
            radius: r,
            material: mat.clone(),
        }
    }
    fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y_dir).acos();
        let phi = p.x_dir.atan2(-p.z_dir) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut hittable::HitRecord) -> bool {
        let oc = vec3::vec3_sub(&r.origin, &self.center);
        let a: f64 = vec3::vec3_dot(&r.direction, &r.direction);
        let half_b: f64 = vec3::vec3_dot(&oc, &r.direction); // b -> half_b
        let c: f64 = vec3::vec3_dot(&oc, &oc) - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = (half_b * half_b - a * c).sqrt();
        let mut root: f64 = (-half_b - sqrt_d) / a;
        if root < *t_min || *t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < *t_min || *t_max < root {
                return false;
            }
        }
        let point_at = r.point_at_parameter(&root);
        rec.get_value(
            root,
            point_at,
            vec3::vec3_mul(
                &(1.0 / self.radius),
                &vec3::vec3_sub(&point_at, &self.center),
            ),
            &self.material,
        );
        let outward_normal: vec3::Vec3 = vec3::vec3_mul(
            &(1.0 / self.radius),
            &vec3::vec3_sub(&point_at, &self.center),
        );
        rec.set_face_normal(r, &outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        true
    }

    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool {
        output_box.set(
            vec3_sub(
                &self.center,
                &Vec3::set(self.radius, self.radius, self.radius),
            ),
            vec3_add(
                &self.center,
                &Vec3::set(self.radius, self.radius, self.radius),
            ),
        );
        true
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Materials,
}

impl MovingSphere {
    pub fn set(
        center0: &Vec3,
        center1: &Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: &Materials,
    ) -> Self {
        Self {
            center0: *center0,
            center1: *center1,
            time0,
            time1,
            radius,
            material: material.clone(),
        }
    }
    pub fn center(&self, tm: f64) -> Vec3 {
        vec3_add(
            &self.center0,
            &vec3_mul(
                &((tm - self.time0) / (self.time1 - self.time0)),
                &vec3_sub(&self.center1, &self.center0),
            ),
        )
    }
}

impl hittable::Hittable for MovingSphere {
    fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut hittable::HitRecord) -> bool {
        let oc = vec3::vec3_sub(&r.origin, &self.center(r.tm));
        let a: f64 = vec3::vec3_dot(&r.direction, &r.direction);
        let half_b: f64 = vec3::vec3_dot(&oc, &r.direction); // b -> half_b
        let c: f64 = vec3::vec3_dot(&oc, &oc) - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = (half_b * half_b - a * c).sqrt();
        let mut root: f64 = (-half_b - sqrt_d) / a;
        if root < *t_min || *t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < *t_min || *t_max < root {
                return false;
            }
        }
        let point_at = r.point_at_parameter(&root);
        let outward_normal: Vec3 = vec3_mul(
            &(1.0 / self.radius),
            &vec3::vec3_sub(&point_at, &self.center(r.tm)),
        );
        rec.get_value(root, point_at, outward_normal, &self.material);
        rec.set_face_normal(r, &outward_normal);
        true
    }

    fn bounding_box(
        &self,
        time0: f64,
        time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool {
        let box0 = AxisAlignedBoundingBoxes::new(
            vec3_sub(
                &self.center(time0),
                &Vec3::set(self.radius, self.radius, self.radius),
            ),
            vec3_add(
                &self.center(time0),
                &Vec3::set(self.radius, self.radius, self.radius),
            ),
        );
        let box1 = AxisAlignedBoundingBoxes::new(
            vec3_sub(
                &self.center(time1),
                &Vec3::set(self.radius, self.radius, self.radius),
            ),
            vec3_add(
                &self.center(time1),
                &Vec3::set(self.radius, self.radius, self.radius),
            ),
        );
        let sur = surrounding_box(&box0, &box1);
        output_box.copy_from_other(&sur);
        true
    }
}
