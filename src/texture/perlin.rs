use crate::math::*;
use rand::prelude::*;

pub struct PerlinNoise {
    random: [Vec3; 256],
    x_permutation: [u8; 256],
    y_permutation: [u8; 256],
    z_permutation: [u8; 256],
}

fn permute(input: &mut [u8; 256]) {
    for i in (1..256).rev() {
        let target = thread_rng().gen_range(0, i);
        let tmp = input[target];
        input[target] = input[i];
        input[i] = tmp;
    }
}

fn generate_permutation() -> [u8; 256] {
    let mut result = [0; 256];
    for i in 0..256 {
        result[i] = i as u8;
    }
    permute(&mut result);
    result
}

fn perlin_interpolation(c: &[Vec3; 8], u: Float, v: Float, w: Float) -> Float {
    let mut accu = 0.0;
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let fi = i as Float;
                let fj = j as Float;
                let fk = k as Float;
                let weight = Vec3::new(u - fi, v - fj, w - fk);
                let vec = c[i * 4 + j * 2 + k];
                accu += (fi * uu + (1.0 - fi) * (1.0 - uu)) *
                    (fj * vv + (1.0 - fj) * (1.0 - vv)) *
                    (fk * ww + (1.0 - fk) * (1.0 - ww)) * Vec3::dot(vec, weight);
            }
        }
    }
    let denom = (3.0_f32).sqrt();
    accu = accu / denom + 0.5;
    if accu > 1.0 { 1.0 } else if accu < 0.0 { 0.0 } else { accu }
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut random = [Vec3::zero(); 256];
        let x_perm = generate_permutation();
        let y_perm = generate_permutation();
        let z_perm = generate_permutation();
        for i in 0..256 {
            let mut v = Vec3::zero();
            while v.length_squared() < 0.0001 {
                v = 2.0 * Vec3::new(rand::thread_rng().gen(), rand::thread_rng().gen(), rand::thread_rng().gen()) - Vec3::one()
            }

            random[i] = Vec3::normalize(v);
        }
        PerlinNoise { random: random, x_permutation: x_perm, y_permutation: y_perm, z_permutation: z_perm }
    }

    pub fn noise(&self, p: &Vec3) -> Float {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [Vec3::zero(); 8];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di * 4 + dj * 2 + dk] = self.random[
                        (self.x_permutation[(i as usize + di) & 255] ^
                        self.y_permutation[(j as usize + dj) & 255] ^
                        self.z_permutation[(k as usize + dk) & 255]) as usize]
                }
            }
        }
        perlin_interpolation(&c, u, v, w)
    }
}