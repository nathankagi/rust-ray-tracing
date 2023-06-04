use vec3;

pub fn write_colour(pixel: vec3::Vec3) {
    println!(
        "{} {} {}",
        (255.99 * pixel.x()) as i32,
        (255.99 * pixel.y()) as i32,
        (255.99 * pixel.z()) as i32
    );
}
