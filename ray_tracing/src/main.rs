mod ray;
mod vec3;

use std::io::{self, Write};

use crate::ray::Ray;
use crate::vec3::Vec3;

fn hit_sphere(centre: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin() - *centre;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - (4.0 * a * c);

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn ray_colour(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return Vec3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);

    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    io::stdout().write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;
    for j in (0i32..image_height).rev() {
        io::stderr().write_all(format!("Scanlines remaining: {j}\n").as_bytes())?;
        for i in 0i32..image_width {
            let u = i as f64 / ((image_width - 1) as f64);
            let v = j as f64 / ((image_height - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let colour: Vec3 = ray_colour(&ray) * 255.999;

            io::stdout()
                .write_all(format!("{} {} {}\n", colour.x(), colour.y(), colour.z()).as_bytes())?;
        }
    }
    io::stderr().write_all(b"Done\n")?;
    Ok(())
}
