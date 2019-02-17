use crate::math::*;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min: min, max: max }
    }

    pub fn hit(self, ray: &Ray, t_min: Float, t_max: Float) -> bool {
        for dim in 0..3 {
            let inv_d = 1.0 / ray.direction()[dim];
            let mut t0 = (self.min[dim] - ray.origin()[dim]) * inv_d;
            let mut t1 = (self.max[dim] - ray.origin()[dim]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_mi = if t0 > t_min { t0 } else { t_min };
            let t_ma = if t1 < t_max { t1 } else { t_max };
            if t_ma <= t_mi {
                return false;
            }
        }
        true
    }
}