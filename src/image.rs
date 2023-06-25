use std::io::{self, Write};

use crate::structures::vec3::Vec3;

pub fn save_ppm(image: Vec<Vec<Vec3>>) {
    let image_height = image[0].len();
    let image_width = image.len();

    let _ =
        io::stdout().write_all(format!("P3\n{} {}\n255\n", image_height, image_width).as_bytes());
    for row in image.iter() {
        for each in row.iter() {
            let _ = io::stdout()
                .write_all(format!("{} {} {}\n", each.x(), each.y(), each.z()).as_bytes());
        }
    }
}
