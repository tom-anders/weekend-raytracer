use enum_dispatch::enum_dispatch;
use rand::{thread_rng, Rng};

use crate::{
    color::Color, hittables::HitRecord, math::{dot, reflect, refract, Ray, Vec3}, texture::{Texture, TextureValue}
};

#[derive(Debug, Clone)]
#[enum_dispatch(Scatter)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

#[enum_dispatch]
pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay>;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    texture: Box<Texture>,
}

impl Lambertian {
    pub fn new(texture: impl Into<Texture>) -> Self {
        Self {
            texture: Box::new(texture.into()),
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let attenuation = self.texture.value(&hit_record.texture_coords, hit_record.p);
        ScatteredRay::new(hit_record, attenuation, scatter_direction, ray_in.time()).into()
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(ray_in.direction(), &hit_record.normal).normalized()
            + self.fuzz * Vec3::random_unit_vector();
        let scattered = ScatteredRay::new(hit_record, self.albedo, reflected, ray_in.time());
        (dot(scattered.ray.direction(), &hit_record.normal) > 0.0).then_some(scattered)
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().normalized();

        let cos_theta = dot(&-unit_direction, &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, ri) > thread_rng().gen_range(0.0..=1.0)
        {
            reflect(&unit_direction, &hit_record.normal)
        } else {
            refract(&unit_direction, &hit_record.normal, ri)
        };
        ScatteredRay::new(hit_record, Color::white(), direction, ray_in.time()).into()
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

pub struct ScatteredRay {
    pub attenuation: Color,
    pub ray: Ray,
}

impl ScatteredRay {
    pub fn new(hit_record: &HitRecord, attenuation: Color, direction: Vec3, time: f64) -> Self {
        Self {
            attenuation,
            ray: Ray::new(hit_record.p, direction, time),
        }
    }
}
