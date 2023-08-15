use std::ops::{Add, Mul, Sub};

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// right-hand rule cross product, where a is the first vector
pub fn cross_product(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn normalize(a: &Vec3) -> Vec3 {
    (1.0 / a.magnitude()) * *a
}

/// find the normal vector to a and b of length 1
pub fn unit_normal(a: &Vec3, b: &Vec3) -> Vec3 {
    let vec = cross_product(a, b);
    normalize(&vec)
}

fn x_axis_rotate(vector: &Vec3, degrees: f32) -> Vec3 {
    let cos_degrees = degrees.cos();
    let sin_degrees = degrees.sin();
    Vec3 {
        x: vector.x,
        y: vector.y * cos_degrees - vector.z * sin_degrees,
        z: vector.y * sin_degrees + vector.z * cos_degrees,
    }
}

fn y_axis_rotate(vector: &Vec3, degrees: f32) -> Vec3 {
    let cos_degrees = degrees.cos();
    let sin_degrees = degrees.sin();
    Vec3 {
        x: vector.x * cos_degrees - vector.z * sin_degrees,
        y: vector.y,
        z: vector.x * sin_degrees + vector.z * cos_degrees,
    }
}

fn z_axis_rotate(vector: &Vec3, degrees: f32) -> Vec3 {
    let cos_degrees = degrees.cos();
    let sin_degrees = degrees.sin();
    Vec3 {
        x: vector.x * cos_degrees - vector.y * sin_degrees,
        y: vector.x * sin_degrees + vector.y * cos_degrees,
        z: vector.z,
    }
}

pub fn rotate_vec3(
    vector: &Vec3,
    x_degrees: f32,
    y_degrees: f32,
    z_degrees: f32,
) -> Vec3 {
    let result = vector.clone();
    let result = x_axis_rotate(&result, x_degrees);
    let result = y_axis_rotate(&result, y_degrees);
    let result = z_axis_rotate(&result, z_degrees);

    result
}
