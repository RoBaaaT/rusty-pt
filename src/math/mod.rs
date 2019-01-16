use std::ops;
use std::fmt;
use std::f32;

use rand::prelude::*;

pub type Float = f32;
pub const MAX_FLOAT: Float = f32::MAX;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [Float; 3]
}

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

#[allow(dead_code)]
impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn one() -> Vec3 {
        Vec3 { e: [1.0, 1.0, 1.0] }
    }

    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn normalize(value: Vec3) -> Vec3 {
        let length = value.length();
        Vec3 { e: [value.e[0] / length, value.e[1] / length, value.e[2] / length] }
    }

    pub fn dot(a: Vec3, b: Vec3) -> Float {
        a.e[0] * b.e[0] + a.e[1] * b.e[1] + a.e[2] * b.e[2]
    }

    pub fn length(self) -> Float {
        let squared_length = self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
        squared_length.sqrt()
    }

    pub fn length_squared(self) -> Float {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn x(&self) -> Float {
        self.e[0]
    }
    pub fn y(&self) -> Float {
        self.e[1]
    }
    pub fn z(&self) -> Float {
        self.e[2]
    }

    pub fn r(&self) -> Float {
        self.e[0]
    }
    pub fn g(&self) -> Float {
        self.e[1]
    }
    pub fn b(&self) -> Float {
        self.e[2]
    }
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

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2])
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2])
    }
}

impl ops::Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(other.e[0] * self, other.e[1] * self, other.e[2] * self)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(other.e[0] * self.e[0], other.e[1] * self.e[1], other.e[2] * self.e[2])
    }
}


impl ops::Div<Float> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Float) -> Vec3 {
        Vec3::new(self.e[0] / other, self.e[1] / other, self.e[2] / other)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3::new(self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]);
    }
}

impl ops::DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, other: Float) {
        *self = Vec3::new(self.e[0] / other, self.e[1] / other, self.e[2] / other);
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{};{};{}]", self.e[0], self.e[1], self.e[2])
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rand::thread_rng().gen(), rand::thread_rng().gen(), rand::thread_rng().gen()) - Vec3::one();
        if p.length_squared() < 1.0 {
            return p
        }
    }
}