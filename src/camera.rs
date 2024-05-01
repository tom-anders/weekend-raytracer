use indicatif::{ProgressBar, ProgressDrawTarget, ProgressIterator};
use itertools::iproduct;
use rand::{thread_rng, Rng};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    color::Color,
    hittables::Hit,
    material::ScatterAndEmit,
    math::{cross, Point3, Ray, Vec3},
};

#[derive(Debug, derive_builder::Builder)]
#[builder(pattern = "owned", build_fn(private, name = "build_private"))]
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

    #[builder(setter, default = "Color::black()")]
    background: Color,

    #[builder(setter, default = "ProgressDrawTarget::stderr()")]
    progress_draw_target: ProgressDrawTarget,
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
    pixel_samples_scale: f64,
    sqrt_spp: usize,     // Square root of number of samples per pixel
    recip_sqrt_spp: f64, // 1 / sqrt_spp
    max_depth: i32,
    center: Point3,

    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    defocus_disk: Option<DefocusDisk>,

    background: Color,

    progress_bar: ProgressBar,
}

impl CameraParamsBuilder {
    pub fn build(self) -> Camera {
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

        let sqrt_spp = f64::sqrt(params.samples_per_pixel as f64) as usize;
        let pixel_samples_scale = 1.0 / (sqrt_spp * sqrt_spp) as f64;
        let recip_sqrt_spp = 1.0 / (sqrt_spp as f64);

        Camera {
            image_width: params.image_width,
            image_height,
            pixel_samples_scale,
            sqrt_spp,
            recip_sqrt_spp,
            max_depth: params.max_depth,
            center: params.look_from,

            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk,
            background: params.background,

            progress_bar: ProgressBar::with_draw_target(
                Some(image_height as u64),
                params.progress_draw_target,
            ),
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
    ) -> std::io::Result<()> {
        writeln!(output, "P3")?;
        writeln!(output, "{} {}", self.image_width, self.image_height)?;
        writeln!(output, "255")?;

        let mut colors = Vec::with_capacity(self.image_width);
        for j in (0..self.image_height).progress_with(self.progress_bar.clone()) {
            (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    self.pixel_samples_scale
                        * iproduct!((0..self.sqrt_spp), (0..self.sqrt_spp))
                            .map(|(s_i, s_j)| self.get_ray(i, j, s_i, s_j))
                            .map(|ray| self.ray_color(&ray, self.max_depth, world))
                            .sum::<Color>()
                })
                .collect_into_vec(&mut colors);

            for color in &colors {
                writeln!(output, "{color}")?;
            }
        }
        Ok(())
    }

    fn get_ray(&self, i: usize, j: usize, s_i: usize, s_j: usize) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j for stratified sample square s_i, s_j.

        let offset = self.sample_square_stratified(s_i, s_j);
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

    fn sample_square_stratified(&self, s_i: usize, s_j: usize) -> Vec3 {
        // Returns the vector to a random point in the square sub-pixel specified by grid
        // indices s_i and s_j, for an idealized unit square pixel [-.5,-.5] to [+.5,+.5].

        let px = ((s_i as f64 + thread_rng().gen::<f64>()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + thread_rng().gen::<f64>()) * self.recip_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0)
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &impl Hit) -> Color {
        if depth <= 0 {
            return Color::black();
        }
        if let Some(hit_record) = world.hit(r, &(0.001..=f64::INFINITY).into()) {
            let color_from_emission = hit_record.material.emit(&hit_record);
            if let Some(scattered) = hit_record.material.scatter(r, &hit_record) {
                let color_from_scatter =
                    scattered.attenuation * self.ray_color(&scattered.ray, depth - 1, world);
                color_from_emission + color_from_scatter
            } else {
                color_from_emission
            }
        } else {
            // If the ray hits nothing, return the background color.
            self.background
        }
    }
}
