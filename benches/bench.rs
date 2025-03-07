use criterion::{criterion_group, criterion_main, Criterion};
use indicatif::ProgressDrawTarget;
use rand::{thread_rng, Rng};
use weekend_raytracer::{
    camera::Camera,
    color::Color,
    hittables::{BvhNode, HittableList, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    math::{Point3, Vec3},
};

fn sphere(c: &mut Criterion) {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Sphere::stationary(
        Point3::new(0, -1000, 0),
        1000.0,
        ground_material,
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
                let center2 = center + Vec3::new(0, thread_rng().gen_range(0.0..0.5), 0);
                world.push(Sphere::moving(
                    center,
                    center2,
                    0.2,
                    if chose_mat < 0.8 {
                        let albedo = Color::from(Vec3::random(0.0..1.0) * Vec3::random(0.0..1.0));
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

    let world = BvhNode::new(world.into_iter().collect());

    let camera = Camera::builder()
        .aspect_ratio(16.0 / 9.0)
        .image_width(50)
        .samples_per_pixel(10)
        .max_depth(50)
        .vfov_degrees(20.0)
        .look_from(Point3::new(13, 2, 3))
        .look_at(Point3::new(0, 0, 0))
        .v_up(Vec3::new(0, 1, 0))
        .defocus_angle(Some(0.6))
        .focus_dist(10.0)
        .progress_draw_target(ProgressDrawTarget::hidden())
        .build();

    c.bench_function("Render cover image of book 1", |b| {
        b.iter(|| camera.render(&world, &mut std::io::sink()).unwrap())
    });
}

criterion_group!(benches, sphere);
criterion_main!(benches);
