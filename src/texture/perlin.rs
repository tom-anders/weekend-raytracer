use std::thread;

use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::math::Point3;

const NUM_POINTS: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    randfloat: [f64; NUM_POINTS],
    perm_x: [usize; NUM_POINTS],
    perm_y: [usize; NUM_POINTS],
    perm_z: [usize; NUM_POINTS],
}

fn perlin_generate_perm() -> [usize; NUM_POINTS] {
    let mut p = std::array::from_fn(|i| i);
    p.shuffle(&mut thread_rng());
    p
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            randfloat: std::array::from_fn(|_| thread_rng().gen()),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;

        self.randfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }
}
