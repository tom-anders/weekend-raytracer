use super::{Interval, Point3, Ray};

/// Axis-aligned bounding box
#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn from_points(a: &Point3, b: &Point3) -> Self {
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

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        let ray_orig = ray.origin();
        let ray_dir = ray.direction();

        let (mut t_min, mut t_max) = (ray_t.min(), ray_t.max());

        for axis in [0, 1, 2] {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min() - ray_orig[axis]) * adinv;
            let t1 = (ax.max() - ray_orig[axis]) * adinv;

            t_min = t_min.max(t0.min(t1));
            t_max = t_max.min(t1.max(t0));

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}
