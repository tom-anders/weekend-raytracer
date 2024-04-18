use hittables::{Sphere, Hittable};
use math::{Ray, Vec3};

use color::Color;

use crate::{camera::Camera, hittables::HittableList};

mod camera;
mod color;
mod hittables;
mod math;

fn main() {
    // World
    let mut world = HittableList::default();
    world.push(Box::new(Sphere::new(Vec3::new(0, 0, -1), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0, -100.5, -1), 100.0)));

    let camera = Camera::new(16.0 / 9.0, 400, 100);

    camera.render(&world);
}
