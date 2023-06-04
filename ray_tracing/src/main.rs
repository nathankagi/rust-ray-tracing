mod vec3;

use std::io::{self, Write};

use vec3::Vec3;

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
