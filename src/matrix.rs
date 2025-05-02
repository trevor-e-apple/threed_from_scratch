use crate::vector::Vector4;

pub struct Matrix4 {
    pub data: [[f32; 4]; 4],
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

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn dot(a: &Matrix4, b: &Matrix4, i: usize, j: usize) -> f32 {
        a.data[i][0] * b.data[0][j]
            + a.data[i][1] * b.data[1][j]
            + a.data[i][2] * b.data[2][j]
            + a.data[i][3] * b.data[3][j]
    }

    pub fn mult_mat4(a: &Self, b: &Self) -> Self {
        Self {
            data: [
                [
                    Self::dot(a, b, 0, 0),
                    Self::dot(a, b, 0, 1),
                    Self::dot(a, b, 0, 2),
                    Self::dot(a, b, 0, 3),
                ],
                [
                    Self::dot(a, b, 1, 0),
                    Self::dot(a, b, 1, 1),
                    Self::dot(a, b, 1, 2),
                    Self::dot(a, b, 1, 3),
                ],
                [
                    Self::dot(a, b, 2, 0),
                    Self::dot(a, b, 2, 1),
                    Self::dot(a, b, 2, 2),
                    Self::dot(a, b, 2, 3),
                ],
                [
                    Self::dot(a, b, 3, 0),
                    Self::dot(a, b, 3, 1),
                    Self::dot(a, b, 3, 2),
                    Self::dot(a, b, 3, 3),
                ],
            ],
        }
    }

    pub fn mult_vector(a: &Self, b: Vector4) -> Vector4 {
        Vector4 {
            x: b.x * a.data[0][0] + b.y * a.data[0][1] + b.z * a.data[0][2] + b.w * a.data[0][3],
            y: b.x * a.data[1][0] + b.y * a.data[1][1] + b.z * a.data[1][2] + b.w * a.data[1][3],
            z: b.x * a.data[2][0] + b.y * a.data[2][1] + b.z * a.data[2][2] + b.w * a.data[2][3],
            w: b.x * a.data[3][0] + b.y * a.data[3][1] + b.z * a.data[3][2] + b.w * a.data[3][3],
        }
    }
}
