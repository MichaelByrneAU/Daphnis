use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let soln = (-b - (b * b - a * c).sqrt()) / a;
            if (soln < t_max) && (soln > t_min) {
                let t = soln;
                let p = r.point_at_parameter(soln);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(t, p, normal));
            }
            let soln = (-b + (b * b - a * c).sqrt()) / a;
            if (soln < t_max) && (soln > t_min) {
                let t = soln;
                let p = r.point_at_parameter(soln);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(t, p, normal));
            }
            None
        } else {
            None
        }
    }
}
