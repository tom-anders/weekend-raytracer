use crate::{
    material::Material, math::{Ray, Point3, dot, Vec3, Interval},
};

pub mod sphere;
use enum_dispatch::enum_dispatch;
pub use sphere::Sphere;

pub mod hittable_list;
pub use hittable_list::HittableList;

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, p: Point3, ray: &Ray, outward_normal: Vec3, material: &'a Material) -> Self {
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
            material,
        }
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum Hittable {
    Sphere(Sphere),
    List(HittableList),
}

#[enum_dispatch(Hittable)]
pub trait Hit : Sync {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord<'_>>;
}
