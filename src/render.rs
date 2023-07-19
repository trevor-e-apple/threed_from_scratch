use std::mem::size_of;

use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};

pub struct ColorBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
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
        let index = y * self.width + x;
        return self.buffer.get_mut(index);
    }

    pub fn clear(&mut self, color: u32) {
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

pub fn draw_grid(
    color_buffer: &mut ColorBuffer,
    width_interval: usize,
    height_interval: usize,
    color: u32,
) {
    for y in 0..color_buffer.height {
        for x in 0..color_buffer.width {
            if (x % width_interval == 0) || (y % height_interval == 0) {
                let pixel = match color_buffer.get_mut(x, y) {
                    Some(value) => value,
                    None => todo!(),
                };
                *pixel = color;
            }
        }
    }
}

pub fn draw_rect(
    color_buffer: &mut ColorBuffer,
    top_left_x: i32,
    top_left_y: i32,
    width: i32,
    height: i32,
    color: u32,
) {
    // calculate start / stop x coordinates
    let (start_x, end_x) = {
        let mut start_x = top_left_x;
        let buffer_width = color_buffer.width as i32;

        if start_x < 0 {
            start_x = 0;
        } else if start_x > buffer_width {
            start_x = buffer_width;
        }

        let mut end_x = top_left_x + width;
        if end_x < 0 {
            end_x = 0;
        } else if end_x > buffer_width {
            end_x = buffer_width;
        }

        (start_x, end_x)
    };

    // calculate start / stop y coordinates
    let (start_y, end_y) = {
        let mut start_y = top_left_y;
        let buffer_height = color_buffer.height as i32;

        if start_y < 0 {
            start_y = 0;
        } else if start_y > buffer_height {
            start_y = buffer_height;
        }

        let mut end_y = top_left_y + height;
        if end_y < 0 {
            end_y = 0;
        } else if end_y > buffer_height {
            end_y = buffer_height;
        }

        (start_y, end_y)
    };

    for y in start_y..end_y {
        for x in start_x..end_x {
            let pixel = match color_buffer.get_mut(x as usize, y as usize) {
                Some(value) => value,
                None => todo!(),
            };
            *pixel = color;
        }
    }
}

pub fn draw_pixel(color_buffer: &mut ColorBuffer, x: i32, y: i32, color: u32) {
    let pixel = match color_buffer.get_mut(x as usize, y as usize) {
        Some(value) => value,
        None => todo!(),
    };
    *pixel = color;
}


/// 
/// color_buffer: the color_buffer that contains the render data
/// canvas: the sdl canvas to draw to
/// texture: the texture to copy the color buffer to
pub fn render(
    color_buffer: &mut ColorBuffer,
    canvas: &mut Canvas<Window>,
    texture: &mut Texture,
) -> bool {
    color_buffer.clear(0xFFFF0000);

    draw_grid(color_buffer, 10, 10, 0xFFFFFFFF);

    {
        let rect_width = (color_buffer.width / 10) as i32;
        let rect_height = (color_buffer.height / 10) as i32;
        draw_rect(color_buffer, 0, 0, rect_width, rect_height, 0xFF000000);
    }

    let window_width = color_buffer.width;

    // TODO: Error handling
    match texture.update(
        None,
        unsafe { color_buffer.get_raw_data() },
        (window_width as usize) * size_of::<u32>(),
    ) {
        Err(err) => {
            println!("Texture update failed: {:?}", err);
            return false;
        }
        _ => {}
    };

    // TODO: error handling
    match canvas.copy(&texture, None, None) {
        Err(err) => {
            println!("Canvas copy failed: {:?}", err);
            return false;
        }
        _ => {}
    };

    canvas.present();

    true
}
