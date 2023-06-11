mod hittable;
mod vec3;

use crate::hittable;
use crate::vec3::Vec3;

pub struct Sphere {
    centre: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64) -> Sphere {
        return Sphere { centre, radius };
    }
}

impl Hittable for Sphere {
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool {
        let oc: Vec3 = r.origin() - centre;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - radius * radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Nearest root
        let root = -(half_b - sqrtd) / a;
        if (root < t_min || t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min || t_max < root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = Vec3::new(rec.p - centre) / radius;
        rec.set_face_normal(r, outward_normal);
        return true;
    }
}
