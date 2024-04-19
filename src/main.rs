use hittables::Sphere;
use math::Vec3;

use crate::{camera::Camera, hittables::HittableList};

mod camera;
mod color;
mod hittables;
mod math;

fn main() {
    // World
    let mut world = HittableList::default();
    world.push(Sphere::new(Vec3::new(0, 0, -1), 0.5));
    world.push(Sphere::new(Vec3::new(0, -100.5, -1), 100.0));

    let camera = Camera::builder()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .build();

    camera.render(&world);
}
