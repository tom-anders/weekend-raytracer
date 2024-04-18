use crate::math::vec3::Vec3;

pub struct Color(Vec3);

impl Color {
    pub fn from_rgb_float(v: Vec3) -> Self {
        Self(v)
    }

    pub fn red(&self) -> u8 {
        (self.0.x.clamp(0.0, 0.999) * 256.0) as u8
    }

    pub fn green(&self) -> u8 {
        (self.0.y.clamp(0.0, 0.999) * 256.0) as u8
    }

    pub fn blue(&self) -> u8 {
        (self.0.z.clamp(0.0, 0.999) * 256.0) as u8
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.red(), self.green(), self.blue())
    }
}
