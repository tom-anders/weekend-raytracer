use crate::{
    math::ray::Ray,
    math::{vec3::{dot, Vec3}, Range},
};

pub mod sphere;
pub use sphere::Sphere;

pub mod hittable_list;
pub use hittable_list::HittableList;

#[derive(Debug, Clone)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = dot(ray.direction(), &outward_normal) < 0.0;
        Self {
            p,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t,
            front_face,
        }
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}

pub trait Hittable : std::fmt::Debug + Sync {
    fn hit(&self, r: &Ray, ray_bounds: &Range) -> Option<HitRecord>;
}
