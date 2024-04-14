use crate::vec3::Vec3;

pub struct Color(Vec3);

impl Color {
    pub fn from_rgb_float(r: f64, g: f64, b: f64) -> Self {
        debug_assert!(
            (0.0..=1.0).contains(&r) && (0.0..=1.0).contains(&g) && (0.0..=1.0).contains(&b)
        );
        Color(Vec3::new(r, g, b))
    }

    pub fn red(&self) -> u8 {
        (self.0.x * 255.999) as u8
    }

    pub fn green(&self) -> u8 {
        (self.0.y * 255.999) as u8
    }

    pub fn blue(&self) -> u8 {
        (self.0.z * 255.999) as u8
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.red(), self.green(), self.blue())
    }
}
