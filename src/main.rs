use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vec3::dot;

use crate::{color::Color, vec3::Vec3};

mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

fn ray_color(r: &Ray) -> Color {
    let sphere = Sphere::new(Vec3::new(0, 0, -1), 0.5);
    if let Some(hit_record) = sphere.hit(r, &(0.0..=f64::MAX)) {
        let surface_normal = (r.at(hit_record.t) - sphere.center()).normalized();
        return Color::from_rgb_float(
            0.5 * Vec3::new(
                surface_normal.x + 1.0,
                surface_normal.y + 1.0,
                surface_normal.z + 1.0,
            ),
        );
    }
    let unit_direction = r.direction().normalized();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::from_rgb_float((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camara_center = Vec3::zero();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camara_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + i * pixel_delta_u + j * pixel_delta_v;
            let ray_direction = pixel_center - camara_center;
            let ray = Ray::new(camara_center, ray_direction);
            let color = ray_color(&ray);
            println!("{color}");
        }
    }
}
