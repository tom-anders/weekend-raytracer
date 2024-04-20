use core::f64;

#[derive(Debug, Clone, derive_more::From)]
pub struct Range {
    r: std::ops::RangeInclusive<f64>,
}

#[allow(dead_code)] // These functions are all implemented in book 1 but never used. Maybe we use
                    // them in book 2, so don't warn about them being unused for now.
impl Range {
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

    pub fn contains(&self, x: f64) -> bool {
        self.r.contains(&x)
    }

    pub fn surrounds(&self, x: f64) -> bool {
        *self.r.start() < x && x < *self.r.end()
    }
}
