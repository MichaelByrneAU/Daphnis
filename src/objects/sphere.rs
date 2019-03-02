use crate::geometry::{Ray, Vec3};
use crate::materials::Material;
use crate::objects::{HitRecord, Object};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
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

        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

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
