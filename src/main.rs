extern crate sdl3;

use std::time::Duration;

use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    sys::{
        pixels::SDL_PIXELFORMAT_ARGB8888, render::SDL_TEXTUREACCESS_STREAMING,
    },
};

fn clear_color_buffer(
    color_buffer: &mut Vec<u32>,
    window_width: usize,
    window_height: usize,
    color: u32,
) {
    for y in 0..window_height {
        for x in 0..window_width {
            let pixel = color_buffer
                .get_mut(y * window_width + x)
                .expect("Failure to get pixel for writing");
            *pixel = color;
        }
    }
}

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_width = 800;
    let window_height = 600;
    let window = video_subsystem
        .window("threed_from_scratch", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    // Initialize color buffer
    let mut color_buffer: Vec<u32> =
        vec![0; window_width as usize * window_height as usize];

    let texture_creator = canvas.texture_creator();
    let mut color_buffer_texture = unsafe {
        texture_creator
            .create_texture::<PixelFormat>(
                PixelFormat::from_ll(SDL_PIXELFORMAT_ARGB8888),
                SDL_TEXTUREACCESS_STREAMING.into(),
                window_width,
                window_height,
            )
            .unwrap()
    };
    let pitch = (4 * window_width) as usize;

    canvas.set_draw_color(Color::RGB(0xFE, 0x03, 0x6A));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
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

        // render
        canvas.set_draw_color(Color::RGB(0xFE, 0x03, 0x6A));
        canvas.clear();

        clear_color_buffer(
            &mut color_buffer,
            window_width as usize,
            window_height as usize,
            0xFFFFFF00,
        );

        // write color buffer to texture
        unsafe {
            color_buffer_texture
                .update(None, color_buffer.align_to::<u8>().1, pitch)
                .expect("Failure to update texture");
        }
        canvas
            .copy(&color_buffer_texture, None, None)
            .expect("Failure to copy texture to canvas");

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
