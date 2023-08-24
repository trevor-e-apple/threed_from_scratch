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

    pub fn translate(vector: Vec3) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, vector.x],
                [0.0, 1.0, 0.0, vector.y],
                [0.0, 0.0, 1.0, vector.z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn multiply(a: Self, b: Self) -> Self {
        let a0 = Vec4::from_array(&a.data[0]);
        let a1 = Vec4::from_array(&a.data[1]);
        let a2 = Vec4::from_array(&a.data[2]);
        let a3 = Vec4::from_array(&a.data[3]);

        let b0 = Vec4 {
            x: b.data[0][0],
            y: b.data[1][0],
            z: b.data[2][0],
            w: b.data[3][0],
        };
        let b1 = Vec4 {
            x: b.data[0][1],
            y: b.data[1][1],
            z: b.data[2][1],
            w: b.data[3][1],
        };
        let b2 = Vec4 {
            x: b.data[0][2],
            y: b.data[1][2],
            z: b.data[2][2],
            w: b.data[3][2],
        };
        let b3 = Vec4 {
            x: b.data[0][3],
            y: b.data[1][3],
            z: b.data[2][3],
            w: b.data[3][3],
        };

        Self {
            data: [
                [
                    vector4::dot(&a0, &b0),
                    vector4::dot(&a0, &b1),
                    vector4::dot(&a0, &b2),
                    vector4::dot(&a0, &b3),
                ],
                [
                    vector4::dot(&a1, &b0),
                    vector4::dot(&a1, &b1),
                    vector4::dot(&a1, &b2),
                    vector4::dot(&a1, &b3),
                ],
                [
                    vector4::dot(&a2, &b0),
                    vector4::dot(&a2, &b1),
                    vector4::dot(&a2, &b2),
                    vector4::dot(&a2, &b3),
                ],
                [
                    vector4::dot(&a3, &b0),
                    vector4::dot(&a3, &b1),
                    vector4::dot(&a3, &b2),
                    vector4::dot(&a3, &b3),
                ],
            ],
        }
    }

    pub fn x_rotation(degrees: f32) -> Self {
        let cos = degrees.cos();
        let sin = degrees.sin();

        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, -sin, 0.0],
                [0.0, sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn y_rotation(degrees: f32) -> Self {
        let cos = degrees.cos();
        let sin = degrees.sin();

        Self {
            data: [
                [cos, 0.0, sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn z_rotation(degrees: f32) -> Self {
        let cos = degrees.cos();
        let sin = degrees.sin();

        Self {
            data: [
                [cos, -sin, 0.0, 0.0],
                [sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
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
