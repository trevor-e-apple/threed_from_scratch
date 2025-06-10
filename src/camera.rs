use crate::{
    matrix::Matrix4,
    vector::{calc_cross_product, Vector3, Vector4},
};

pub struct Camera {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            target: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            up: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        }
    }

    /// Returns the view matrix for the camera
    ///
    /// Args
    /// eye_pos: the position of the eye (camera) in world space
    /// target_pos: the position in world space to look at
    /// up: The cross product between this value and the target vector will define the x-axis for the camera
    pub fn view_matrix(&self) -> Matrix4 {
        let eye_pos = &self.position;
        let target_pos = &self.target;
        let up = &self.up;

        let z = Vector3::calc_normalized_vector(&(target_pos - eye_pos)); // the camera's z-axis
        let x = Vector3::calc_normalized_vector(&calc_cross_product(up, &z)); // the camera's x-axis
        let y = calc_cross_product(&z, &x); // camera's y-axis

        // The view matrix is a translation and a rotation. The rotation corresponds to the
        // inverse of the rotation of the camera's axes,
        // This matrix is the result of the matrix multiplication of those two matrices.
        Matrix4 {
            data: [
                [x.x, x.y, x.z, -1.0 * Vector3::dot_product(&x, eye_pos)],
                [y.x, y.y, y.z, -1.0 * Vector3::dot_product(&y, eye_pos)],
                [z.x, z.y, z.z, -1.0 * Vector3::dot_product(&z, eye_pos)],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}
