use std::f64::consts::PI;

use crate::ray::{random_in_unit_sphere, Ray};
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        v_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = v_fov * (PI / 180.0);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u);
        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
