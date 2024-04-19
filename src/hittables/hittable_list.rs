use crate::math::{self, Range};

use super::{Hit, HitRecord, Hittable};

#[derive(Debug, Clone, Default)]
pub struct HittableList {
    objects: Vec<Hittable>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Into<Hittable>) {
        self.objects.push(hittable.into());
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hit for HittableList {
    fn hit(&self, r: &math::Ray, ray_bounds: &Range) -> Option<HitRecord> {
        self.objects
            .iter()
            .flat_map(|o| o.hit(r, ray_bounds))
            .min_by(|lhs, rhs| lhs.t.total_cmp(&rhs.t))
    }
}
