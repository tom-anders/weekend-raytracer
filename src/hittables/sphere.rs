use crate::{
    hittables::{Hit, HitRecord},
    material::Material,
    math::{vec3::dot, Point3},
    math::{ray::Ray, Range},
};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: impl Into<Material>) -> Self {
        Self {
            center,
            radius,
            material: material.into(),
        }
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
        Some(HitRecord::new(root, p, r, outward_normal, &self.material))
    }
}
