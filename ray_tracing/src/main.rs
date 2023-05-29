use std::io::{self, Write};

mod vec3;

fn main() -> io::Result<()> {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    //Render
    io::stdout().write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;
    for j in (0i32..image_height).rev() {
        io::stderr().write_all(format!("Scanlines remaining: {j}\n").as_bytes())?;
        for i in 0i32..image_width {
            let r: f32 = i as f32 / (image_width as f32 - 1.0);
            let g: f32 = j as f32 / (image_height as f32 - 1.0);
            let b: f32 = 0.25;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            io::stdout().write_all(format!("{ir} {ig} {ib}\n").as_bytes())?;
        }
    }
    io::stderr().write_all(b"Done\n")?;
    Ok(())
}
