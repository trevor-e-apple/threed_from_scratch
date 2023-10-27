use std::mem::size_of;

use crate::{color::Color, buffer_utils::get_index_by_coord};

pub struct ColorBuffer {
    buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl ColorBuffer {
    // TODO: documentation
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    // TODO: documentation
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u32> {
        let index = get_index_by_coord(x, y, self.width);
        self.buffer.get_mut(index)
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in &mut self.buffer {
            *pixel = color;
        }
    }

    pub unsafe fn get_raw_data(&mut self) -> &[u8] {
        std::slice::from_raw_parts(
            self.buffer.as_ptr() as *const u8,
            self.buffer.len() * size_of::<u32>(),
        )
    }
}