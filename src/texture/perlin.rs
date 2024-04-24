use itertools::iproduct;
use rand::{seq::SliceRandom, thread_rng};

use crate::math::{dot, Point3, Vec3};

const NUM_POINTS: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    randvec: [Vec3; NUM_POINTS],
    perm_x: [usize; NUM_POINTS],
    perm_y: [usize; NUM_POINTS],
    perm_z: [usize; NUM_POINTS],
}

fn perlin_generate_perm() -> [usize; NUM_POINTS] {
    let mut p = std::array::from_fn(|i| i);
    p.shuffle(&mut thread_rng());
    p
}

fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let hermitian = |x| x * x * (3.0 - 2.0 * x);

    let u = hermitian(u);
    let v = hermitian(v);
    let w = hermitian(w);

    let vals = |factor: f64| {
        [0.0, 1.0]
            .into_iter()
            .map(move |i| i * factor + (1.0 - i) * (1.0 - factor))
            .enumerate()
    };

    iproduct!(vals(u), vals(v), vals(w))
        .map(|((i, ival), (j, jval), (k, kval))| {
            let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
            ival * jval * kval * dot(&c[i][j][k], &weight_v)
        })
        .sum()
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            randvec: std::array::from_fn(|_| Vec3::random(-1.0..1.0).normalized()),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        use std::array::from_fn;
        let c: [[[Vec3; 2]; 2]; 2] = from_fn(|di| {
            from_fn(|dj| {
                from_fn(|dk| {
                    self.randvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                })
            })
        });

        trilinear_interp(&c, u, v, w)
    }
}
