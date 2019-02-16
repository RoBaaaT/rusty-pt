mod vec3;

use std::f32;
use rand::prelude::*;

pub use self::vec3::Vec3;
pub use self::vec3::random_in_unit_disk;
pub use self::vec3::random_in_unit_sphere;

pub type Float = f32;
pub const MAX_FLOAT: Float = f32::MAX;
pub const MIN_FLOAT: Float = f32::MIN;
pub const EPSILON: Float = f32::EPSILON;
pub const PI: Float = f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

#[allow(dead_code)]
impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { origin: o, direction: d }
    }

    pub fn eval(self, t: Float) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn origin(self) -> Vec3 {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }
}

pub fn random() -> Float {
    rand::thread_rng().gen()
}