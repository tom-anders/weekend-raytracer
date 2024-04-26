use itertools::iproduct;

use crate::math::{Aabb, Axis, Interval, Matrix3, Point3, Ray};

use super::{Hit, HitRecord, Hittable};

#[derive(Debug, Clone)]
pub struct Rotate {
    object: Box<Hittable>,
    to_object_space: Matrix3,
    to_world_space: Matrix3,
    bbox: Aabb,
}

impl Rotate {
    pub fn new(object: impl Into<Hittable>, angle_degrees: f64, axis: Axis) -> Self {
        let object: Hittable = object.into();

        let to_object_space = Matrix3::rotate(-angle_degrees, axis);
        let to_world_space = Matrix3::rotate(angle_degrees, axis);

        let bbox = object.bounding_box().clone();
        let corners = iproduct!(
            [bbox.x.min(), bbox.x.max()],
            [bbox.y.min(), bbox.y.max()],
            [bbox.z.min(), bbox.z.max()]
        );
        let bbox = corners
            .map(|(x, y, z)| to_world_space * Point3::new(x, y, z))
            .collect();

        Self {
            object: Box::new(object),
            to_object_space,
            to_world_space,
            bbox,
        }
    }
}

impl Hit for Rotate {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord<'_>> {
        let rotated_ray = Ray::new(
            self.to_object_space * *r.origin(),
            self.to_object_space * *r.direction(),
            r.time(),
        );

        self.object
            .hit(&rotated_ray, ray_bounds)
            .map(|mut hit_record| {
                hit_record.p = self.to_world_space * hit_record.p;
                hit_record.normal = self.to_world_space * hit_record.normal;
                hit_record
            })
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
