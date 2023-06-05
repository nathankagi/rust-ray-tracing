mod ray;
mod vec3;

use std::io::{self, Write};

use crate::ray::Ray;
use crate::vec3::Vec3;

fn ray_colour(&r: Ray) -> f64 {
    unit_direction: Vec3 = r.direction().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn main() -> io::Result<()> {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    //Render
    io::stdout().write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;
    for j in (0i32..image_height).rev() {
        io::stderr().write_all(format!("Scanlines remaining: {j}\n").as_bytes())?;
        for i in 0i32..image_width {
            let colour = Vec3::new(
                255.999 * i as f64 / (image_width as f64 - 1.0),
                255.999 * j as f64 / (image_height as f64 - 1.0),
                0.25,
            );

            io::stdout()
                .write_all(format!("{} {} {}\n", colour.x(), colour.y(), colour.z()).as_bytes())?;
        }
    }
    io::stderr().write_all(b"Done\n")?;
    Ok(())
}
