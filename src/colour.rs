use std::io::{self, Write};

use crate::structures::vec3::Vec3;

pub fn write_colour(pixel_colour: &Vec3, samples_per_pixel: i32) -> io::Result<()> {
    let mut r = pixel_colour.x();
    let mut g = pixel_colour.y();
    let mut b = pixel_colour.z();

    let scale = 1.0 / (samples_per_pixel as f64);

    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    io::stdout().write_all(
        format!(
            "{} {} {}\n",
            256.0 * r.clamp(0.0, 0.999),
            256.0 * g.clamp(0.0, 0.999),
            256.0 * b.clamp(0.0, 0.999)
        )
        .as_bytes(),
    )?;
    Ok(())
}
