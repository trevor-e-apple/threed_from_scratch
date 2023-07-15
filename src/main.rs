extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureAccess;
use sdl2::video::FullscreenType;
use std::mem::size_of;
use std::time::Duration;

struct ColorBuffer {
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

fn draw_grid(
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

pub fn main() {
    // TODO: Handle errors
    let sdl_context = sdl2::init().unwrap();

    // TODO: handle errors
    let video_subsystem = sdl_context.video().unwrap();

    // get display mode to make full screen possible
    let display_mode = match video_subsystem.current_display_mode(0) {
        Ok(value) => value,
        Err(err) => {
            println!("Failed to get display mode with error: {:?}", err);
            assert!(false);
            return;
        }
    };

    let fullscreen_width = display_mode.w as u32;
    let fullscreen_height = display_mode.h as u32;

    let window_width = fullscreen_width;
    let window_height = fullscreen_height;

    // TODO: handle errors
    let mut window = video_subsystem
        .window("rust-sdl2 demo", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    match window.set_fullscreen(FullscreenType::True) {
        Err(err) => {
            println!("Error setting to fullscreen: {:?}", err);
        }
        _ => {}
    };

    // TODO: handle errors
    let mut canvas = window.into_canvas().build().unwrap();

    let mut color_buffer = ColorBuffer::new(window_width as usize, window_height as usize);

    let texture_creator = canvas.texture_creator();
    let mut texture = match texture_creator.create_texture(
        None,
        TextureAccess::Streaming,
        window_width,
        window_height,
    ) {
        Ok(result) => result,
        Err(_) => todo!(),
    };

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // TODO: handle errors
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // process inputs
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // update
        // The rest of the game loop goes here...

        // RENDER
        {
            color_buffer.clear(0xFFFF0000);

            draw_grid(&mut color_buffer, 10, 10, 0xFFFFFFFF);

            // TODO: Error handling
            match texture.update(
                None,
                unsafe { color_buffer.get_raw_data() },
                (window_width as usize) * size_of::<u32>(),
            ) {
                Err(err) => {
                    println!("Texture update failed: {:?}", err);
                    break 'running;
                }
                _ => {}
            };

            // TODO: error handling
            match canvas.copy(&texture, None, None) {
                Err(err) => {
                    println!("Canvas copy failed: {:?}", err);
                    break 'running;
                }
                _ => {}
            };

            canvas.present();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
