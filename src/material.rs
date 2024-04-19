use crate::{
    color::Color,
    hittables::HitRecord,
    math::{dot, reflect, Ray, Vec3},
};

#[derive(Debug, Clone, derive_more::From)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let mut scatter_direction = hit_record.normal() + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal();
        }
        ScatteredRay::new(hit_record, self.albedo, scatter_direction).into()
    }
}

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(ray_in.direction(), &hit_record.normal()).normalized()
            + self.fuzz * Vec3::random_unit_vector();
        let scattered = ScatteredRay::new(hit_record, self.albedo, reflected);
        (dot(scattered.ray.direction(), &hit_record.normal()) > 0.0).then_some(scattered)
    }
}

pub struct ScatteredRay {
    pub attenuation: Color,
    pub ray: Ray,
}

impl ScatteredRay {
    pub fn new(hit_record: &HitRecord, attenuation: Color, direction: Vec3) -> Self {
        Self {
            attenuation,
            ray: Ray::new(hit_record.p(), direction),
        }
    }
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        match self {
            Material::Lambertian(l) => l.scatter(ray_in, hit_record),
            Material::Metal(m) => m.scatter(ray_in, hit_record),
        }
    }
}
