use crate::vector::Vector2;

#[derive(Clone, Default)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub color: u32,
}

#[derive(Clone, Default)]
pub struct Triangle {
    pub points: [Vector2; 3],
    pub color: u32,
    pub avg_depth: f32,
}
