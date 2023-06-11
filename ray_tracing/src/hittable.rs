mod ray;
mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hittable {
    pub fn hit(r: &Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool {}
}

pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64) -> HitRecord {
        return HitRecord { p, normal, t };
    }

    pub fn set_face_normal(r: &Ray, outward_normal: &Vec3) {
        front_face = Vec3::dot(r.direction(), outward_normal) < 0;
        normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct HittableList {
    list: Vec<Hittable>,
}

impl HittableList {
    pub fn new(list: Vec<Box<Hittable>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool {
        let temp_rec: HitRecord;
        let hit_anything = false;
        let closest_so_far = t_max;

        for each in self.list.item() {
            if (each.hit(r, t_min, closest_so_far, temp_rec)) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }

        return hit_anything;
        hit_anything
    }
}
