use crate::{
    hittables::{Hit, HitRecord},
    material::Material,
    math::{ray::Ray, Interval, Vec3},
    math::{vec3::dot, Point3},
};

#[derive(Debug, Clone)]
pub struct Sphere {
    center1: Point3,
    radius: f64,
    material: Material,
    is_moving: bool,
    center_vec: Vec3,
}

impl Sphere {
    pub fn stationary(center: Point3, radius: f64, material: impl Into<Material>) -> Self {
        Self {
            center1: center,
            radius,
            material: material.into(),
            is_moving: false,
            center_vec: Vec3::zero(),
        }
    }

    pub fn moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: impl Into<Material>,
    ) -> Self {
        Self {
            center1,
            radius,
            material: material.into(),
            is_moving: true,
            center_vec: center2 - center1,
        }
    }

    // TODO right now it seems more readable to me to inline the `if self.is_moving` call here,
    // but let's first see if the book uses this function at some other point.
    fn sphere_center(&self, time: f64) -> Point3 {
        self.center1 + time * self.center_vec
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            self.sphere_center(r.time())
        } else {
            self.center1
        };
        let oc = center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_bounds.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_bounds.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);

        let outward_normal = (p - center) / self.radius;
        Some(HitRecord::new(root, p, r, outward_normal, &self.material))
    }
}
