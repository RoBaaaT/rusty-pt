use crate::math::*;

pub type TextureId = usize;

pub trait Texture: Send + Sync {
    fn value(&self, u: Float, v: Float, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> ConstantTexture {
        ConstantTexture { color: color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        self.color
    }
}