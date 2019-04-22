use crate::materials::{Material, Scatter};
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vector,
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &HitRecord) -> Scatter {
        let target = hit.p + hit.normal + Vector::random();
        let scattered = Ray::new(hit.p, target - hit.p);
        Scatter::new(self.albedo, scattered)
    }
}
