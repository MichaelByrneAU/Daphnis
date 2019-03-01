use std::fmt::Debug;

use rand::Rng;

use crate::hitable::HitRecord;
use crate::ray::{random_in_unit_sphere, Ray};
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Vec3, ray: Ray) -> Scatter {
        Scatter { attenuation, ray }
    }
}

pub trait Material: Debug {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &HitRecord) -> Scatter {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        let attenuation = self.albedo;
        Scatter::new(attenuation, scattered)
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(*n) * *n
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        let fuzz = if fuzz > 1.0 { 1.0 } else { fuzz };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        let reflected = reflect(&r_in.direction.unit_vector(), &hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;
        Scatter::new(attenuation, scattered)
    }
}

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

        let (outward_normal, ni_over_nt, cosine) = if r_in.direction.dot(hit.normal) > 0.0 {
            (
                -hit.normal,
                self.ref_idx,
                self.ref_idx * r_in.direction.dot(hit.normal) / r_in.direction.length(),
            )
        } else {
            (
                hit.normal,
                1.0 / self.ref_idx,
                -r_in.direction.dot(hit.normal) / r_in.direction.length(),
            )
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
