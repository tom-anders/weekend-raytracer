#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

mod vec3;
pub use vec3::*;
mod matrix3;
pub use matrix3::*;
mod point3;
pub use point3::*;
mod ray;
pub use ray::*;
mod interval;
pub use interval::Interval;
mod aabb;
pub use aabb::*;
