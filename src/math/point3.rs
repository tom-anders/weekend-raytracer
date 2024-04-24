use std::ops::{Add, Index, Mul, Sub};

use super::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Default, derive_more::From)]
pub struct Point3(Vec3);

#[allow(dead_code)] // Might use some methods later in the book
impl Point3 {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self(Vec3::new(x, y, z))
    }

    pub fn origin() -> Self {
        Self(Vec3::zero())
    }

    pub fn x(&self) -> f64 {
        self.0.x
    }

    pub fn y(&self) -> f64 {
        self.0.y
    }

    pub fn z(&self) -> f64 {
        self.0.z
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point3(self.0 + rhs)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Point3(self.0 - rhs)
    }
}

impl Mul<Point3> for f64 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        Point3(self * rhs.0)
    }
}


impl Index<usize> for Point3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
