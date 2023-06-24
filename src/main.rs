mod colour;
mod materials;
mod objects;
mod raytracer;
mod scene;
mod structures;

use rand::Rng;
use std::io::{self, Write};

use crate::objects::camera::Camera;
use crate::structures::hittable::HittableList;
use crate::structures::vec3::Vec3;

use crate::colour::write_colour;
use crate::scene::create_random_scene;

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 5;

    // World
    let world: HittableList = create_random_scene();

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
                pixel_colour = raytracer::ray_colour(r, &world, max_depth) + pixel_colour;
            }
            write_colour(&pixel_colour, samples_per_pixel)?;
        }
    }
    io::stderr().write_all(b"Done\n")?;
    Ok(())
}
