extern crate rand;

use rand::Rng;

use crate::structures::hittable::HitRecord;
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;

use crate::materials::Scatterable;

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    pub fn reflectance(&self, cosine: f64, refraction_ratio: f64) -> f64 {
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().unit_vector();

        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || self.reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>()
        {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);

        true
    }
}
