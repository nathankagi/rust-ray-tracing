mod ray;
mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64) -> HitRecord {
        return HitRecord { p, normal, t };
    }
}

pub trait Hittable {
    pub fn hit(r: &Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool {}
}
