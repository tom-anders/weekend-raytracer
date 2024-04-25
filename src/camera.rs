use rand::{thread_rng, Rng};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    color::Color, hittables::Hit, material::ScatterAndEmit, math::{cross, Point3, Ray, Vec3}
};

#[derive(Debug, Clone, derive_builder::Builder)]
#[builder(build_fn(private, name = "build_private"))]
pub struct CameraParams {
    #[builder(default = "1.0")]
    aspect_ratio: f64,
    #[builder(default = "100")]
    image_width: usize,
    #[builder(default = "10")]
    samples_per_pixel: usize,
    #[builder(default = "90.0")]
    vfov_degrees: f64,
    #[builder(default = "10")]
    max_depth: i32,
    #[builder(default = "Point3::new(0, 0, -1)")]
    look_at: Point3,
    #[builder(default = "Vec3::new(0, 1, 0)")]
    v_up: Vec3,
    #[builder(setter, default = "10.0")]
    focus_dist: f64,

    look_from: Point3,
    #[builder(setter, default = "None")]
    defocus_angle: Option<f64>,
}

#[derive(Debug, Clone)]
struct DefocusDisk {
    u: Vec3,
    v: Vec3,
}

impl DefocusDisk {
    fn sample(&self, center: Point3) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        center + (p.x * self.u) + (p.y * self.v)
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    pixel_samples_scale: f64,
    max_depth: i32,
    center: Point3,

    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    defocus_disk: Option<DefocusDisk>,
}

impl CameraParamsBuilder {
    pub fn build(&self) -> Camera {
        let params = self.build_private().unwrap();

        let image_height = ((params.image_width as f64 / params.aspect_ratio) as usize).max(1);

        let theta = params.vfov_degrees.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * params.focus_dist;
        let viewport_width = viewport_height * (params.image_width as f64 / image_height as f64);

        let w = (params.look_from - params.look_at).normalized();
        let u = cross(&params.v_up, &w).normalized();
        let v = cross(&w, &u);

        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * (-v); // Vector down viewport vertical edge

        let pixel_delta_u = viewport_u / params.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            params.look_from - params.focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_disk = params.defocus_angle.map(|defocus_angle| {
            let defocus_radius = params.focus_dist * f64::tan((defocus_angle / 2.0).to_radians());
            DefocusDisk {
                u: defocus_radius * u,
                v: defocus_radius * v,
            }
        });

        Camera {
            image_width: params.image_width,
            image_height,
            samples_per_pixel: params.samples_per_pixel,
            pixel_samples_scale: 1.0 / params.samples_per_pixel as f64,
            max_depth: params.max_depth,
            center: params.look_from,

            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk,
        }
    }
}

impl Camera {
    pub fn builder() -> CameraParamsBuilder {
        CameraParamsBuilder::default()
    }

    pub fn render(
        &self,
        world: &impl Hit,
        output: &mut impl std::io::Write,
        progress: &mut impl std::io::Write,
    ) -> std::io::Result<()> {
        writeln!(output, "P3")?;
        writeln!(output, "{} {}", self.image_width, self.image_height)?;
        writeln!(output, "255")?;

        let mut colors = Vec::with_capacity(self.image_width);
        for j in 0..self.image_height {
            write!(progress, "\rScanlines remaining: {} ", self.image_height - j)?;
            (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    self.pixel_samples_scale
                        * std::iter::repeat_with(|| self.get_ray(i, j))
                            .take(self.samples_per_pixel)
                            .map(|ray| Self::ray_color(&ray, self.max_depth, world))
                            .sum::<Color>()
                })
                .collect_into_vec(&mut colors);

            for color in &colors {
                writeln!(output, "{color}")?;
            }
        }
        Ok(())
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self
            .defocus_disk
            .as_ref()
            .map(|d| d.sample(self.center))
            .unwrap_or(self.center);
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = thread_rng().gen();
        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5), 0.0)
    }

    fn ray_color(r: &Ray, depth: i32, world: &impl Hit) -> Color {
        if depth <= 0 {
            return Color::black();
        }
        if let Some(hit_record) = world.hit(r, &(0.001..=f64::INFINITY).into()) {
            return if let Some(scattered) = hit_record.material.scatter(r, &hit_record) {
                scattered.attenuation * Self::ray_color(&scattered.ray, depth - 1, world)
            } else {
                Color::black()
            };
        }
        let unit_direction = r.direction().normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
