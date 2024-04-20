use enum_dispatch::enum_dispatch;

use crate::{color::Color, math::Point3};

#[derive(Debug, Clone)]
#[enum_dispatch(TextureValue)]
pub enum Texture {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
}

impl From<Color> for Texture {
    fn from(color: Color) -> Self {
        Self::SolidColor(color.into())
    }
}

#[enum_dispatch]
pub trait TextureValue {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

#[derive(Debug, Clone, derive_more::From)]
pub struct SolidColor {
    albedo: Color,
}

impl TextureValue for SolidColor {
    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        self.albedo
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<Texture>,
    odd: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, c1: impl Into<Texture>, c2: impl Into<Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Box::new(c1.into()),
            odd: Box::new(c2.into()),
        }
    }
}

impl TextureValue for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_integer = f64::floor(self.inv_scale * p.x()) as i32;
        let y_integer = f64::floor(self.inv_scale * p.y()) as i32;
        let z_integer = f64::floor(self.inv_scale * p.z()) as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;
        if is_even { &self.even } else { &self.odd }.value(u, v, p)
    }
}
