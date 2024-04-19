use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    color::Color,
    hittables::Hit,
    math::{cross, Ray, Vec3},
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
    #[builder(setter, default = "90.0")]
    vfov_degrees: f64,
    #[builder(setter, default = "10")]
    max_depth: i32,
    #[builder(setter, default = "Vec3::new(0, 0, -1)")]
    look_at: Vec3,
    #[builder(setter, default = "Vec3::new(0, 1, 0)")]
    v_up: Vec3,
    #[builder(setter(name = "look_from"), default = "Vec3::zero()")]
    center: Vec3,
    #[builder(setter, default = "None")]
    defocus_angle: Option<f64>,
    #[builder(setter, default = "10.0")]
    focus_dist: f64,

    image_height: usize,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl CameraBuilder {
    pub fn build(&self) -> Camera {
        let mut camera = self.build_private().unwrap();

        camera.image_height = ((camera.image_width as f64 / camera.aspect_ratio) as usize).max(1);

        let theta = camera.vfov_degrees.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * camera.focus_dist;
        let viewport_width =
            viewport_height * (camera.image_width as f64 / camera.image_height as f64);

        let w = (camera.center - camera.look_at).normalized();
        let u = cross(&camera.v_up, &w).normalized();
        let v = cross(&w, &u);

        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * (-v); // Vector down viewport vertical edge

        camera.pixel_delta_u = viewport_u / camera.image_width as f64;
        camera.pixel_delta_v = viewport_v / camera.image_height as f64;

        let viewport_upper_left =
            camera.center - camera.focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;

        camera.pixel00_loc =
            viewport_upper_left + 0.5 * (camera.pixel_delta_u + camera.pixel_delta_v);

        if let Some(defocus_angle) = camera.defocus_angle {
            let defocus_radius = camera.focus_dist * f64::tan((defocus_angle / 2.0).to_radians());
            camera.defocus_disk_u = defocus_radius * u;
            camera.defocus_disk_v = defocus_radius * v;
        }

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
        // Construct a camera ray originating from the defocus disk and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle.is_some() {
            self.defocus_disk_sample()
        } else {
            self.center
        };
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
            return if let Some(scattered) = hit_record.material().scatter(r, &hit_record) {
                scattered.attenuation.0 * Self::ray_color(&scattered.ray, depth - 1, world)
            } else {
                Vec3::zero()
            };
        }
        let unit_direction = r.direction().normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
