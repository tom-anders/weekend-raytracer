use std::f64::consts::PI;

use crate::{
    hittables::{Hit, HitRecord},
    material::Material,
    math::{dot, Aabb, Interval, Point3, Ray, Vec3},
};

use super::TextureCoords;

#[derive(Debug, Clone)]
pub struct Sphere {
    center1: Point3,
    radius: f64,
    material: Material,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl Sphere {
    pub fn stationary(center: Point3, radius: f64, material: impl Into<Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center1: center,
            radius,
            material: material.into(),
            is_moving: false,
            center_vec: Vec3::zero(),
            bbox: Aabb::from_points(center - rvec, center + rvec),
        }
    }

    pub fn moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: impl Into<Material>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox1 = Aabb::from_points(center1 - rvec, center1 + rvec);
        let bbox2 = Aabb::from_points(center2 - rvec, center2 + rvec);
        Self {
            center1,
            radius,
            material: material.into(),
            is_moving: true,
            center_vec: center2 - center1,
            bbox: Aabb::merge([bbox1, bbox2]),
        }
    }

    fn center_at_time(&self, time: f64) -> Point3 {
        if self.is_moving {
            self.center1 + time * self.center_vec
        } else {
            self.center1
        }
    }

    fn texture_coords(&self, p: &Point3) -> TextureCoords {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        TextureCoords {
            u: phi / (2.0 * PI),
            v: theta / PI,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord> {
        let center = self.center_at_time(r.time());
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
        Some(HitRecord::new(
            root,
            p,
            r,
            outward_normal,
            &self.material,
            self.texture_coords(&outward_normal.into()),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
