// for functions that mix the different vector types
use crate::{vector2::Vec2, vector2::Vec2i, vector3::Vec3, vector4::Vec4};

impl Vec2 {
    pub fn from_vec4(vec4: &Vec4) -> Self {
        Self {
            x: vec4.x,
            y: vec4.y,
        }
    }
}

impl Vec3 {
    pub fn from_vec4(vec4: &Vec4) -> Self {
        Self {
            x: vec4.x,
            y: vec4.y,
            z: vec4.z,
        }
    }
}

impl Vec4 {
    pub fn from_vec3(vec3: &Vec3) -> Self {
        Self {
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
            w: 1.0,
        }
    }
}

impl Vec2i {
    pub fn from_vec4_floor(vec4: &Vec4) -> Self {
        Self {
            x: vec4.x as i32,
            y: vec4.y as i32,
        }
    }
}
