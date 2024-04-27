use crate::{
    material::Material, math::{dot, Aabb, Axis, Interval, Point3, Ray, Vec3}, texture::TextureCoords
};

use enum_dispatch::enum_dispatch;

mod sphere;
pub use sphere::*;

mod quad;
pub use quad::*;

mod hittable_list;
pub use hittable_list::*;

mod bvh;
pub use bvh::*;

mod translate;
pub use translate::*;

mod rotate;
pub use rotate::*;

mod constant_medium;
pub use constant_medium::*;

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a Material,
    pub t: f64,
    pub front_face: bool,
    pub texture_coords: TextureCoords,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        p: Point3,
        ray: &Ray,
        outward_normal: Vec3,
        material: &'a Material,
        texture_coords: TextureCoords,
    ) -> Self {
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
            texture_coords,
        }
    }

    pub fn offset(mut self, offset: Vec3) -> Self {
        self.p += offset;
        self
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum Hittable {
    Sphere(Sphere),
    List(HittableList),
    BvhNode(BvhNode),
    Quad(Quad),
    Translate(Translate),
    Rotate(Rotate),
    ConstantMedium(ConstantMedium),
}

#[enum_dispatch(Hittable)]
pub trait Hit: Sync {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> &Aabb;
}

pub trait Instance {
    fn translate(self, offset: Vec3) -> Hittable;
    fn rotate_x(self, angle_degrees: f64) -> Hittable;
    fn rotate_y(self, angle_degrees: f64) -> Hittable;
    fn rotate_z(self, angle_degrees: f64) -> Hittable;
}

impl<H: Into<Hittable>> Instance for H {
    fn translate(self, offset: Vec3) -> Hittable {
        Translate::new(self.into(), offset).into()
    }

    fn rotate_x(self, angle_degrees: f64) -> Hittable {
        Rotate::new(self.into(), angle_degrees, Axis::X).into()
    }

    fn rotate_y(self, angle_degrees: f64) -> Hittable {
        Rotate::new(self.into(), angle_degrees, Axis::Y).into()
    }

    fn rotate_z(self, angle_degrees: f64) -> Hittable {
        Rotate::new(self.into(), angle_degrees, Axis::Z).into()
    }
}
