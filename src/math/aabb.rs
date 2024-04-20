use super::{Interval, Point3, Ray};

/// Axis-aligned bounding box
#[derive(Debug, Default, Clone, derive_more::Constructor)]
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
    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: if a.x() < b.x() {
                a.x()..=b.x()
            } else {
                b.x()..=a.x()
            }
            .into(),
            y: if a.y() < b.y() {
                a.y()..=b.y()
            } else {
                b.y()..=a.y()
            }
            .into(),
            z: if a.z() < b.z() {
                a.z()..=b.z()
            } else {
                b.z()..=a.z()
            }
            .into(),
        }
    }

    pub fn merge(box0: &Self, box1: &Self) -> Self {
        Self {
            x: Interval::merge(&box0.x, &box1.x),
            y: Interval::merge(&box0.y, &box1.y),
            z: Interval::merge(&box0.z, &box1.z),
        }
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
