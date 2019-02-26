use rand::Rng;

use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX);
    let mut rng = rand::thread_rng();
    loop {
        if point.squared_length() < 1.0 {
            break;
        }
        point = Vec3::new(
            rng.gen_range(-0.5, 0.5),
            rng.gen_range(-0.5, 0.5),
            rng.gen_range(-0.5, 0.5),
        );
    }
    point
}
