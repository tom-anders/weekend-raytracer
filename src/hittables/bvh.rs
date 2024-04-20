use std::cmp::Ordering;

use crate::math::{Aabb, Axis, Interval, Ray};

use super::{Hit, HitRecord, Hittable};

#[derive(Debug, Clone)]
pub struct BvhNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(mut objects: Vec<Hittable>) -> Self {
        let bbox = objects.iter()
            .fold(Aabb::empty(), |acc, o| Aabb::merge(&acc, o.bounding_box()));

        let axis = bbox.longest_axis();

        let (left, right) = match objects.len() {
            1 => (
                Box::new(objects[0].clone()),
                Box::new(objects.pop().unwrap()),
            ),
            2 => (
                Box::new(objects.pop().unwrap()),
                Box::new(objects.pop().unwrap()),
            ),
            _ => {
                objects.sort_unstable_by(|lhs, rhs| {
                    if Self::box_compare(lhs, rhs, axis) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });

                let mid = objects.len() / 2;
                let right = objects.split_off(mid);
                let left = objects;

                (
                    Box::new(BvhNode::new(left).into()),
                    Box::new(BvhNode::new(right).into()),
                )
            }
        };

        Self { left, right, bbox }
    }

    fn box_compare(a: &Hittable, b: &Hittable, axis: Axis) -> bool {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);
        a_axis_interval.min() < b_axis_interval.min()
    }
}

impl Hit for BvhNode {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord<'_>> {
        if !self.bbox.hit(r, ray_bounds) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_bounds);
        let right_bound = hit_left.as_ref().map(|h| h.t).unwrap_or(ray_bounds.max());
        let hit_right = self
            .right
            .hit(r, &Interval::from(ray_bounds.min()..=right_bound));

        // Note: In the book this reads `hit_left || hit_right` (as it's using bool+out-parameter
        // instead of Option). For our Option-based approach the order actually matters here. We
        // first try to return hit_right, since in case we have a hit in both nodes, the right hit
        // will have a lower `t` (as we restricted the bounds accordingly). In the book, they get
        // the same effect since their `right->hit(...)` call will overwrite the hit record that may
        // have already been filled by `left->hit(...)`
        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
