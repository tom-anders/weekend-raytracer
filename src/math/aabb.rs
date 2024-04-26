use std::borrow::Borrow;

use super::{Interval, Point3, Ray};

/// Axis-aligned bounding box
#[derive(Debug, Default, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Aabb {
    fn new(x: impl Into<Interval>, y: impl Into<Interval>, z: impl Into<Interval>) -> Self {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.
        let delta = 0.0001;
        Self {
            x: x.into().expand_if_smaller_than(delta),
            y: y.into().expand_if_smaller_than(delta),
            z: z.into().expand_if_smaller_than(delta),
        }
    }

    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self::new(
            if a.x() < b.x() {
                a.x()..=b.x()
            } else {
                b.x()..=a.x()
            },
            if a.y() < b.y() {
                a.y()..=b.y()
            } else {
                b.y()..=a.y()
            },
            if a.z() < b.z() {
                a.z()..=b.z()
            } else {
                b.z()..=a.z()
            },
        )
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let ray_orig = ray.origin();
        let ray_dir = ray.direction();

        let (mut t_min, mut t_max) = (ray_t.min(), ray_t.max());

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis as usize];

            let t0 = (ax.min() - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max() - ray_orig[axis as usize]) * adinv;

            t_min = t_min.max(t0.min(t1));
            t_max = t_max.min(t1.max(t0));

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn axis_interval(&self, a: Axis) -> &Interval {
        match a {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }

    pub fn longest_axis(&self) -> Axis {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                Axis::X
            } else {
                Axis::Z
            }
        } else if self.y.size() > self.z.size() {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}

impl<BorrowedBbox: Borrow<Aabb>> FromIterator<BorrowedBbox> for Aabb {
    fn from_iter<T: IntoIterator<Item = BorrowedBbox>>(iter: T) -> Self {
        iter.into_iter().fold(Self::empty(), |acc, bbox| {
            Self::new(
            Interval::merge(&acc.x, &bbox.borrow().x),
            Interval::merge(&acc.y, &bbox.borrow().y),
            Interval::merge(&acc.z, &bbox.borrow().z),
            )
        })
    }
}
