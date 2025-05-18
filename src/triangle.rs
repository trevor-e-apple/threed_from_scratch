use crate::{texture::Texture2, vector::Vector2};

#[derive(Clone, Default)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub a_uv: Texture2,
    pub b_uv: Texture2,
    pub c_uv: Texture2,
    pub color: u32,
}

#[derive(Clone, Default)]
pub struct Triangle {
    pub points: [Vector2; 3],
    pub texel_coordinates: [Texture2; 3],
    pub color: u32,
    pub avg_depth: f32,
}
