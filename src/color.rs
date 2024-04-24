use std::ops::{Add, Mul};

use crate::math::Vec3;

#[derive(Debug, Clone, Copy, derive_more::From)]
pub struct Color(Vec3);

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.max(0.0).sqrt()
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn black() -> Self {
        Self(Vec3::zero())
    }

    pub fn white() -> Self {
        Self(Vec3::new(1, 1, 1))
    }

    fn red(&self) -> u8 {
        (linear_to_gamma(self.0.x).clamp(0.0, 0.999) * 256.0) as u8
    }

    fn green(&self) -> u8 {
        (linear_to_gamma(self.0.y).clamp(0.0, 0.999) * 256.0) as u8
    }

    fn blue(&self) -> u8 {
        (linear_to_gamma(self.0.z).clamp(0.0, 0.999) * 256.0) as u8
    }
}

impl From<palette::rgb::LinSrgb> for Color {
    fn from(value: palette::rgb::LinSrgb) -> Self {
        Self::new(value.red.into(), value.green.into(), value.blue.into())
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.red(), self.green(), self.blue())
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::black(), |acc, v| acc + v)
    }
}
