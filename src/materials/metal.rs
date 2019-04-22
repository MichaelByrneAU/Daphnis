use crate::materials::{Material, Scatter};
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Debug)]
pub struct Metal {
    pub albedo: Vector,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector, fuzz: f64) -> Metal {
        let fuzz = if fuzz > 1.0 { 1.0 } else { fuzz };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        let reflected = reflect(&r_in.direction.unit_vector(), &hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * Vector::random());
        Scatter::new(self.albedo, scattered)
    }
}

fn reflect(v: &Vector, n: &Vector) -> Vector {
    *v - 2.0 * Vector::dot(&v, n) * *n
}
