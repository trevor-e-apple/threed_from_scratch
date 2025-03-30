use std::ops;

// TODO: operator overloading for other standard vecotr operations (add, scale, sub)

#[derive(Clone, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Default)]
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

impl Vector2 {
    pub fn magnitude(v: &Vector2) -> f32 {
        (v.x * v.x + v.y * v.y).sqrt()
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

#[derive(Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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

impl Vector3 {
    pub fn magnitude(v: &Vector3) -> f32 {
        (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
    }
}
