use crate::math::{Aabb, Interval, Ray, Vec3};

use super::{Hit, HitRecord, Hittable};

#[derive(Debug, Clone)]
pub struct Translate {
    object: Box<Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: impl Into<Hittable>, offset: Vec3) -> Self {
        let object = object.into();
        let bbox = object.bounding_box().clone() + offset;
        Self {
            object: Box::new(object),
            offset,
            bbox,
        }
    }
}

impl Hit for Translate {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord<'_>> {
        let offset_ray = r.offset(self.offset);

        self.object
            .hit(&offset_ray, ray_bounds)
            .map(|hit_record| hit_record.offset(self.offset))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
