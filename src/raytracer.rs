use crate::materials::Scatterable;
use crate::objects::camera::Camera;
use crate::structures::hittable::{HitRecord, Hittable};
use crate::structures::ray::Ray;
use crate::structures::vec3::Vec3;

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
    (Vec3::one() * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t)
}

pub fn render(
    image_height: i32,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    world: impl Hittable + std::marker::Sync,
    cam: Camera,
) -> Vec<Vec<Vec3>> {
    eprintln!("Rendering {} pixels", (image_height * image_width));
    let bar = ProgressBar::new((image_height * image_width).try_into().unwrap());

    let image: Vec<Vec<Vec3>> = (0i32..image_height)
        .into_par_iter()
        .map(|x| {
            let j = image_height - x - 1;
            let row: Vec<Vec3> = (0i32..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_colour = Vec3::zero();

                    for _ in 0i32..samples_per_pixel {
                        let u = (i as f64 + rand::random::<f64>()) / ((image_width - 1) as f64);
                        let v = (j as f64 + rand::random::<f64>()) / ((image_height - 1) as f64);

                        let r = cam.get_ray(u, v);
                        pixel_colour = ray_colour(r, &world, max_depth) + pixel_colour;
                    }
                    bar.inc(1);
                    pixel_colour.to_colour(samples_per_pixel)
                })
                .collect();
            row
        })
        .collect();

    bar.finish();
    image
}
