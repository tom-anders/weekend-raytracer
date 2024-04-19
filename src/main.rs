use color::Color;
use hittables::Sphere;
use material::{Dielectric, Lambertian, Metal};
use math::Vec3;

use crate::{camera::Camera, hittables::HittableList};

mod camera;
mod color;
mod hittables;
mod material;
mod math;

fn main() {
    let mut world = HittableList::default();

    let material_ground = Lambertian::new(Color::from_rgb_float(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Lambertian::new(Color::from_rgb_float(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Color::from_rgb_float(Vec3::new(0.8, 0.6, 0.2)), 1.0);

    world.push(Sphere::new(
        Vec3::new(0, -100.5, -1),
        100.0,
        material_ground.into(),
    ));
    world.push(Sphere::new(
        Vec3::new(0, 0, -1.2),
        0.5,
        material_center.into(),
    ));
    world.push(Sphere::new(Vec3::new(-1, 0, -1), 0.5, material_left.into()));
    world.push(Sphere::new(Vec3::new(-1, 0, -1), 0.4, material_bubble.into()));
    world.push(Sphere::new(Vec3::new(1, 0, -1), 0.5, material_right.into()));

    let camera = Camera::builder()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov_degrees(20.0)
        .look_from(Vec3::new(-2, 2, 1))
        .look_at(Vec3::new(0, 0, -1))
        .v_up(Vec3::new(0, 1, 0))
        .build();

    camera.render(&world);
}
