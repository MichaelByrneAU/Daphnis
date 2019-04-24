use crate::materials::Material;
use crate::ray::Ray;
use crate::vector::Vector;

mod sphere;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector,
    pub normal: Vector,
    pub material: Material,
}

impl HitRecord {
    pub fn new(t: f64, p: Vector, normal: Vector, material: Material) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

pub enum Object {
    Sphere {
        center: Vector,
        radius: f64,
        material: Material,
    },
    Multiple(Vec<Object>),
}

impl Object {
    pub fn new_sphere(center: Vector, radius: f64, material: Material) -> Object {
        Object::Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere {
                center,
                radius,
                material,
            } => sphere::hit(*center, *radius, *material, r, t_min, t_max),
            Object::Multiple(objects) => hit(&objects, r, t_min, t_max),
        }
    }
}

fn hit(objects: &Vec<Object>, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest_hit: Option<HitRecord> = None;
    for hitable in objects.iter() {
        if let Some(hit) = hitable.hit(r, t_min, t_max) {
            match closest_hit {
                None => closest_hit = Some(hit),
                Some(prev_hit) => {
                    if hit.t < prev_hit.t {
                        closest_hit = Some(hit)
                    }
                }
            }
        }
    }
    closest_hit
}
