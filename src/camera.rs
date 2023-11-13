use crate::vector3::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
}

pub const CAMERA_UP: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};