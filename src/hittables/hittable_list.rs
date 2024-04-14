use std::ops::RangeInclusive;

use crate::math;

use super::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }

    fn clear(&mut self) {
        self.objects.clear();
    }
}

impl FromIterator<Box<dyn Hittable>> for HittableList {
    fn from_iter<T: IntoIterator<Item = Box<dyn Hittable>>>(iter: T) -> Self {
        Self {
            objects: iter.into_iter().collect(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &math::Ray, ray_bounds: &RangeInclusive<f64>) -> Option<HitRecord> {
        self.objects
            .iter()
            .flat_map(|o| o.hit(r, ray_bounds))
            .min_by(|lhs, rhs| lhs.t.total_cmp(&rhs.t))
    }
}
