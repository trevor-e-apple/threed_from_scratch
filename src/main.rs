extern crate sdl3;

mod vector;

use std::{process::ExitCode, time::Duration};

use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    sys::{
        pixels::SDL_PIXELFORMAT_ARGB8888, render::SDL_TEXTUREACCESS_STREAMING,
    },
};
use vector::{Vector2, Vector3};

const FOV_FACTOR: f32 = 128.0;

struct ColorBuffer {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl ColorBuffer {
    /// Initializes a new instance of Self
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; 4 * width * height],
            width,
            height,
        }
    }

    /// Gets the value of a single pixel
    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        *self.buffer.get(y * self.width + x).unwrap()
    }

    /// Sets a single pixel's value
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let pixel = self.buffer.get_mut(y * self.width + x).unwrap();
        *pixel = color;
    }

    /// Clears the color buffer to a specified color
    pub fn clear(&mut self, color: u32) {
        for pixel in &mut self.buffer {
            *pixel = color;
        }
    }
}

/// Draws a grid on the color buffer at every 'step' pixels
fn draw_dot_grid(color_buffer: &mut ColorBuffer, step: usize, color: u32) {
    // draw horizontal lines
    for y in (0..color_buffer.height).step_by(step) {
        for x in (0..color_buffer.width).step_by(step) {
            color_buffer.set_pixel(x, y, color);
        }
    }
}

fn draw_rect(
    color_buffer: &mut ColorBuffer,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: u32,
) {
    for y in y..(y + height) {
        for x in x..(x + width) {
            color_buffer.set_pixel(x as usize, y as usize, color);
        }
    }
}

fn orthographic_projection(vector: &Vector3) -> Vector2 {
    Vector2 {
        x: FOV_FACTOR * vector.x,
        y: FOV_FACTOR * vector.y,
    }
}

pub fn main() -> ExitCode {
    // TEMP: Create an point cloud
    let point_cloud = {
        const POINT_COUNT: usize = 9 * 9 * 9;
        let mut points = Vec::<Vector3>::with_capacity(POINT_COUNT);
        let mut x: f32 = -1.0;
        while x <= 1.0 {
            let mut y: f32 = -1.0;
            while y <= 1.0 {
                let mut z: f32 = -1.0;
                while z <= 1.0 {
                    points.push(Vector3 { x, y, z });
                    z += 0.25;
                }
                y += 0.25;
            }
            x += 0.25;
        }
        points
    };

    // Init projected points
    let mut projected_points = Vec::<Vector2>::with_capacity(point_cloud.len());

    // Init SDL
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (fullscreen_width, fullscreen_height) = {
        let displays = video_subsystem.displays().unwrap();
        let display = displays.get(0).unwrap();
        let display_mode = display.get_mode().unwrap();

        (display_mode.w, display_mode.h)
    };

    let window_width = (0.5 * fullscreen_width as f32) as u32;
    let window_height = {
        let window_height = (0.75 * window_width as f32) as u32;

        if window_height > fullscreen_height as u32 {
            fullscreen_height as u32
        } else {
            window_height
        }
    };
    let window = video_subsystem
        .window("threed_from_scratch", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    // Initialize color buffer
    let mut color_buffer =
        ColorBuffer::new(window_width as usize, window_height as usize);

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

        projected_points.clear();
        for point in &point_cloud {
            projected_points.push(orthographic_projection(&point));
        }

        // render
        color_buffer.clear(0xFF000000);
        draw_dot_grid(&mut color_buffer, 10, 0xFFFFFFFF);

        for point in &projected_points {
            draw_rect(
                &mut color_buffer,
                (point.x + (window_width as f32 / 2.0)) as i32,
                (point.y + (window_height as f32 / 2.0)) as i32,
                4,
                4,
                0xFFFFFF00,
            );
        }

        // write color buffer to texture
        unsafe {
            color_buffer_texture
                .update(None, color_buffer.buffer.align_to::<u8>().1, pitch)
                .expect("Failure to update texture");
        }
        canvas
            .copy(&color_buffer_texture, None, None)
            .expect("Failure to copy texture to canvas");

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    ExitCode::from(0)
}
