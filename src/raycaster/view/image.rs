// Inside view/image.rs
use crate::raycaster::model::vector::Vec3;
use std::fs::File;
use std::io::Write;

pub fn save_image(width: u32, height: u32, buffer: Vec<Vec<Vec3>>) {
    let filename = "output.ppm";
    let mut file = File::create(filename).expect("Unable to create file");

    // Write the PPM header
    writeln!(file, "P3").expect("Unable to write header");
    writeln!(file, "{} {}", width, height).expect("Unable to write dimensions");
    writeln!(file, "255").expect("Unable to write max color value");

    // Write the pixel data
    for j in 0..height {
        for i in 0..width {
            let pixel = &buffer[j as usize][i as usize];
            let r = (pixel.x * 255.0).clamp(0.0, 255.0) as u32;
            let g = (pixel.y * 255.0).clamp(0.0, 255.0) as u32;
            let b = (pixel.z * 255.0).clamp(0.0, 255.0) as u32;
            writeln!(file, "{} {} {}", r, g, b).expect("Unable to write pixel data");
        }
    }

    println!("Image saved to {}", filename);
}
