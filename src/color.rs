use crate::math::vec3::Vec3;

pub struct Color(Vec3);

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.max(0.0).sqrt()
}

impl Color {
    pub fn from_rgb_float(v: Vec3) -> Self {
        Self(v)
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

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.red(), self.green(), self.blue())
    }
}
