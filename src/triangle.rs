use crate::vector::Vec2;

pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[derive(Default, Clone)]
pub struct Triangle {
    pub points: [Vec2; 3],
}
