use crate::{
    vector3::Vec3,
    vector4::{self, Vec4},
};

pub struct Matrix4 {
    data: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(scale_vector: Vec3) -> Self {
        Self {
            data: [
                [scale_vector.x, 0.0, 0.0, 0.0],
                [0.0, scale_vector.y, 0.0, 0.0],
                [0.0, 0.0, scale_vector.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn transform(&self, to_transform: Vec4) -> Vec4 {
        let row_zero = Vec4::from_array(&self.data[0]);
        let row_one = Vec4::from_array(&self.data[1]);
        let row_two = Vec4::from_array(&self.data[2]);
        let row_three = Vec4::from_array(&self.data[3]);

        let x = vector4::dot(&row_zero, &to_transform);
        let y = vector4::dot(&row_one, &to_transform);
        let z = vector4::dot(&row_two, &to_transform);
        let w = vector4::dot(&row_three, &to_transform);

        Vec4 { x, y, z, w }
    }
}
