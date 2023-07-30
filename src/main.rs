extern crate sdl2;

pub mod display;
pub mod render;
pub mod vector;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureAccess;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    render::{draw_grid, draw_rect, render, ColorBuffer},
    vector::{rotate_vec3, Vec2, Vec3},
};

const FOV_FACTOR: f32 = 640.0;
const FRAMES_PER_SECOND: f64 = 60.0;
const FRAME_TIME_MS: f64 = 1000.0 / FRAMES_PER_SECOND;

pub fn orthographic_projection(point: &Vec3) -> Vec2 {
    Vec2 {
        x: (FOV_FACTOR * point.x),
        y: (FOV_FACTOR * point.y),
    }
}

pub fn perspective_projection(point: &Vec3) -> Vec2 {
    Vec2 {
        x: (FOV_FACTOR * point.x) / point.z,
        y: (FOV_FACTOR * point.y) / point.z,
    }
}

pub fn main() {
    // TODO: Handle errors
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = match sdl_context.video() {
        Ok(video_subsystem) => video_subsystem,
        Err(_) => todo!(),
    };

    let window_width = 800;
    let window_height = 600;

    // TODO: handle errors
    let window = video_subsystem
        .window("rust-sdl2 demo", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

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

    let camera_position = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -5.0,
    };

    // setup cube points
    const CUBE_POINT_DIM: usize = 9;
    const CUBE_POINT_COUNT: usize = CUBE_POINT_DIM * CUBE_POINT_DIM * CUBE_POINT_DIM;
    let mut cube_points: Vec<Vec3> = Vec::with_capacity(CUBE_POINT_COUNT);
    for x_index in 0..CUBE_POINT_DIM {
        for y_index in 0..CUBE_POINT_DIM {
            for z_index in 0..CUBE_POINT_DIM {
                cube_points.push(Vec3 {
                    x: x_index as f32 * 0.25 - 1.0,
                    y: y_index as f32 * 0.25 - 1.0,
                    z: z_index as f32 * 0.25 - 1.0,
                });
            }
        }
    }
    let mut projected_cube_points: Vec<Vec2> = vec![
        Vec2 {
            ..Default::default()
        };
        CUBE_POINT_COUNT
    ];

    let mut x_rotation = 0.0;
    let mut y_rotation = 0.0;
    let mut z_rotation = 0.0;

    // TODO: handle errors
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let start = Instant::now();

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

        // UPDATE
        {
            x_rotation += 0.02;
            y_rotation += 0.02;
            z_rotation += 0.02;
            for (index, cube_point) in cube_points.iter().enumerate() {
                // rotate
                let rotated_point = rotate_vec3(&cube_point, x_rotation, y_rotation, z_rotation);

                // project
                let projected_point = match projected_cube_points.get_mut(index) {
                    Some(projected_point) => projected_point,
                    None => todo!(),
                };
                *projected_point = perspective_projection(&Vec3 {
                    x: rotated_point.x,
                    y: rotated_point.y,
                    z: rotated_point.z - camera_position.z,
                });
            }
        }

        // RENDER
        {
            color_buffer.clear(0xFF000000);

            draw_grid(&mut color_buffer, 10, 10, 0xFFFFFFFF);

            let window_width_over_two = window_width as f32 / 2.0;
            let window_height_over_two = window_height as f32 / 2.0;
            for point in &projected_cube_points {
                draw_rect(
                    &mut color_buffer,
                    (point.x + window_width_over_two) as i32,
                    (point.y + window_height_over_two) as i32,
                    4,
                    4,
                    0xFFFFFF00,
                );
            }

            let render_result = render(&mut color_buffer, &mut canvas, &mut texture);

            if !render_result {
                break 'running;
            }
        }

        // SLEEP
        let end = Instant::now();
        let mut sleep_duration = FRAME_TIME_MS - end.duration_since(start).as_millis() as f64;
        if sleep_duration < 0.0 {
            sleep_duration = 0.0;
        }
        // assumes that our sleep time is never more than 1 second
        sleep(Duration::new(0, (1000.0 * sleep_duration) as u32));
    }
}
