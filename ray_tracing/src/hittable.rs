use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(r.direction(), *outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = *outward_normal * -1.0;
        };
    }
}

pub struct HittableList {
    items: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { items: Vec::new() }
    }

    pub fn push(&mut self, item: impl Hittable + 'static) {
        self.items.push(Box::new(item));
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for each in self.items.iter() {
            if each.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
            }
        }

        hit_anything
    }
}

#[cfg(test)]
mod tests {
    use crate::sphere::Sphere;

    use super::*;

    #[test]
    fn test_hit_record() {
        let hr = HitRecord::new();
        assert_eq!(
            hr,
            HitRecord {
                p: Vec3::new(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 0.0),
                t: 0.0,
                front_face: false,
            }
        )
    }

    #[test]
    fn test_hit_record_set_face_norm() {}

    #[test]
    fn test_hittable_list() {
        let mut list = HittableList::new();

        let sphere = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 4.7);
        list.push(sphere);
    }

    #[test]
    fn test_hittable_list_hit() {}
}
