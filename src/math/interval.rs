use core::f64;
use std::ops::Add;

#[derive(Debug, Clone, derive_more::From)]
pub struct Interval {
    r: std::ops::RangeInclusive<f64>,
}

#[allow(dead_code)] // These functions are all implemented in book 1 but never used. Maybe we use
                    // them in book 2, so don't warn about them being unused for now.
impl Interval {
    pub fn merge(a: &Self, b: &Self) -> Self {
        Self::from(f64::min(a.min(), b.min())..=f64::max(a.max(), b.max()))
    }

    pub fn empty() -> Self {
        (f64::INFINITY..=f64::NEG_INFINITY).into()
    }

    pub fn universe() -> Self {
        (f64::NEG_INFINITY..=f64::INFINITY).into()
    }

    pub fn is_empty(&self) -> bool {
        self.r.is_empty()
    }

    pub fn size(&self) -> f64 {
        self.r.end() - self.r.start()
    }

    pub fn min(&self) -> f64 {
        *self.r.start()
    }

    pub fn max(&self) -> f64 {
        *self.r.end()
    }

    pub fn include(&mut self, x: f64) {
        self.r = (self.r.start().min(x))..=(self.r.end().max(x))
    }

    pub fn contains(&self, x: f64) -> bool {
        self.r.contains(&x)
    }

    pub fn surrounds(&self, x: f64) -> bool {
        *self.r.start() < x && x < *self.r.end()
    }

    #[must_use]
    pub fn expand(self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval {
            r: self.r.start() - padding..=self.r.end() + padding,
        }
    }

    pub fn expand_if_smaller_than(self, delta: f64) -> Interval {
        if self.size() < delta {
            self.expand(delta)
        } else {
            self
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::empty()
    }
}

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, displacement: f64) -> Self::Output {
        (self.min() + displacement..=self.max() + displacement).into()
    }
}
