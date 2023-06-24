use crate::materials::Material;
use crate::structures::hittable::{HitRecord, Hittable};
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;

pub struct Sphere {
    centre: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            centre,
            radius,
            material,
        }
    }

    pub fn centre(&self) -> Vec3 {
        self.centre
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> Material {
        self.material
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.centre();
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius().powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Nearest root
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.centre()) / self.radius();
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material();

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_struct() {
        // let sphere = Sphere::new(Vec3::new(0.1, 0.5, 0.2), 5.4, material: material::material);
        // assert_eq!(sphere.centre, Vec3::new(0.1, 0.5, 0.2))
    }
}
