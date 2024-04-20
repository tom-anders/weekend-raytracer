use crate::math::vec3::Vec3;

use super::Point3;

#[derive(Debug, Clone, PartialEq, derive_more::Constructor)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
