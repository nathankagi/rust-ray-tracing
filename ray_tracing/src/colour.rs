use crate::vec3::Vec3;
use std::io::{self, Write};

pub fn write_colour(colour: &Vec3) -> io::Result<()> {
    let c = *colour * 255.999;
    io::stdout().write_all(format!("{} {} {}\n", c.x(), c.y(), c.z()).as_bytes())?;
    Ok(())
}
