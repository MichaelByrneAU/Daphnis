use std::f64::consts::PI;

use crate::ray::Ray;
use crate::vector::Vector;

pub struct Orientation {
    pub look_from: Vector,
    pub look_at: Vector,
    pub v_up: Vector,
}

impl Orientation {
    pub fn new(look_from: Vector, look_at: Vector, v_up: Vector) -> Orientation {
        Orientation {
            look_from,
            look_at,
            v_up,
        }
    }
}

pub struct Lens {
    pub v_fov: f64,
    pub aperture: f64,
    pub focus_dist: f64,
}

impl Lens {
    pub fn new(v_fov: f64, aperture: f64, focus_dist: f64) -> Lens {
        Lens {
            v_fov,
            aperture,
            focus_dist,
        }
    }
}

pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    lens_radius: f64,
}

impl Camera {
    pub fn new(orientation: Orientation, lens: Lens, aspect: f64) -> Camera {
        // Redefine inputs
        let look_from = orientation.look_from;
        let look_at = orientation.look_at;
        let v_up = orientation.v_up;

        let v_fov = lens.v_fov;
        let aperture = lens.aperture;
        let focus_dist = lens.focus_dist;

        // Convert FOV to radians
        let theta = v_fov * (PI / 180.0);

        // Define FOV in both dimensions of image
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        // Define orthonormal basis for camera orientation
        let w = (look_from - look_at).unit();
        let u = Vector::cross(&v_up, &w).unit();
        let v = Vector::cross(&w, &u);

        // Important locations
        let origin = look_from;
        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        // Define lens radius
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // Generate a random position on a disk around the origin
        let rd = self.lens_radius * Vector::random();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
