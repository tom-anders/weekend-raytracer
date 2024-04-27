use rand::{thread_rng, Rng};

use crate::{
    material::{Isotropic, Material},
    math::{Aabb, Interval, Ray, Vec3},
    texture::{Texture, TextureCoords},
};

use super::{Hit, HitRecord, Hittable};

#[derive(Debug, Clone)]
pub struct ConstantMedium {
    boundary: Box<Hittable>,
    neg_inv_density: f64,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: impl Into<Hittable>, density: f64, texture: impl Into<Texture>) -> Self {
        Self {
            boundary: Box::new(boundary.into()),
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new(texture.into()).into(),
        }
    }
}

impl Hit for ConstantMedium {
    fn hit(&self, r: &Ray, ray_bounds: &Interval) -> Option<HitRecord<'_>> {
        let mut entry_hit = self.boundary.hit(r, &Interval::universe())?;
        let mut exit_hit = self
            .boundary
            .hit(r, &Interval::from(entry_hit.t + 0.0001..=f64::INFINITY))?;

        entry_hit.t = entry_hit.t.max(ray_bounds.min());
        exit_hit.t = exit_hit.t.min(ray_bounds.max());

        if entry_hit.t >= exit_hit.t {
            return None;
        }

        entry_hit.t = entry_hit.t.max(0.0);

        let ray_length = r.direction().length();
        let distance_inside_boundary = (exit_hit.t - entry_hit.t) * ray_length;
        let hit_distance = self.neg_inv_density * thread_rng().gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None; // Ray passes through the medium
        }

        let t = entry_hit.t + hit_distance / ray_length;
        HitRecord::new(
            t,
            r.at(t),
            r,
            Vec3::new(1, 0, 0), // arbitrary
            &self.phase_function,
            TextureCoords::default(),
        )
        .into()
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}
