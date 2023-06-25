mod colour;
mod image;
mod materials;
mod objects;
mod raytracer;
mod scene;
mod structures;

use std::io;

use crate::objects::camera::Camera;
use crate::scene::create_random_scene;
use crate::structures::hittable::HittableList;
use crate::structures::vec3::Vec3;

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
    let lookat = Vec3::zero();
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
    let colour_matrix = raytracer::render(
        image_height,
        image_width,
        samples_per_pixel,
        max_depth,
        world,
        cam,
    );

    image::save_ppm(colour_matrix);

    Ok(())
}
