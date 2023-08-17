use std::ops::{Add, Mul, Sub};

#[derive(Default, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * _rhs.x,
            y: self * _rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

pub fn dot(a: &Vec2, b: &Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn normalize(a: &Vec2) -> Vec2 {
    (1.0 / a.magnitude()) * *a
}

pub fn rotate_vec2(vector: &Vec2, degrees: f32) -> Vec2 {
    let cos_degrees = degrees.cos();
    let sin_degrees = degrees.sin();
    Vec2 {
        x: vector.x * cos_degrees - vector.y * sin_degrees,
        y: vector.x * sin_degrees + vector.y * cos_degrees,
    }
}

#[derive(Default, Copy, Clone)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn from_vec2_floor(vec: &Vec2) -> Vec2i {
        Vec2i { x: vec.x as i32, y: vec.y as i32 }
    }
}
