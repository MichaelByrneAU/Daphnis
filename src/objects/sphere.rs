#![allow(clippy::many_single_char_names)]

use crate::objects::{HitRecord, Material};
use crate::ray::Ray;
use crate::vector::Vector;

pub fn hit(
    center: Vector,
    radius: f64,
    material: Material,
    r: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord> {
    let oc = r.origin - center;

    let a = Vector::dot(&r.direction, &r.direction);
    let b = Vector::dot(&oc, &r.direction);
    let c = Vector::dot(&oc, &oc) - radius * radius;

    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
        let soln = (-b - (b * b - a * c).sqrt()) / a;

        let t;
        let p;
        let normal;

        if (soln < t_max) && (soln > t_min) {
            t = soln;
            p = r.point_at_parameter(soln);
            normal = (p - center) / radius;
            return Some(HitRecord::new(t, p, normal, material));
        }
        let soln = (-b + (b * b - a * c).sqrt()) / a;
        if (soln < t_max) && (soln > t_min) {
            t = soln;
            p = r.point_at_parameter(soln);
            normal = (p - center) / radius;
            return Some(HitRecord::new(t, p, normal, material));
        }
        None
    } else {
        None
    }
}
