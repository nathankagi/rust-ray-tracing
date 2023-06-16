mod camera;
mod colour;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use rand::Rng;
use std::io::{self, Write};

use crate::camera::Camera;
use crate::colour::write_colour;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn ray_colour(r: &Ray, world: &dyn hittable::Hittable, depth: i32) -> Vec3 {
    let mut rec = hittable::HitRecord::new();

    // set limit on number of bounces to limit recursion
    if depth <= 0 {
        return Vec3::zero();
    }

    if world.hit(r, 0.0001, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        return (ray_colour(&Ray::new(rec.p, target - rec.p), world, depth - 1)) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = (unit_direction.y() + 1.0) * 0.5;
    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t)
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world: hittable::HittableList = hittable::HittableList::new();
    world.push(sphere::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(sphere::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let cam = Camera::new();

    //Render
    io::stdout().write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    let mut rng = rand::thread_rng();

    for j in (0i32..image_height).rev() {
        io::stderr().write_all(format!("Scanlines remaining: {j}\n").as_bytes())?;
        for i in 0i32..image_width {
            let mut pixel_colour = Vec3::zero();
            for s in 0i32..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / ((image_width - 1) as f64);
                let v = (j as f64 + rng.gen::<f64>()) / ((image_height - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_colour = ray_colour(&r, &world, max_depth) + pixel_colour;
            }
            write_colour(&pixel_colour, samples_per_pixel)?;
        }
    }
    io::stderr().write_all(b"Done\n")?;
    Ok(())
}
