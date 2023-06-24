pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::structures::hittable::HitRecord;
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;

use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

pub trait Scatterable {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

impl Scatterable for Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
            Material::Metal(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
            Material::Dielectric(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
        }
    }
}
