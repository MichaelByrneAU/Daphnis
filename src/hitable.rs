use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList(pub Vec<Box<dyn Hitable>>);

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        for hitable in self.0.iter() {
            if let Some(hit) = hitable.hit(r, t_min, t_max) {
                match closest_hit {
                    None => closest_hit = Some(hit),
                    Some(prev_hit) if hit.t < prev_hit.t => closest_hit = Some(hit),
                    Some(_) => {}
                }
            }
        }
        closest_hit
    }
}
