use crate::{
    color::Color,
    hittables::HitRecord,
    math::{reflect, Ray, Vec3},
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
}

impl Metal {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(ray_in.direction(), &hit_record.normal());
        ScatteredRay::new(hit_record, self.albedo, reflected).into()
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
