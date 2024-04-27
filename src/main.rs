use std::path::Path;

use anyhow::Result;
use clap::Parser;
use itertools::{iproduct, Itertools};
use rand::{thread_rng, Rng};
use weekend_raytracer::{
    color::Color,
    hittables::{BvhNode, ConstantMedium, Hittable, Instance, Quad, Sphere},
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
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
    SimpleLight,
    CornellBox,
    CornellSmoke,
    FinalScene,
}

impl Scene {
    fn create(&self) -> Result<(Camera, Hittable)> {
        let mut world = HittableList::default();
        let camera = match self {
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

                Camera::builder()
                    .background(Color::new(0.70, 0.80, 1.00))
                    .aspect_ratio(16.0 / 9.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(20.0)
                    .look_from(Point3::new(13, 2, 3))
                    .look_at(Point3::new(0, 0, 0))
                    .v_up(Vec3::new(0, 1, 0))
                    .defocus_angle(Some(0.6))
                    .focus_dist(10.0)
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

                Camera::builder()
                    .background(Color::new(0.70, 0.80, 1.00))
                    .aspect_ratio(16.0 / 9.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(20.0)
                    .look_from(Point3::new(13, 2, 3))
                    .look_at(Point3::new(0, 0, 0))
                    .defocus_angle(None)
            }
            Self::Earth => {
                let earth_texture = Lambertian::new(Image::new(Path::new("res/earthmap.jpg"))?);
                world.push(Sphere::stationary(Point3::new(0, 0, 0), 2.0, earth_texture));

                Camera::builder()
                    .background(Color::new(0.70, 0.80, 1.00))
                    .aspect_ratio(16.0 / 9.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(20.0)
                    .look_from(Point3::new(0, 0, 12))
                    .look_at(Point3::new(0, 0, 0))
                    .v_up(Vec3::new(0, 1, 0))
                    .defocus_angle(Some(0.6))
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

                Camera::builder()
                    .background(Color::new(0.70, 0.80, 1.00))
                    .aspect_ratio(16.0 / 9.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(20.0)
                    .look_from(Point3::new(13, 2, 3))
                    .look_at(Point3::new(0, 0, 0))
                    .v_up(Vec3::new(0, 1, 0))
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

                Camera::builder()
                    .background(Color::new(0.70, 0.80, 1.00))
                    .aspect_ratio(1.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(80.0)
                    .look_from(Point3::new(0, 0, 9))
                    .look_at(Point3::new(0, 0, 0))
                    .v_up(Vec3::new(0, 1, 0))
            }
            Self::SimpleLight => {
                let pertext = Lambertian::new(Noise::new(4.0));
                world.push(Sphere::stationary(
                    Point3::new(0, -1000, 0),
                    1000.0,
                    pertext.clone(),
                ));
                world.push(Sphere::stationary(
                    Point3::new(0, 2, 0),
                    2.0,
                    pertext.clone(),
                ));

                let difflight = DiffuseLight::new(Color::new(4.0, 4.0, 4.0));
                world.push(Quad::new(
                    Point3::new(3, 1, -2),
                    Vec3::new(2, 0, 0),
                    Vec3::new(0, 2, 0),
                    difflight.clone(),
                ));
                world.push(Sphere::stationary(Point3::new(0, 7, 0), 2.0, difflight));

                Camera::builder()
                    .aspect_ratio(16.0 / 9.0)
                    .image_width(400)
                    .samples_per_pixel(100)
                    .max_depth(50)
                    .vfov_degrees(20.0)
                    .look_from(Point3::new(26, 3, 6))
                    .look_at(Point3::new(0, 2, 0))
                    .v_up(Vec3::new(0, 1, 0))
            }
            Self::CornellBox => {
                let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
                let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
                let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
                let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

                world.push(Quad::new(
                    Point3::new(555, 0, 0),
                    Vec3::new(0, 555, 0),
                    Vec3::new(0, 0, 555),
                    green,
                ));
                world.push(Quad::new(
                    Point3::new(0, 0, 0),
                    Vec3::new(0, 555, 0),
                    Vec3::new(0, 0, 555),
                    red,
                ));
                world.push(Quad::new(
                    Point3::new(343, 554, 332),
                    Vec3::new(-130, 0, 0),
                    Vec3::new(0, 0, -105),
                    light,
                ));
                world.push(Quad::new(
                    Point3::new(0, 0, 0),
                    Vec3::new(555, 0, 0),
                    Vec3::new(0, 0, 555),
                    white.clone(),
                ));
                world.push(Quad::new(
                    Point3::new(555, 555, 555),
                    Vec3::new(-555, 0, 0),
                    Vec3::new(0, 0, -555),
                    white.clone(),
                ));
                world.push(Quad::new(
                    Point3::new(0, 0, 555),
                    Vec3::new(555, 0, 0),
                    Vec3::new(0, 555, 0),
                    white.clone(),
                ));

                world.push(
                    Hittable::from(Quad::make_box(
                        Point3::origin(),
                        Point3::new(165, 330, 165),
                        white.clone(),
                    ))
                    .rotate_y(15.0)
                    .translate(Vec3::new(265, 0, 295)),
                );

                world.push(
                    Hittable::from(Quad::make_box(
                        Point3::origin(),
                        Point3::new(165, 165, 165),
                        white.clone(),
                    ))
                    .rotate_y(-18.0)
                    .translate(Vec3::new(130, 0, 65)),
                );

                Camera::builder()
                    .aspect_ratio(1.0)
                    .image_width(600)
                    .samples_per_pixel(200)
                    .max_depth(50)
                    .vfov_degrees(40.0)
                    .look_from(Point3::new(278, 278, -800))
                    .look_at(Point3::new(278, 278, 0))
                    .v_up(Vec3::new(0, 1, 0))
            }
            Self::CornellSmoke => {
                let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
                let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
                let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
                let light = DiffuseLight::new(Color::new(7.0, 7.0, 7.0));

                world.push(Quad::new(
                    Point3::new(555, 0, 0),
                    Vec3::new(0, 555, 0),
                    Vec3::new(0, 0, 555),
                    green,
                ));
                world.push(Quad::new(
                    Point3::new(0, 0, 0),
                    Vec3::new(0, 555, 0),
                    Vec3::new(0, 0, 555),
                    red,
                ));
                world.push(Quad::new(
                    Point3::new(113, 554, 127),
                    Vec3::new(330, 0, 0),
                    Vec3::new(0, 0, 305),
                    light,
                ));
                world.push(Quad::new(
                    Point3::new(0, 0, 0),
                    Vec3::new(555, 0, 0),
                    Vec3::new(0, 0, 555),
                    white.clone(),
                ));
                world.push(Quad::new(
                    Point3::new(555, 555, 555),
                    Vec3::new(-555, 0, 0),
                    Vec3::new(0, 0, -555),
                    white.clone(),
                ));
                world.push(Quad::new(
                    Point3::new(0, 0, 555),
                    Vec3::new(555, 0, 0),
                    Vec3::new(0, 555, 0),
                    white.clone(),
                ));

                let box1 = Hittable::from(Quad::make_box(
                    Point3::origin(),
                    Point3::new(165, 330, 165),
                    white.clone(),
                ))
                .rotate_y(15.0)
                .translate(Vec3::new(265, 0, 295));
                world.push(ConstantMedium::new(box1, 0.01, Color::black()));

                let box2 = Hittable::from(Quad::make_box(
                    Point3::origin(),
                    Point3::new(165, 165, 165),
                    white.clone(),
                ))
                .rotate_y(-18.0)
                .translate(Vec3::new(130, 0, 65));
                world.push(ConstantMedium::new(box2, 0.01, Color::white()));

                Camera::builder()
                    .aspect_ratio(1.0)
                    .image_width(600)
                    .samples_per_pixel(200)
                    .max_depth(50)
                    .vfov_degrees(40.0)
                    .look_from(Point3::new(278, 278, -800))
                    .look_at(Point3::new(278, 278, 0))
                    .v_up(Vec3::new(0, 1, 0))
            }
            Self::FinalScene => {
                let ground = Lambertian::new(Color::new(0.48, 0.83, 0.53));

                let boxes_per_side = 20;
                let boxes1 = iproduct!(0..boxes_per_side, 0..boxes_per_side)
                    .map(|(i, j)| {
                        let w = 100.0;
                        let x0 = -1000.0 + i as f64 * w;
                        let z0 = -1000.0 + j as f64 * w;
                        let y0 = 0.0;

                        let x1 = x0 + w;
                        let y1 = thread_rng().gen_range(1.0..101.0);
                        let z1 = z0 + w;
                        Hittable::from(Quad::make_box(
                            Point3::new(x0, y0, z0),
                            Point3::new(x1, y1, z1),
                            ground.clone(),
                        ))
                    })
                    .collect();
                world.push(BvhNode::new(boxes1));

                let light = DiffuseLight::new(Color::new(7.0, 7.0, 7.0));
                world.push(Quad::new(
                    Point3::new(123, 554, 147),
                    Vec3::new(300, 0, 0),
                    Vec3::new(0, 0, 265),
                    light,
                ));

                let center1 = Point3::new(400, 400, 200);
                let center2 = center1 + Vec3::new(30, 0, 0);
                world.push(Sphere::moving(
                    center1,
                    center2,
                    50.0,
                    Lambertian::new(Color::new(0.7, 0.3, 0.1)),
                ));

                world.push(Sphere::stationary(
                    Point3::new(260, 150, 45),
                    50.0,
                    Dielectric::new(1.5),
                ));
                world.push(Sphere::stationary(
                    Point3::new(0, 150, 145),
                    50.0,
                    Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
                ));

                let boundary =
                    Sphere::stationary(Point3::new(360, 150, 145), 70.0, Dielectric::new(1.5));
                world.push(boundary.clone());
                world.push(ConstantMedium::new(
                    boundary,
                    0.2,
                    Color::new(0.2, 0.4, 0.9),
                ));

                let boundary = Sphere::stationary(Point3::origin(), 5000.0, Dielectric::new(1.5));
                world.push(ConstantMedium::new(
                    boundary,
                    0.0001,
                    Color::new(1.0, 1.0, 1.0),
                ));

                world.push(Sphere::stationary(
                    Point3::new(400, 200, 400),
                    100.0,
                    Lambertian::new(Image::new(Path::new("res/earthmap.jpg"))?),
                ));
                world.push(Sphere::stationary(
                    Point3::new(220, 280, 300),
                    80.0,
                    Lambertian::new(Noise::new(0.2)),
                ));

                let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
                let boxes2 = (0..1000)
                    .map(|_| {
                        Hittable::from(Sphere::stationary(
                            Point3::from_vec3(Vec3::random(0.0..165.0)),
                            10.0,
                            white.clone(),
                        ))
                    })
                    .collect_vec();
                world.push(
                    BvhNode::new(boxes2)
                        .rotate_y(15.0)
                        .translate(Vec3::new(-100, 270, 395)),
                );

                Camera::builder()
                    .aspect_ratio(1.0)
                    .image_width(400)
                    .samples_per_pixel(250)
                    .max_depth(4)
                    .vfov_degrees(40.0)
                    .look_from(Point3::new(478, 278, -600))
                    .look_at(Point3::new(278, 278, 0))
                    .v_up(Vec3::new(0, 1, 0))
            }
        };
        Ok((
            camera.build(),
            BvhNode::new(world.into_iter().collect()).into(),
        ))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (camera, world) = args.scene.create()?;

    camera.render(&world, &mut std::io::stdout())?;

    Ok(())
}
