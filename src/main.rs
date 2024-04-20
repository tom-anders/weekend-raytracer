use color::Color;
use hittables::Sphere;
use material::{Dielectric, Lambertian, Material, Metal};
use math::Vec3;
use rand::{thread_rng, Rng};

use crate::{camera::Camera, hittables::HittableList};

mod camera;
mod color;
mod hittables;
mod material;
mod math;

fn main() {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(Color::from_rgb_float(Vec3::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(Vec3::new(0, -1000, 0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let chose_mat = thread_rng().gen_range(0.0..1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * thread_rng().gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * thread_rng().gen_range(0.0..1.0),
            );

            if (center - Vec3::new(4, 0.2, 0)).length() > 0.9 {
                world.push(Sphere::new(
                    center,
                    0.2,
                    if chose_mat < 0.8 {
                        let albedo = Color(Vec3::random(0.0..1.0) * Vec3::random(0.0..1.0));
                        Material::from(Lambertian::new(albedo))
                    } else if chose_mat < 0.95 {
                        let albedo = Color::from_rgb_float(Vec3::random(0.5..1.0));
                        let fuzz = thread_rng().gen_range(0.0..0.5);
                        Material::from(Metal::new(albedo, fuzz))
                    } else {
                        Material::from(Dielectric::new(1.5))
                    },
                ))
            }
        }
    }

    world.push(Sphere::new(Vec3::new(0, 1, 0), 1.0, Dielectric::new(1.5)));
    world.push(Sphere::new(
        Vec3::new(-4, 1, 0),
        1.0,
        Lambertian::new(Color::from_rgb_float(Vec3::new(0.4, 0.2, 0.1))),
    ));
    world.push(Sphere::new(
        Vec3::new(4, 1, 0),
        1.0,
        Metal::new(Color::from_rgb_float(Vec3::new(0.7, 0.6, 0.5)), 0.0),
    ));

    let camera = Camera::builder()
        .aspect_ratio(16.0 / 9.0)
        .image_width(1200)
        .samples_per_pixel(10)
        .max_depth(50)
        .vfov_degrees(20.0)
        .look_from(Vec3::new(13, 2, 3))
        .look_at(Vec3::new(0, 0, 0))
        .v_up(Vec3::new(0, 1, 0))
        .defocus_angle(Some(0.6))
        .focus_dist(10.0)
        .build();

    camera.render(&world);
}
