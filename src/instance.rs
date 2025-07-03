use crate::vector::Vector4;

pub struct Instance {
    pub orientation: Vector4,
    pub translation: Vector4,
    pub scale: f32,
    pub mesh_handle: usize,
    pub texture_handle: usize,
}
