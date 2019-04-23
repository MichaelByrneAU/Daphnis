use crate::materials::Scatter;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector;

pub fn scatter(albedo: Vector, _r_in: &Ray, hit: &HitRecord) -> Scatter {
    let target = hit.p + hit.normal + Vector::random();
    let scattered = Ray::new(hit.p, target - hit.p);
    Scatter::new(albedo, scattered)
}
