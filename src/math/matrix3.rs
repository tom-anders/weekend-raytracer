use std::ops::Mul;

use super::{Axis, Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    data: [[f64; 3]; 3],
}

impl Matrix3 {
    pub fn zero() -> Self {
        Self {
            data: [[0.0; 3]; 3],
        }
    }

    pub fn rotate(angle_degrees: f64, axis: Axis) -> Self {
        match axis {
            Axis::X => Self::rotate_about_x(angle_degrees),
            Axis::Y => Self::rotate_about_y(angle_degrees),
            Axis::Z => Self::rotate_about_z(angle_degrees),
        }
    }

    pub fn rotate_about_x(angle_degrees: f64) -> Self {
        let angle = angle_degrees.to_radians();
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, angle.cos(), -angle.sin()],
                [0.0, angle.sin(), angle.cos()],
            ],
        }
    }

    pub fn rotate_about_y(angle_degrees: f64) -> Self {
        let angle = angle_degrees.to_radians();
        Self {
            data: [
                [angle.cos(), 0.0, angle.sin()],
                [0.0, 1.0, 0.0],
                [-angle.sin(), 0.0, angle.cos()],
            ],
        }
    }

    pub fn rotate_about_z(angle_degrees: f64) -> Self {
        let angle = angle_degrees.to_radians();
        Self {
            data: [
                [angle.cos(), -angle.sin(), 0.0],
                [angle.sin(), angle.cos(), 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Mul<Vec3> for Matrix3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let x = self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z;
        let y = self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z;
        let z = self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z;
        Vec3::new(x, y, z)
    }
}

impl Mul<Point3> for Matrix3 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Point3 {
        Point3::from_vec3(self * *rhs.as_vec3())
    }
}
