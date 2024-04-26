use crate::{
    material::Material,
    math::{cross, dot, Aabb, Interval, Point3, Ray, Vec3},
    texture::TextureCoords,
};

use super::{Hit, HitRecord, HittableList};

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
            bbox: Aabb::merge([bbox_diagonal1, bbox_diagonal2]),
            normal,
            D,
        }
    }

    fn is_interior(a: f64, b: f64) -> bool {
        (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b)
    }

    pub fn make_box(a: Point3, b: Point3, material: impl Into<Material>) -> HittableList {
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0, 0);
        let dy = Vec3::new(0, max.y() - min.y(), 0);
        let dz = Vec3::new(0, 0, max.z() - min.z());

        let material = material.into();
        [
            Quad::new(Point3::new(min.x(), min.y(), max.z()),  dx,  dy, material.clone()), // front
            Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz,  dy, material.clone()), // right
            Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx,  dy, material.clone()), // back
            Quad::new(Point3::new(min.x(), min.y(), min.z()),  dz,  dy, material.clone()), // left
            Quad::new(Point3::new(min.x(), max.y(), max.z()),  dx, -dz, material.clone()), // top
            Quad::new(Point3::new(min.x(), min.y(), min.z()),  dx,  dz, material.clone()), // bottom
        ].into_iter().collect()
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
