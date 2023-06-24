use crate::materials::Scatterable;
use crate::structures::hittable::{HitRecord, Hittable};
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;

pub fn ray_colour(r: Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new();

    // set limit on number of bounces to limit recursion
    if depth <= 0 {
        return Vec3::zero();
    }

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Vec3::zero();

        if rec
            .material
            .scatter(&r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_colour(scattered, world, depth - 1);
        }

        return Vec3::zero();
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = (unit_direction.y() + 1.0) * 0.5;
    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t)
}
