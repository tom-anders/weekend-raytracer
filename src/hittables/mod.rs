use crate::{
    material::Material, math::ray::Ray, math::{vec3::{dot, Vec3}, Range}
};

pub mod sphere;
pub use sphere::Sphere;

pub mod hittable_list;
pub use hittable_list::HittableList;

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    p: Vec3,
    normal: Vec3,
    material: &'a Material,
    t: f64,
    front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, p: Vec3, ray: &Ray, outward_normal: Vec3, material: &'a Material) -> Self {
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

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn material(&self) -> &Material {
        self.material
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

#[derive(Debug, Clone, derive_more::From)]
pub enum Hittable {
    Sphere(Sphere),
    List(HittableList),
}

pub trait Hit : Sync {
    fn hit(&self, r: &Ray, ray_bounds: &Range) -> Option<HitRecord<'_>>;
}

impl Hittable {
    fn hit(&self, r: &Ray, ray_bounds: &Range) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(s) => s.hit(r, ray_bounds),
            Hittable::List(l) => l.hit(r, ray_bounds),
        }
    }
}
