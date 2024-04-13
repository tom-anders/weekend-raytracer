fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_width - 1) as f64;
            let b = 0.0;

            let ir = (r * 255.999) as u8;
            let ig = (g * 255.999) as u8;
            let ib = (b * 255.999) as u8;

            println!("{ir} {ig} {ib}");
        }
    }
}
