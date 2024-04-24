use std::path::Path;

use enum_dispatch::enum_dispatch;
use image::{io::Reader as ImageReader, Rgb32FImage};
use palette::Srgb;

use crate::{color::Color, math::Point3};

#[derive(Debug, Clone)]
#[enum_dispatch(TextureValue)]
pub enum Texture {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
    Image(Image),
}

impl From<Color> for Texture {
    fn from(color: Color) -> Self {
        Self::SolidColor(color.into())
    }
}

#[derive(Debug, Clone)]
pub struct TextureCoords {
    pub u: f64,
    pub v: f64,
}

#[enum_dispatch]
pub trait TextureValue {
    fn value(&self, coords: &TextureCoords, p: Point3) -> Color;
}

#[derive(Debug, Clone, derive_more::From)]
pub struct SolidColor {
    albedo: Color,
}

impl TextureValue for SolidColor {
    fn value(&self, _: &TextureCoords, _: Point3) -> Color {
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
    fn value(&self, coords: &TextureCoords, p: Point3) -> Color {
        let x_integer = f64::floor(self.inv_scale * p.x()) as i32;
        let y_integer = f64::floor(self.inv_scale * p.y()) as i32;
        let z_integer = f64::floor(self.inv_scale * p.z()) as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;
        if is_even { &self.even } else { &self.odd }.value(coords, p)
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    img: Rgb32FImage,
}

impl Image {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            img: ImageReader::open(path)?.decode()?.to_rgb32f(),
        })
    }
}

impl TextureValue for Image {
    fn value(&self, coords: &TextureCoords, _: Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = coords.u.clamp(0.0, 1.0);
        let v = 1.0 - coords.v.clamp(0.0, 1.0); // Flip V to image coordinates

        let i = (u * self.img.width() as f64) as u32;
        let j = (v * self.img.height() as f64) as u32;

        let pixel = self.img[(i, j)];

        // The image crate loads images in sRGB color space, but our Color class expects linear.
        Color::from(Srgb::new(pixel[0], pixel[1], pixel[2]).into_linear())
    }
}
