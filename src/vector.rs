use std::ops;

#[derive(Clone, Default, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn from_vector4(v: &Vector4) -> Self {
        Self { x: v.x, y: v.y }
    }
}

#[derive(Clone, Default, Copy)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::Mul<&Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<f32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Vector2 {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let scalar = 1.0 / self.magnitude();
        self.x = self.x * scalar;
        self.y = self.y * scalar;
    }

    pub fn dot_product(a: &Self, b: &Self) -> f32 {
        a.x * b.x + a.y * b.y
    }
}

impl Vector2i {
    pub fn from_vector2(v: &Vector2) -> Vector2i {
        Vector2i {
            x: v.x.round() as i32,
            y: v.y.round() as i32,
        }
    }
}

#[derive(Clone, Default, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn from_vector2(v: &Vector2) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: 0.0,
        }
    }

    pub fn from_vector4(v: &Vector4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Vector3 {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let scalar = 1.0 / self.magnitude();
        self.x = scalar * self.x;
        self.y = scalar * self.y;
        self.z = scalar * self.z;
    }

    pub fn calc_normalized_vector(v: &Vector3) -> Vector3 {
        let scalar = 1.0 / v.magnitude();
        scalar * v
    }

    pub fn dot_product(a: &Self, b: &Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

pub fn rotate_around_x(v: &Vector3, angle: f32) -> Vector3 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    Vector3 {
        x: v.x,
        y: v.y * cos_angle - v.z * sin_angle,
        z: v.y * sin_angle + v.z * cos_angle,
    }
}

pub fn rotate_around_y(v: &Vector3, angle: f32) -> Vector3 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    Vector3 {
        x: v.x * cos_angle - v.z * sin_angle,
        y: v.y,
        z: v.x * sin_angle + v.z * cos_angle,
    }
}

pub fn rotate_around_z(v: &Vector3, angle: f32) -> Vector3 {
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();
    Vector3 {
        x: v.x * cos_angle - v.y * sin_angle,
        y: v.x * sin_angle + v.y * cos_angle,
        z: v.z,
    }
}

pub fn calc_cross_product(a: &Vector3, b: &Vector3) -> Vector3 {
    Vector3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

#[derive(Clone, Default, PartialEq, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn from_vector3(v: &Vector3) -> Self {
        Vector4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 1.0,
        }
    }
}

impl ops::Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Add<&Vector4> for &Vector4 {
    type Output = Vector4;

    fn add(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Sub<&Vector4> for &Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl ops::Mul<&Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl ops::Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Vector4 {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Mul<f32> for &Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Vector4 {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Vector2i {
    pub fn from_vector4(v: &Vector4) -> Vector2i {
        Vector2i {
            x: v.x.round() as i32,
            y: v.y.round() as i32,
        }
    }
}
