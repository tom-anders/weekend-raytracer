use std::ops::RangeInclusive;

use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_bounds: &RangeInclusive<f64>) -> Option<HitRecord>;
}
