extern crate sdl2;

mod display;
mod matrix;
mod mesh;
mod render;
mod triangle;
mod vector;
mod vector2;
mod vector3;
mod vector4;

use mesh::load_mesh;
use render::draw_filled_triangle;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureAccess;
use std::{
    env,
    f32::consts::PI,
    println,
    thread::sleep,
    time::{Duration, Instant},
    todo,
};
use triangle::Triangle;
use vector2::Vec2;
use vector4::Vec4;

use crate::{
    matrix::Matrix4,
    mesh::load_cube_mesh,
    render::{draw_grid, draw_rect, draw_triangle, render, ColorBuffer},
    vector3::{unit_normal, Vec3},
};

const FRAMES_PER_SECOND: f64 = 60.0;
const FRAME_TIME_MS: f64 = 1000.0 / FRAMES_PER_SECOND;

#[derive(Debug)]
struct RenderState {
    show_wireframe: bool,
    show_vertices: bool,
    show_filled_triangles: bool,
    show_grid: bool,
    backface_culling_enabled: bool,
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

    let mut color_buffer =
        ColorBuffer::new(window_width as usize, window_height as usize);

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

    let mut render_state = RenderState {
        show_wireframe: true,
        show_vertices: true,
        show_filled_triangles: true,
        show_grid: false,
        backface_culling_enabled: true,
    };

    let camera_position = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
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

    let projection_matrix = {
        let fov = PI / 3.0;
        let aspect = window_height as f32 / window_width as f32;
        let znear = 0.1;
        let zfar = 100.0;

        Matrix4::projection_matrix(fov, aspect, znear, zfar)
    };

    let mut grow = true;
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
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    render_state.show_vertices = !render_state.show_vertices;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    render_state.show_wireframe = !render_state.show_wireframe;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    render_state.show_filled_triangles =
                        !render_state.show_filled_triangles;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    render_state.show_grid = !render_state.show_grid;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    render_state.backface_culling_enabled = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    render_state.backface_culling_enabled = false;
                }
                _ => {}
            }
        }

        // UPDATE
        {
            let window_width_over_two = window_width as f32 / 2.0;
            let window_height_over_two = window_height as f32 / 2.0;

            // test_mesh.translation.x += 0.01;

            test_mesh.rotation.x += 0.01;
            test_mesh.rotation.y += 0.01;
            test_mesh.rotation.z += 0.01;

            if grow {
                test_mesh.scale += Vec3 {
                    x: 0.002,
                    y: 0.002,
                    z: 0.002,
                };
            } else {
                test_mesh.scale -= Vec3 {
                    x: 0.002,
                    y: 0.002,
                    z: 0.002,
                };
            }
            if test_mesh.scale.x > 1.1 {
                grow = false;
            } else if test_mesh.scale.x < 0.5 {
                grow = true;
            }

            let scale_matrix = Matrix4::scale(test_mesh.scale);
            let rotation_matrix = {
                let x_rotation = Matrix4::x_rotation(test_mesh.rotation.x);
                let y_rotation = Matrix4::y_rotation(test_mesh.rotation.y);
                let z_rotation = Matrix4::z_rotation(test_mesh.rotation.z);

                let xy = Matrix4::multiply(x_rotation, y_rotation);

                Matrix4::multiply(xy, z_rotation)
            };
            let translation_matrix = Matrix4::translate(test_mesh.translation);

            let transformation_matrix = {
                // translation must be last (which means its first in the multiplication)
                let m1 = Matrix4::multiply(translation_matrix, rotation_matrix);
                Matrix4::multiply(m1, scale_matrix)
            };

            triangles_to_render.clear();

            for face in &test_mesh.faces {
                let mut mesh_vertices: [Vec3; 3] = [
                    test_mesh.vertices[face.a - 1],
                    test_mesh.vertices[face.b - 1],
                    test_mesh.vertices[face.c - 1],
                ];

                for vertex in &mut mesh_vertices {
                    let transformed_vertex = transformation_matrix
                        .transform(Vec4::from_vec3(vertex));
                    *vertex = Vec3::from_vec4(&transformed_vertex);
                    // move all vertices farther from the monitor
                    vertex.z += 5.0;
                }

                // backface cull check
                let should_cull: bool = if render_state.backface_culling_enabled
                {
                    // find the vectors which define the surface
                    let a = mesh_vertices[0];
                    let b = mesh_vertices[1];
                    let c = mesh_vertices[2];

                    let ab = b - a;
                    let ac = c - a;

                    // calculate the cross product of those vectors to find
                    // -- the surface normal (left-handed system)
                    let surface_normal = unit_normal(&ab, &ac);

                    // find the vector to the camera from the surface
                    let camera_ray = camera_position - a;

                    // find the dot product between the vector to the camera and
                    // -- the surface normal
                    let dot_product =
                        vector3::dot(&camera_ray, &surface_normal);

                    dot_product <= 0.0
                } else {
                    false
                };
                if should_cull {
                    continue;
                }

                let mut triangle = Triangle {
                    color: face.color,
                    ..Default::default()
                };
                for (vertex_index, vertex) in
                    (&mesh_vertices).into_iter().enumerate()
                {
                    // let mut projected_point = perspective_projection(vertex);
                    let mut projected_point = {
                        let mut projected_point = projection_matrix
                            .transform(Vec4::from_vec3(vertex));

                        // perform perspective projection
                        if projected_point.w != 0.0 {
                            projected_point.x /= projected_point.w;
                            projected_point.y /= projected_point.w;
                            projected_point.z /= projected_point.w;
                        }

                        Vec2 {
                            x: projected_point.x,
                            y: projected_point.y,
                        }
                    };

                    // scale into view
                    projected_point.x *= window_width_over_two;
                    projected_point.y *= window_height_over_two;

                    // center our points
                    projected_point.x += window_width_over_two;
                    projected_point.y += window_height_over_two;

                    triangle.points[vertex_index] = projected_point;

                    // accumulate sum for avg depth
                    triangle.avg_depth += vertex.z;
                }
                // compute average depth
                triangle.avg_depth /= 3.0;

                triangles_to_render.push(triangle);
            }
        }

        // RENDER
        {
            // sort the triangles by their average depth
            triangles_to_render
                .sort_by(|a, b| b.avg_depth.partial_cmp(&a.avg_depth).unwrap());

            color_buffer.clear(0xFF000000);

            if render_state.show_grid {
                draw_grid(&mut color_buffer, 10, 10, 0xFFFFFFFF);
            }

            for triangle in &triangles_to_render {
                if render_state.show_vertices {
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
                }

                if render_state.show_filled_triangles {
                    draw_filled_triangle(
                        &mut color_buffer,
                        triangle,
                        triangle.color,
                    );
                }

                if render_state.show_wireframe {
                    draw_triangle(&mut color_buffer, triangle, 0xFF00FF00);
                }
            }

            let render_result =
                render(&mut color_buffer, &mut canvas, &mut texture);

            if !render_result {
                break 'running;
            }
        }

        // SLEEP
        let end = Instant::now();
        let frame_time = end.duration_since(start).as_millis() as f64;
        let mut sleep_duration = FRAME_TIME_MS - frame_time;
        if sleep_duration < 0.0 {
            sleep_duration = 0.0;
        }
        // assumes that our sleep time is never more than 1 second
        sleep(Duration::new(0, (1000000.0 * sleep_duration) as u32));
        if sleep_duration == 0.0 {
            print!("Missed frame time. ");
            print!("Expected: {FRAME_TIME_MS} ms. ");
            print!("Actual: {frame_time} ms. ");
            println!();
        }
    }
}
