use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector;

mod dielectric;
mod lambertian;
mod metal;

#[derive(Copy, Clone, Debug)]
pub struct Scatter {
    pub attenuation: Vector,
    pub ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Vector, ray: Ray) -> Scatter {
        Scatter { attenuation, ray }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Dielectric { ref_idx: f64 },
    Lambertian { albedo: Vector },
    Metal { albedo: Vector, fuzz: f64 },
}

impl Material {
    pub fn new_dielectric(ref_idx: f64) -> Material {
        Material::Dielectric { ref_idx }
    }

    pub fn new_lambertian(albedo: Vector) -> Material {
        Material::Lambertian { albedo }
    }

    pub fn new_metal(albedo: Vector, fuzz: f64) -> Material {
        Material::Metal { albedo, fuzz }
    }

    pub fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        match *self {
            Material::Dielectric { ref_idx } => dielectric::scatter(ref_idx, r_in, hit),
            Material::Lambertian { albedo } => lambertian::scatter(albedo, r_in, hit),
            Material::Metal { albedo, fuzz } => metal::scatter(albedo, fuzz, r_in, hit),
        }
    }
}
