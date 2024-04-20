use crate::{
    material::Material, math::{dot, Aabb, Interval, Point3, Ray, Vec3},
};

use enum_dispatch::enum_dispatch;
mod sphere;
pub use sphere::*;

mod hittable_list;
pub use hittable_list::*;

mod bvh;
pub use bvh::*;

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
    BvhNode(BvhNode),
}

#[enum_dispatch(Hittable)]
pub trait Hit : Sync {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> &Aabb;
}
