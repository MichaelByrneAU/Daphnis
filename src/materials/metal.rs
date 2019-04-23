use crate::materials::Scatter;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector;

pub fn scatter(albedo: Vector, fuzz: f64, r_in: &Ray, hit: &HitRecord) -> Scatter {
    let reflected = reflect(&r_in.direction.unit(), &hit.normal);
    let scattered = Ray::new(hit.p, reflected + fuzz * Vector::random());
    Scatter::new(albedo, scattered)
}

fn reflect(v: &Vector, n: &Vector) -> Vector {
    *v - 2.0 * Vector::dot(&v, n) * *n
}
