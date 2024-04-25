use std::path::Path;

use anyhow::Result;
use clap::Parser;
use rand::{thread_rng, Rng};
use weekend_raytracer::{
    color::Color,
    hittables::{BvhNode, Hittable, Quad, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    math::{Point3, Vec3},
    texture::{CheckerTexture, Image, Noise},
    {camera::Camera, hittables::HittableList},
};

#[derive(clap::Parser)]
#[command(version, about)]
struct Args {
    scene: Scene,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Scene {
    BouncingSpheres,
    CheckeredSpheres,
    Earth,
    PerlinSpheres,
    Quads,
}

impl Scene {
    fn create(&self) -> Result<(Camera, Hittable)> {
        let mut camera = Camera::builder();
        let mut world = HittableList::default();
        match self {
            Self::BouncingSpheres => {
                let checker =
                    CheckerTexture::new(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
                world.push(Sphere::stationary(
                    Point3::new(0, -1000, 0),
                    1000.0,
                    Lambertian::new(checker),
                ));

                for a in -11..11 {
                    for b in -11..11 {
                        let chose_mat = thread_rng().gen_range(0.0..1.0);
                        let center = Point3::new(
                            a as f64 + 0.9 * thread_rng().gen_range(0.0..1.0),
                            0.2,
                            b as f64 + 0.9 * thread_rng().gen_range(0.0..1.0),
                        );

                        if (center - Point3::new(4, 0.2, 0)).length() > 0.9 {
                            let center2 =
                                center + Vec3::new(0, thread_rng().gen_range(0.0..0.5), 0);
                            world.push(Sphere::moving(
                                center,
                                center2,
                                0.2,
                                if chose_mat < 0.8 {
                                    let albedo = Color::from(
                                        Vec3::random(0.0..1.0) * Vec3::random(0.0..1.0),
                                    );
                                    Material::from(Lambertian::new(albedo))
                                } else if chose_mat < 0.95 {
                                    let albedo = Color::from(Vec3::random(0.5..1.0));
                                    let fuzz = thread_rng().gen_range(0.0..0.5);
                                    Material::from(Metal::new(albedo, fuzz))
                                } else {
                                    Material::from(Dielectric::new(1.5))
                                },
                            ))
                        }
                    }
                }

                world.push(Sphere::stationary(
                    Point3::new(0, 1, 0),
                    1.0,
                    Dielectric::new(1.5),
                ));
                world.push(Sphere::stationary(
                    Point3::new(-4, 1, 0),
                    1.0,
                    Lambertian::new(Color::new(0.4, 0.2, 0.1)),
                ));
                world.push(Sphere::stationary(
                    Point3::new(4, 1, 0),
                    1.0,
                    Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
                ));

                camera
                    .aspect_ratio(16.0 / 9.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(20.0)
                    .look_from(Point3::new(13, 2, 3))
                    .look_at(Point3::new(0, 0, 0))
                    .v_up(Vec3::new(0, 1, 0))
                    .defocus_angle(Some(0.6))
                    .focus_dist(10.0);
            }
            Self::CheckeredSpheres => {
                let checker =
                    CheckerTexture::new(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

                world.push(Sphere::stationary(
                    Point3::new(0, -10, 0),
                    10.0,
                    Lambertian::new(checker.clone()),
                ));
                world.push(Sphere::stationary(
                    Point3::new(0, 10, 0),
                    10.0,
                    Lambertian::new(checker.clone()),
                ));

                camera
                    .aspect_ratio(16.0 / 9.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(20.0)
                    .look_from(Point3::new(13, 2, 3))
                    .look_at(Point3::new(0, 0, 0))
                    .defocus_angle(None);
            }
            Self::Earth => {
                let earth_texture = Lambertian::new(Image::new(Path::new("res/earthmap.jpg"))?);
                world.push(Sphere::stationary(Point3::new(0, 0, 0), 2.0, earth_texture));

                camera
                    .aspect_ratio(16.0 / 9.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(20.0)
                    .look_from(Point3::new(0, 0, 12))
                    .look_at(Point3::new(0, 0, 0))
                    .v_up(Vec3::new(0, 1, 0))
                    .defocus_angle(Some(0.6));
            }
            Self::PerlinSpheres => {
                let perlin_text = Noise::new(4.0);
                world.push(Sphere::stationary(
                    Point3::new(0, -1000, 0),
                    1000.0,
                    Lambertian::new(perlin_text.clone()),
                ));
                world.push(Sphere::stationary(
                    Point3::new(0, 2, 0),
                    2.0,
                    Lambertian::new(perlin_text.clone()),
                ));

                camera
                    .aspect_ratio(16.0 / 9.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(20.0)
                    .look_from(Point3::new(13, 2, 3))
                    .look_at(Point3::new(0, 0, 0))
                    .v_up(Vec3::new(0, 1, 0));
            }
            Self::Quads => {
                let left_red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
                let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
                let right_blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
                let upper_orange = Lambertian::new(Color::new(1.0, 0.5, 0.0));
                let lower_teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));

                world.push(Quad::new(
                    Point3::new(-3, -2, 5),
                    Vec3::new(0, 0, -4),
                    Vec3::new(0, 4, 0),
                    left_red,
                ));
                world.push(Quad::new(
                    Point3::new(-2, -2, 0),
                    Vec3::new(4, 0, 0),
                    Vec3::new(0, 4, 0),
                    back_green,
                ));
                world.push(Quad::new(
                    Point3::new(3, -2, 1),
                    Vec3::new(0, 0, 4),
                    Vec3::new(0, 4, 0),
                    right_blue,
                ));
                world.push(Quad::new(
                    Point3::new(-2, 3, 1),
                    Vec3::new(4, 0, 0),
                    Vec3::new(0, 0, 4),
                    upper_orange,
                ));
                world.push(Quad::new(
                    Point3::new(-2, -3, 5),
                    Vec3::new(4, 0, 0),
                    Vec3::new(0, 0, -4),
                    lower_teal,
                ));

                camera
                    .aspect_ratio(1.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(80.0)
                    .look_from(Point3::new(0, 0, 9))
                    .look_at(Point3::new(0, 0, 0))
                    .v_up(Vec3::new(0, 1, 0));
            }
        }
        Ok((
            camera.build(),
            BvhNode::new(world.into_iter().collect()).into(),
        ))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (camera, world) = args.scene.create()?;

    camera.render(&world, &mut std::io::stdout(), &mut std::io::stderr())?;

    Ok(())
}
