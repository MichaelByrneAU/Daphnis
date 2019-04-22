#![allow(clippy::many_single_char_names)]

use crate::materials::Material;
use crate::objects::{HitRecord, Object};
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        let a = Vector::dot(&r.direction, &r.direction);
        let b = Vector::dot(&oc, &r.direction);
        let c = Vector::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let soln = (-b - (b * b - a * c).sqrt()) / a;

            let t;
            let p;
            let normal;

            if (soln < t_max) && (soln > t_min) {
                t = soln;
                p = r.point_at_parameter(soln);
                normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(t, p, normal, &*self.material));
            }
            let soln = (-b + (b * b - a * c).sqrt()) / a;
            if (soln < t_max) && (soln > t_min) {
                t = soln;
                p = r.point_at_parameter(soln);
                normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(t, p, normal, &*self.material));
            }
            None
        } else {
            None
        }
    }
}
