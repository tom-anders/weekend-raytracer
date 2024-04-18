use rand::Rng;

use crate::{
    color::Color,
    hittables::Hittable,
    math::{Ray, Vec3},
};

#[derive(Debug, Clone)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: usize,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: usize) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Vec3::zero();

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u: viewport_u / image_width as f64,
            pixel_delta_v: viewport_v / image_height as f64,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let color = Color::from_rgb_float(
                    self.pixel_samples_scale
                        * std::iter::repeat_with(|| self.get_ray(i, j))
                            .take(self.samples_per_pixel)
                            .map(|ray| Self::ray_color(&ray, world))
                            .sum::<Vec3>()
                );

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

    fn ray_color(r: &Ray, world: &impl Hittable) -> Vec3 {
        if let Some(hit_record) = world.hit(r, &(0.0..=f64::INFINITY).into()) {
            return 0.5 * (hit_record.normal() + Vec3::new(1, 1, 1));
        }
        let unit_direction = r.direction().normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}
