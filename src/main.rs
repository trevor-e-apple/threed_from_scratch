extern crate sdl2;

pub mod render;
pub mod vector;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureAccess;
use sdl2::video::FullscreenType;
use std::time::Duration;

use crate::{render::{ColorBuffer, render}, vector::Vec3};

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

    // setup cube points
    const CUBE_POINT_DIM: usize = 9;
    const CUBE_POINT_COUNT: usize = CUBE_POINT_DIM * CUBE_POINT_DIM * CUBE_POINT_DIM;
    let mut cube_points: Vec<Vec3> = Vec::with_capacity(CUBE_POINT_COUNT);
    for x_index in 0..CUBE_POINT_DIM {
        for y_index in 0..CUBE_POINT_DIM {
            for z_index in 0..CUBE_POINT_DIM {
                cube_points.push(
                    Vec3::new(
                        x_index as f32 * 0.25 - 1.0,
                        y_index as f32 * 0.25 - 1.0,
                        z_index as f32 * 0.25 - 1.0,
                    )
                );
            }
        }
    }

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

        let render_result = render(&mut color_buffer, &mut canvas, &mut texture);

        if !render_result {
            break 'running;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
