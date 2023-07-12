use super::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub direction: Vec3,
    pub origin: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn set(origin: Vec3, direction: Vec3) -> Self {
        Self {
            direction, //: direction.clone(),
            origin,    //: origin.clone(),
            tm: 0.0,
        }
    }

    pub fn set_with_time(origin: Vec3, direction: Vec3, tm: f64) -> Self {
        Self {
            direction,
            origin,
            tm,
        }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        // self.origin + (self.direction * t)
        self.origin + t * self.direction
    }

    pub fn copy_ray(&mut self, other: &Ray) -> &Self {
        [self.direction, self.origin] = [other.direction, other.origin];
        self.tm = other.tm;
        self
    }
    pub fn set_value(&mut self, origin: Vec3, direction: Vec3, tm: f64) -> &Self {
        [self.direction, self.origin] = [direction, origin];
        self.tm = tm;
        self
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            direction: Vec3::set(0.0, 0.0, 0.0),
            origin: Vec3::set(0.0, 0.0, 0.0),
            tm: 0.0,
        }
    }
}
