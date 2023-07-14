use super::{random_double, random_range};

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
        Vec3::set(random_double(), random_double(), random_double())
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
        if vec3_dot(&p, &p) >= 1.0 {
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
    vec3_sub(v, &vec3_mul(&(2.0 * vec3_dot(v, n)), n))
}

pub fn refract(uv: &Vec3, n: &Vec3, eta_i_over_eta_t: &f64) -> Vec3 {
    let mut cos_theta: f64 = 1.0;
    let dt: f64 = vec3_dot(&vec3_mul(&-1.0, uv), n);
    if dt < cos_theta {
        cos_theta = dt;
    }
    let r_out_perpendicular: Vec3 =
        vec3_mul(eta_i_over_eta_t, &vec3_add(uv, &vec3_mul(&cos_theta, n)));
    let r_out_parallel: Vec3 = vec3_mul(
        &(-(1.0 - vec3_dot(&r_out_perpendicular, &r_out_perpendicular))
            .abs()
            .sqrt()),
        n,
    );
    vec3_add(&r_out_perpendicular, &r_out_parallel)
}
