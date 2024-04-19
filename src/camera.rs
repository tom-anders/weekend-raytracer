use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    color::Color,
    hittables::Hit,
    math::{Ray, Vec3},
};

#[derive(Debug, Clone, derive_builder::Builder)]
#[builder(build_fn(private, name = "build_private"))]
#[builder(setter(skip))]
pub struct Camera {
    #[builder(setter, default = "1.0")]
    aspect_ratio: f64,
    #[builder(setter, default = "100")]
    image_width: usize,
    #[builder(setter, default = "10")]
    samples_per_pixel: usize,
    #[builder(setter, default = "10")]
    max_depth: i32,
    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl CameraBuilder {
    pub fn build(&self) -> Camera {
        let mut camera = self.build_private().unwrap();

        camera.image_height = ((camera.image_width as f64 / camera.aspect_ratio) as usize).max(1);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (camera.image_width as f64 / camera.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let viewport_upper_left =
            camera.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        camera.pixel_delta_u = viewport_u / camera.image_width as f64;
        camera.pixel_delta_v = viewport_v / camera.image_height as f64;
        camera.pixel00_loc =
            viewport_upper_left + 0.5 * (camera.pixel_delta_u + camera.pixel_delta_v);

        camera.pixel_samples_scale = 1.0 / camera.samples_per_pixel as f64;

        camera
    }
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn render(&self, world: &impl Hit) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        let mut colors = Vec::with_capacity(self.image_width);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    Color::from_rgb_float(
                        self.pixel_samples_scale
                            * std::iter::repeat_with(|| self.get_ray(i, j))
                                .take(self.samples_per_pixel)
                                .map(|ray| Self::ray_color(&ray, self.max_depth, world))
                                .sum::<Vec3>(),
                    )
                })
                .collect_into_vec(&mut colors);

            for color in &colors {
                println!("{color}");
            }
        }
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5), 0.0)
    }

    fn ray_color(r: &Ray, depth: i32, world: &impl Hit) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }
        if let Some(hit_record) = world.hit(r, &(0.001..=f64::INFINITY).into()) {
            let direction = hit_record.normal() + Vec3::random_unit_vector();
            return 0.5 * Self::ray_color(&Ray::new(hit_record.p(), direction), depth - 1, world);
        }
        let unit_direction = r.direction().normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}
