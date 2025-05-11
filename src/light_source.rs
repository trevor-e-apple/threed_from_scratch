use crate::vector::Vector3;

pub struct LightSource {
    pub direction: Vector3,
}

impl LightSource {
    pub fn new(direction: Vector3) -> Self {
        let mut direction = direction;
        direction.normalize();
        Self { direction }
    }
}

pub fn apply_intensity(color: u32, intensity: f32) -> u32 {
    let a = color & 0xFF000000;
    let r = ((color & 0x00FF0000) as f32 * intensity) as u32;
    let g = ((color & 0x0000FF00) as f32 * intensity) as u32;
    let b = ((color & 0x000000FF) as f32 * intensity) as u32;

    a | (r & 0x00FF0000) | (g & 0x0000FF00) | (b & 0x000000FF)
}
