mod camera;
mod colour;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use material::{Dielectric, Lambertian, Material, Metal, Scatterable};
use rand::Rng;
use std::io::{self, Write};

use crate::camera::Camera;
use crate::colour::write_colour;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn ray_colour(r: Ray, world: &dyn hittable::Hittable, depth: i32) -> Vec3 {
    let mut rec = hittable::HitRecord::new();

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

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 20;
    let max_depth = 10;

    // World
    let mut world: hittable::HittableList = hittable::HittableList::new();

    // material
    let material_ground = Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_centre = Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Material::Dielectric(Dielectric::new(1.5));
    let material_right = Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    // objects
    world.push(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_centre));
    world.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left));
    world.push(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

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
                pixel_colour = ray_colour(r, &world, max_depth) + pixel_colour;
            }
            write_colour(&pixel_colour, samples_per_pixel)?;
        }
    }
    io::stderr().write_all(b"Done\n")?;
    Ok(())
}
