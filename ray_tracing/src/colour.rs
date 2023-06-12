use crate::vec3::Vec3;

pub fn write_colour(pixel: Vec3) {
    println!(
        "{} {} {}",
        (255.999 * pixel.x()) as i32,
        (255.999 * pixel.y()) as i32,
        (255.999 * pixel.z()) as i32
    );
}
