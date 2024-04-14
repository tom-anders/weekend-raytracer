use crate::color::Color;

mod color;
mod ray;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            println!(
                "{}",
                &Color::from_rgb_float(
                    i as f64 / (image_width - 1) as f64,
                    j as f64 / (image_height - 1) as f64,
                    0.0,
                ),
            );
        }
    }
}
