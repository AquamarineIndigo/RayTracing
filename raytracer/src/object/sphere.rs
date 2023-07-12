use std::f64::consts::PI;

use crate::basic::ray; // as ray;
use crate::basic::vec3::{vec3_dot, vec3_mul, vec3_sub, Vec3};
// as vector3;
use crate::object::aabb::{surrounding_box, AxisAlignedBoundingBoxes};
use crate::object::hittable; // as hitable;
use crate::object::material::Material;

#[derive(Clone)]
pub struct Sphere<T: Material + Clone> {
    pub center: Vec3,
    pub radius: f64,
    pub material: T,
}

impl<T: Material + Clone> Sphere<T> {
    pub fn set(cen: Vec3, r: f64, mat: &T) -> Self {
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

impl<T: Material + Clone + 'static> hittable::Hittable for Sphere<T> {
    fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut hittable::HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a: f64 = vec3_dot(&r.direction, &r.direction);
        let half_b: f64 = vec3_dot(&oc, &r.direction); // b -> half_b
        let c: f64 = vec3_dot(&oc, &oc) - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root: f64 = (-half_b - sqrt_d) / a;
        if root < *t_min || *t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < *t_min || *t_max < root {
                return false;
            }
        }
        let point_at = r.point_at_parameter(root);
        rec.get_value(root, point_at, Vec3::set(0.0, 0.0, 0.0), &self.material);
        let outward_normal: Vec3 = (point_at - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Sphere::<T>::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        // print!("[{}, {}, {}]\t", point_at.x_dir, point_at.y_dir, point_at.z_dir);
        true
    }

    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
        output_box: &mut AxisAlignedBoundingBoxes,
    ) -> bool {
        output_box.set(self.center - self.radius, self.center + self.radius);
        true
    }
}

#[derive(Clone)]
pub struct MovingSphere<T: Material + Clone> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: T,
}

impl<T: Material + Clone> MovingSphere<T> {
    pub fn set(
        center0: &Vec3,
        center1: &Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: &T,
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
        self.center0
            - vec3_mul(
                &((tm - self.time0) / (self.time1 - self.time0)),
                &vec3_sub(&self.center1, &self.center0),
            )
    }
}

impl<T: Material + Clone + 'static> hittable::Hittable for MovingSphere<T> {
    fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut hittable::HitRecord) -> bool {
        let oc = vec3_sub(&r.origin, &self.center(r.tm));
        let a: f64 = vec3_dot(&r.direction, &r.direction);
        let half_b: f64 = vec3_dot(&oc, &r.direction); // b -> half_b
        let c: f64 = vec3_dot(&oc, &oc) - self.radius * self.radius;
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
        let point_at = r.point_at_parameter(root);
        let outward_normal: Vec3 = vec3_mul(
            &(1.0 / self.radius),
            &vec3_sub(&point_at, &self.center(r.tm)),
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
            self.center(time0) - self.radius,
            self.center(time0) + self.radius,
        );
        let box1 = AxisAlignedBoundingBoxes::new(
            self.center(time1) - self.radius,
            self.center(time1) + self.radius,
        );
        let sur = surrounding_box(&box0, &box1);
        output_box.copy_from_other(&sur);
        true
    }
}
