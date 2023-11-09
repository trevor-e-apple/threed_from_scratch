use crate::buffer_utils::get_index_by_coord;

pub struct ZBuffer {
    buffer: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl ZBuffer {
    // TODO: documentation
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0.0; width * height],
            width,
            height,
        }
    }

    pub fn get_pixel_value(&self, x: usize, y: usize) -> f32 {
        let index = get_index_by_coord(x, y, self.width);
        self.buffer[index]
    }

    pub fn set_pixel_value(&mut self, x: usize, y: usize, value: f32) {
        let index = get_index_by_coord(x, y, self.width);
        self.buffer[index] = value;
    }

    pub fn clear(&mut self) {
        for pixel_value in &mut self.buffer {
            // 1.0 is the maximum value of the reciprocal of the depth (LHS)
            *pixel_value = 1.0;
        }
    }
}
