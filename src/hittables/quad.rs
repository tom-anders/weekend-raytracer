use crate::{
    material::Material,
    math::{cross, dot, Aabb, Interval, Point3, Ray, Vec3},
    texture::TextureCoords,
};

use super::{Hit, HitRecord};

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Quad {
    Q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Material,
    bbox: Aabb,
    normal: Vec3,
    D: f64, // rhs of the quad's plane equation (Ax+By+Cy=D)
}

impl Quad {
    #[allow(non_snake_case)]
    pub fn new(Q: Point3, u: Vec3, v: Vec3, material: impl Into<Material>) -> Self {
        let bbox_diagonal1 = Aabb::from_points(Q, Q + u + v);
        let bbox_diagonal2 = Aabb::from_points(Q + u, Q + v);

        let n = cross(&u, &v);
        let normal = n.normalized();
        let D = dot(&normal, Q.as_vec3());

        Self {
            Q,
            u,
            v,
            w: n / dot(&n, &n),
            material: material.into(),
            bbox: Aabb::merge(&bbox_diagonal1, &bbox_diagonal2),
            normal,
            D,
        }
    }

    fn is_interior(a: f64, b: f64) -> bool {
        (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b)
    }
}

impl Hit for Quad {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord<'_>> {
        let denominator = dot(&self.normal, r.direction());

        // No hit if the ray is parallel to the plane
        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.D - dot(&self.normal, r.origin().as_vec3())) / denominator;
        if !ray_bounds.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let planar_hit_point_vector = intersection - self.Q;
        let alpha = dot(&self.w, &cross(&planar_hit_point_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hit_point_vector));

        Self::is_interior(alpha, beta).then_some(HitRecord::new(
            t,
            intersection,
            r,
            self.normal,
            &self.material,
            TextureCoords { u: alpha, v: beta },
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
