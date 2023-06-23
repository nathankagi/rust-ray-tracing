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
    let aspect_ratio = 3.0 / 2.0;
    let image_width: i32 = 1200;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world: hittable::HittableList = create_random_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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

fn create_random_scene() -> hittable::HittableList {
    let mut rng = rand::thread_rng();
    let mut world = hittable::HittableList::new();

    let ground_material = Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..12 {
        for b in -11..12 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    let material = Material::Lambertian(Lambertian::new(albedo));
                    world.push(Sphere::new(center, 0.2, material));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rng.gen::<f64>();
                    let material = Material::Metal(Metal::new(albedo, fuzz));
                    world.push(Sphere::new(center, 0.2, material));
                } else {
                    let material = Material::Dielectric(Dielectric::new(1.5));
                    world.push(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    // fixed objects
    let material1 = Material::Dielectric(Dielectric::new(1.5));
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}
