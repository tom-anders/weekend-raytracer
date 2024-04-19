use crate::{
    hittables::{HitRecord, Hit},
    math::{ray::Ray, Range},
    math::vec3::{dot, Vec3},
};

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn center(&self) -> Vec3 {
        self.center
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_bounds: &Range) -> Option<HitRecord> {
        let oc = self.center - *r.origin();
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

        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(root, p, r, outward_normal))
    }
}
