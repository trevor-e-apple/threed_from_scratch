extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureAccess;
use std::mem::size_of;
use std::time::Duration;

pub fn main() {
    // TODO: Handle errors
    let sdl_context = sdl2::init().unwrap();

    // TODO: handle errors
    let video_subsystem = sdl_context.video().unwrap();

    const WINDOW_WIDTH: u32 = 800;
    const WINDOW_HEIGHT: u32 = 600;

    // TODO: handle errors
    let window = video_subsystem
        .window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    // TODO: handle errors
    let mut canvas = window.into_canvas().build().unwrap();

    let mut color_buffer: Vec<u32> = vec![0; WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize];

    let texture_creator = canvas.texture_creator();
    let mut texture = match texture_creator.create_texture(
        None,
        TextureAccess::Streaming,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
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
            for pixel in &mut color_buffer {
                *pixel = 0xFFFF0000;
            }

            let color_buffer_conversion: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    color_buffer.as_ptr() as *const u8,
                    color_buffer.len() * size_of::<u32>(),
                )
            };

            // TODO: Error handling
            match texture.update(
                None,
                color_buffer_conversion,
                (WINDOW_WIDTH as usize) * size_of::<u32>(),
            ) {
                Err(err) => {
                    println!("Texture update failed: {:?}", err);
                    break 'running;
                },
                _ => {},
            };

            // TODO: error handling
            match canvas.copy(&texture, None, None) {
                Err(err) => {
                    println!("Canvas copy failed: {:?}", err);
                    break 'running;
                },
                _ => {},
            };

            canvas.present();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
