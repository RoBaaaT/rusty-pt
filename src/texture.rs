use std::sync::{Arc, RwLock};

use crate::math::*;

mod perlin;
use crate::texture::perlin::PerlinNoise;

pub type TextureId = usize;

pub trait Texture: Send + Sync {
    fn value(&self, u: Float, v: Float, p: &Vec3, textures: &[Box<dyn Texture>]) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3
}

pub struct CheckerTexture {
    even: TextureId,
    odd: TextureId,
    frequency: Float
}

pub struct NoiseTexture {
    frequency: Float,
    noise: Arc<RwLock<PerlinNoise>>
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> ConstantTexture {
        ConstantTexture { color: color }
    }
}

impl CheckerTexture {
    pub fn new(even: TextureId, odd: TextureId, frequency: Float) -> CheckerTexture {
        CheckerTexture { even: even, odd: odd, frequency: frequency }
    }
}

impl NoiseTexture {
    pub fn new(frequency: Float) -> NoiseTexture {
        NoiseTexture { frequency: frequency, noise: Arc::new(RwLock::new(PerlinNoise::new())) }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: Float, _v: Float, _p: &Vec3, _textures: &[Box<dyn Texture>]) -> Vec3 {
        self.color
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: Float, v: Float, p: &Vec3, textures: &[Box<dyn Texture>]) -> Vec3 {
        let mut sines = 1.0;
        for dim in 0..3 {
            sines *= (p[dim] * self.frequency).sin();
        }
        if sines < 0.0 {
            textures[self.odd].value(u, v, p, textures)
        } else {
            textures[self.even].value(u, v, p, textures)
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: Float, _v: Float, p: &Vec3, _textures: &[Box<dyn Texture>]) -> Vec3 {
        let scaled = self.frequency * p;
        self.noise.read().unwrap().noise(&scaled) * Vec3::one()
    }
}