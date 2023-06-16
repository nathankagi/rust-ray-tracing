use crate::vec3::Vec3;
use std::io::{self, Write};

pub fn write_colour(pixel_colour: &Vec3, samples_per_pixel: i32) -> io::Result<()> {
    let scale = 1.0 / (samples_per_pixel as f64);

    let r = pixel_colour.x() * scale;
    let g = pixel_colour.y() * scale;
    let b = pixel_colour.z() * scale;

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
