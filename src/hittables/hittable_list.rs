use crate::math::{self, Aabb, Interval};

use super::{Hit, HitRecord, Hittable};

#[derive(Debug, Clone, Default)]
pub struct HittableList {
    objects: Vec<Hittable>,
    bbox: Aabb,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Into<Hittable>) {
        let hittable = hittable.into();
        self.bbox = Aabb::merge(&self.bbox, hittable.bounding_box());
        self.objects.push(hittable);
    }
}

impl Hit for HittableList {
    fn hit(&self, r: &math::Ray, ray_bounds: &Interval) -> Option<HitRecord> {
        self.objects
            .iter()
            .flat_map(|o| o.hit(r, ray_bounds))
            .min_by(|lhs, rhs| lhs.t.total_cmp(&rhs.t))
    }

    fn bounding_box(&self) ->  &math::Aabb {
        &self.bbox
    }
}
