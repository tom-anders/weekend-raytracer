use std::ops::RangeInclusive;

use crate::math::{self, Range};

use super::{HitRecord, Hittable};

#[derive(Debug, Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &math::Ray, ray_bounds: &Range) -> Option<HitRecord> {
        self.objects
            .iter()
            .flat_map(|o| o.hit(r, ray_bounds))
            .min_by(|lhs, rhs| lhs.t.total_cmp(&rhs.t))
    }
}
