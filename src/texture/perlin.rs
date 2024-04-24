use itertools::iproduct;
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

fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    iproduct!(0..2, 0..2, 0..2)
        .map(|(i, j, k)| {
            let (i, j, k) = (i as f64, j as f64, k as f64);
            (i * u + (1.0 - i) * (1.0 - u))
                * (j * v + (1.0 - j) * (1.0 - v))
                * (k * w + (1.0 - k) * (1.0 - w))
                * c[i as usize][j as usize][k as usize]
        })
        .sum()
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
        let hermitian = |x| x * x * (3.0 - 2.0 * x);

        let u = hermitian(p.x() - p.x().floor());
        let v = hermitian(p.y() - p.y().floor());
        let w = hermitian(p.z() - p.z().floor());

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        use std::array::from_fn;
        let c: [[[f64; 2]; 2]; 2] = from_fn(|di| {
            from_fn(|dj| {
                from_fn(|dk| {
                    self.randfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                })
            })
        });

        trilinear_interp(&c, u, v, w)
    }
}
