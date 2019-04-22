use crate::vector::Vector;
use crate::materials::Material;
use crate::ray::Ray;

mod sphere;

pub use crate::objects::sphere::Sphere;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vector,
    pub normal: Vector,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, p: Vector, normal: Vector, material: &'a Material) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Object {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct ObjectList(pub Vec<Box<dyn Object>>);

impl Object for ObjectList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        for hitable in self.0.iter() {
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
}
