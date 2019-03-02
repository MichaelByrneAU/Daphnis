use rand::Rng;

use crate::geometry::{Ray, Vec3};
use crate::materials::{Material, Scatter};
use crate::objects::HitRecord;

#[derive(Debug)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        let reflected = reflect(&r_in.direction, &hit.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let outward_normal;
        let ni_over_nt;
        let cosine;

        if r_in.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction.dot(hit.normal) / r_in.direction.length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction.dot(hit.normal) / r_in.direction.length();
        };

        let mut rng = rand::thread_rng();
        let prob = rng.gen_range(0.0, 1.0);
        let reflect_prob = schlick(cosine, self.ref_idx);

        if prob < reflect_prob {
            Scatter::new(attenuation, Ray::new(hit.p, reflected))
        } else {
            let refraction = refract(&r_in.direction, &outward_normal, ni_over_nt);
            match refraction {
                Some(refracted) => Scatter::new(attenuation, Ray::new(hit.p, refracted)),
                None => Scatter::new(attenuation, Ray::new(hit.p, reflected)),
            }
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(*n) * *n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(*n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
