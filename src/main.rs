extern crate sdl2;

mod display;
mod mesh;
mod render;
mod triangle;
mod vector2;
mod vector3;

use mesh::load_mesh;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureAccess;
use std::{
    env, println,
    thread::sleep,
    time::{Duration, Instant},
};
use triangle::Triangle;

use crate::{
    mesh::load_cube_mesh,
    render::{draw_grid, draw_rect, draw_triangle, render, ColorBuffer},
    vector2::Vec2,
    vector3::{rotate_vec3, Vec3},
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
    let args: Vec<String> = env::args().collect();
    let mesh_data_path: Option<&String> = args.get(1);

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = match sdl_context.video() {
        Ok(video_subsystem) => video_subsystem,
        Err(_) => todo!(),
    };

    let window_width = 800;
    let window_height = 600;

    // TODO: handle errors
    let window = video_subsystem
        .window("3d from scratch", window_width, window_height)
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

    let mut test_mesh = match mesh_data_path {
        Some(path) => match load_mesh(path) {
            Ok(mesh) => mesh,
            Err(err) => {
                println!("Unable to load mesh at {:?}", path);
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        },
        None => load_cube_mesh(),
    };

    let mut triangles_to_render: Vec<Triangle> = vec![
        Triangle {
            ..Default::default()
        };
        test_mesh.faces.len()
    ];

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
            let window_width_over_two = window_width as f32 / 2.0;
            let window_height_over_two = window_height as f32 / 2.0;

            test_mesh.rotation.x += 0.02;
            test_mesh.rotation.y += 0.02;
            test_mesh.rotation.z += 0.02;
            let rotation = test_mesh.rotation;

            for (face_index, face) in (&test_mesh.faces).into_iter().enumerate() {
                let mesh_vertices: [Vec3; 3] = [
                    test_mesh.vertices[face.a - 1],
                    test_mesh.vertices[face.b - 1],
                    test_mesh.vertices[face.c - 1],
                ];

                let triangle = match triangles_to_render.get_mut(face_index) {
                    Some(triangle) => triangle,
                    None => {
                        // handle error path
                        todo!()
                    }
                };
                for (vertex_index, vertex) in (&mesh_vertices).into_iter().enumerate() {
                    let rotated_point = rotate_vec3(vertex, rotation.x, rotation.y, rotation.z);
                    let mut projected_point = perspective_projection(&Vec3 {
                        z: rotated_point.z - camera_position.z,
                        ..rotated_point
                    });

                    // center our points
                    projected_point.x += window_width_over_two as f32;
                    projected_point.y += window_height_over_two as f32;

                    triangle.points[vertex_index] = projected_point;
                }
            }
        }

        // RENDER
        {
            color_buffer.clear(0xFF000000);

            draw_grid(&mut color_buffer, 10, 10, 0xFFFFFFFF);

            for triangle in &triangles_to_render {
                for point in triangle.points {
                    draw_rect(
                        &mut color_buffer,
                        point.x as i32,
                        point.y as i32,
                        4,
                        4,
                        0xFFFFFF00,
                    );
                }

                draw_triangle(
                    &mut color_buffer,
                    triangle.points[0].x as i32,
                    triangle.points[0].y as i32,
                    triangle.points[1].x as i32,
                    triangle.points[1].y as i32,
                    triangle.points[2].x as i32,
                    triangle.points[2].y as i32,
                    0xFF00FF00,
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
