use crate::vector::{calc_cross_product, Vector3, Vector4};

pub struct Matrix4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn zero() -> Self {
        Self {
            data: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }

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

    pub fn projection_matrix(
        fov: f32,
        aspect: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        let mut result = Self::zero();

        let fov_cot = 1.0 / fov.tan();
        result.data[0][0] = aspect * fov_cot;
        result.data[1][1] = fov_cot;
        result.data[2][2] = zfar / (zfar - znear);
        result.data[2][3] = (-1.0 * znear * zfar) / (zfar - znear);
        result.data[3][2] = 1.0;

        result
    }

    /// eye_pos: the position of the eye (camera) in world space
    /// target_pos: the position in world space to look at
    /// up: The cross product between this value and the target vector will define the x-axis for the camera 
    pub fn look_at_view_matrix(eye_pos: Vector3, target_pos: Vector3, up: Vector3) -> Self {
        let z = Vector3::calc_normalized_vector(&(&target_pos - &eye_pos)); // the camera's z-axis
        let x = Vector3::calc_normalized_vector(&calc_cross_product(&up, &z)); // the camera's x-axis
        let y = calc_cross_product(&z, &x); // camera's y-axis

        // The view matrix is a translation and a rotation. The rotation corresponds to the 
        // inverse of the rotation of the camera's axes, 
        // This matrix is the result of the matrix multiplication of those two matrices.
        Self {
            data: [
                [x.x, x.y, x.z, -1.0 * Vector3::dot_product(&x, &eye_pos)],
                [y.x, y.y, y.z, -1.0 * Vector3::dot_product(&y, &eye_pos)],
                [z.x, z.y, z.z, -1.0 * Vector3::dot_product(&z, &eye_pos)],
                [0.0, 0.0, 0.0, 1.0]
            ],
        }
    }

    pub fn rotate_around_z(angle: f32) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let mut result = Self::identity();

        result.data[0][0] = cos_angle;
        result.data[1][0] = sin_angle;
        result.data[0][1] = -1.0 * sin_angle;
        result.data[1][1] = cos_angle;

        result
    }

    pub fn rotate_around_x(angle: f32) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let mut result = Self::identity();

        result.data[1][1] = cos_angle;
        result.data[2][1] = sin_angle;
        result.data[1][2] = -1.0 * sin_angle;
        result.data[2][2] = cos_angle;

        result
    }

    pub fn rotate_around_y(angle: f32) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let mut result = Self::identity();

        result.data[0][0] = cos_angle;
        result.data[0][2] = sin_angle;
        result.data[2][0] = -1.0 * sin_angle;
        result.data[2][2] = cos_angle;

        result
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

    pub fn mult_vector(a: &Self, b: &Vector4) -> Vector4 {
        Vector4 {
            x: b.x * a.data[0][0]
                + b.y * a.data[0][1]
                + b.z * a.data[0][2]
                + b.w * a.data[0][3],
            y: b.x * a.data[1][0]
                + b.y * a.data[1][1]
                + b.z * a.data[1][2]
                + b.w * a.data[1][3],
            z: b.x * a.data[2][0]
                + b.y * a.data[2][1]
                + b.z * a.data[2][2]
                + b.w * a.data[2][3],
            w: b.x * a.data[3][0]
                + b.y * a.data[3][1]
                + b.z * a.data[3][2]
                + b.w * a.data[3][3],
        }
    }
}
