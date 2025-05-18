extern crate sdl3;

mod light_source;
mod matrix;
mod mesh;
mod point;
mod render;
mod texture;
mod triangle;
mod vector;

use std::{
    env, process::ExitCode, slice, time::{Duration, Instant}
};

use light_source::{apply_intensity, LightSource};
use matrix::Matrix4;
use mesh::{load_obj_mesh, MESH_FACES, MESH_VERTICES};
use render::{
    draw_filled_triangle, draw_textured_triangle, draw_triangle, draw_triangle_vertices, perspective_projection, ColorBuffer
};
use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    sys::{
        pixels::SDL_PIXELFORMAT_ARGB8888, render::SDL_TEXTUREACCESS_STREAMING,
    },
};
use texture::REDBRICK_TEXTURE;
use triangle::Triangle;
use vector::{calc_cross_product, Vector2, Vector3, Vector4};

const FRAMES_PER_SEC: f32 = 30.0;
const FRAME_TARGET_TIME_MS: f32 = 1000.0 / FRAMES_PER_SEC;
const FRAME_TARGET_TIME_NS: u32 = (1000.0 * FRAME_TARGET_TIME_MS) as u32;

#[derive(PartialEq)]
enum RenderMode {
    Wireframe,
    WireframeVertices,
    FilledTriangles,
    WireframeFilledTriangles,
    TexturedTriangles,
    WireframeTexturedTriangles,
}

#[derive(PartialEq)]
enum BackfaceCullingMode {
    Enabled,
    Disabled,
}

pub fn main() -> ExitCode {
    // Grab arguments
    let args: Vec<String> = env::args().collect();

    let (vertices, faces) = if args.len() == 1 {
        println!("No model path passed in. Using in-memory cube data");
        (MESH_VERTICES.to_vec(), MESH_FACES.to_vec())
    } else {
        let model_path = args[1].clone();
        load_obj_mesh(&model_path)
    };

    let texture = unsafe {
        let redbrick_texture_ptr = REDBRICK_TEXTURE.as_ptr() as *const u32;
        slice::from_raw_parts(redbrick_texture_ptr, REDBRICK_TEXTURE.len() / size_of::<u32>())
    };

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
    let aspect_ratio = window_height as f32 / window_width as f32;

    let fov = (std::f64::consts::PI / 3.0) as f32;

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

    // Initialize triangle buffer
    let mut triangles_to_render: Vec<Triangle> = Vec::new();

    // Initialize camera
    let camera_position = Vector4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    // Initialize model orientation
    let mut orientation = Vector4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    // Initialize model translation
    let mut translation = Vector4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    // Initialize model scale
    let mut scale: f32 = 1.0;
    let model_displacement = Vector4 {
        x: 0.0,
        y: 0.0,
        z: 5.0,
        w: 1.0,
    };

    // Initialize light source
    let light_source = LightSource::new(Vector3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    });

    // Initialize render mode
    let mut render_mode: RenderMode = RenderMode::FilledTriangles;
    let mut culling_mode: BackfaceCullingMode = BackfaceCullingMode::Enabled;

    canvas.set_draw_color(Color::RGB(0xFE, 0x03, 0x6A));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let frame_start_time = Instant::now();

        // process input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    culling_mode = BackfaceCullingMode::Enabled;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    culling_mode = BackfaceCullingMode::Disabled;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::_1),
                    ..
                } => {
                    render_mode = RenderMode::WireframeVertices;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::_2),
                    ..
                } => {
                    render_mode = RenderMode::Wireframe;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::_3),
                    ..
                } => {
                    render_mode = RenderMode::FilledTriangles;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::_4),
                    ..
                } => {
                    render_mode = RenderMode::WireframeFilledTriangles;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::_5),
                    ..
                } => {
                    render_mode = RenderMode::TexturedTriangles;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::_6),
                    ..
                } => {
                    render_mode = RenderMode::WireframeTexturedTriangles;
                }
                _ => {}
            }
        }

        // update
        {
            orientation.x += 0.0025;
            orientation.y += 0.0025;
            orientation.z += 0.0025;

            // translation.x += 0.005;
            // translation.z += 0.005;

            // scale += 0.0001;
        }

        // Transform and project
        {
            // loop over faces
            triangles_to_render.clear();
            for face in &faces {
                let vertices: [Vector3; 3] = [
                    vertices[face.a - 1].clone(),
                    vertices[face.b - 1].clone(),
                    vertices[face.c - 1].clone(),
                ];

                let mut transformed_vertices: [Vector4; 3] = [
                    Vector4 {
                        ..Default::default()
                    },
                    Vector4 {
                        ..Default::default()
                    },
                    Vector4 {
                        ..Default::default()
                    },
                ];

                let world_matrix = {
                    let world_matrix = Matrix4::identity();

                    let world_matrix = Matrix4::mult_mat4(
                        &Matrix4::scale(scale, scale, scale),
                        &world_matrix,
                    );
                    let world_matrix = Matrix4::mult_mat4(
                        &Matrix4::rotate_around_x(orientation.x),
                        &world_matrix,
                    );
                    let world_matrix = Matrix4::mult_mat4(
                        &Matrix4::rotate_around_y(orientation.y),
                        &world_matrix,
                    );
                    let world_matrix = Matrix4::mult_mat4(
                        &Matrix4::rotate_around_z(orientation.z),
                        &world_matrix,
                    );

                    let translation_matrix = Matrix4::translate(
                        translation.x,
                        translation.y,
                        translation.z,
                    );
                    let world_matrix =
                        Matrix4::mult_mat4(&translation_matrix, &world_matrix);

                    world_matrix
                };

                // Transform
                for (index, vertex) in vertices.into_iter().enumerate() {
                    let transformed_vertex = {
                        let transformed_vertex = {
                            let transformed_vertex = Matrix4::mult_vector(
                                &world_matrix,
                                &Vector4::from_vector3(&vertex),
                            );

                            let transformed_vertex =
                                &transformed_vertex + &model_displacement;

                            transformed_vertex
                        };

                        transformed_vertex
                    };
                    transformed_vertices[index] = transformed_vertex;
                }

                // Pull out face vectors (left-handed system)
                /*
                     A
                    /  \
                    C - B
                */
                let vector_a = &Vector3::from_vector4(&transformed_vertices[0]);
                let vector_b = &Vector3::from_vector4(&transformed_vertices[1]);
                let vector_c = &Vector3::from_vector4(&transformed_vertices[2]);

                // Find face normal
                let face_normal: Vector3 = {
                    let ab_vector = {
                        let mut ab_vector = vector_b - vector_a;
                        ab_vector.normalize();
                        ab_vector
                    };
                    let ac_vector = {
                        let mut ac_vector = vector_c - vector_a;
                        ac_vector.normalize();
                        ac_vector
                    };
                    let face_normal = {
                        let mut face_normal =
                            calc_cross_product(&ab_vector, &ac_vector);
                        face_normal.normalize();
                        face_normal
                    };

                    face_normal
                };

                // Backface culling
                let culled: bool = if culling_mode
                    == BackfaceCullingMode::Enabled
                {
                    // calculate the to-camera vector
                    let face_to_camera =
                        &Vector3::from_vector4(&camera_position) - &vector_a;

                    let dot_product =
                        Vector3::dot_product(&face_normal, &face_to_camera);

                    dot_product < 0.0
                } else {
                    false
                };

                // Project
                if !culled {
                    // Lighting
                    let light_intensity: f32 = {
                        let dot_product = Vector3::dot_product(
                            &face_normal,
                            &light_source.direction,
                        );

                        /*
                         * If the dot product is negative, then the normal and the light are pointing in opposite directions,
                         * which means that there should be light
                         *
                         * If the dot product is 0, then the normal and the light are orthogonal, and there should be no light.
                         *
                         * If the dot product is positive, then the normal is pointing in the opposite direction of the light, and there
                         * should be no light.
                         *
                         * Note that if both vectors are normalized, then the dot product shall be in the range [-1.0, 1.0]
                         */
                        if dot_product < 0.0 {
                            -1.0 * dot_product
                        } else {
                            0.0
                        }
                    };

                    if light_intensity > 0.0 {
                        let color =
                            apply_intensity(face.color, light_intensity);
                        let mut triangle = Triangle {
                            texel_coordinates: [face.a_uv.clone(), face.b_uv.clone(), face.c_uv.clone()],
                            color,
                            ..Default::default()
                        };

                        let projection_matrix = Matrix4::projection_matrix(
                            fov,
                            aspect_ratio,
                            0.1,
                            100.0,
                        );

                        let mut avg_depth = 0.0;
                        for (index, vertex) in
                            (&mut transformed_vertices).into_iter().enumerate()
                        {
                            match perspective_projection(
                                &projection_matrix,
                                vertex,
                            ) {
                                Some(projected_point) => {
                                    let mut projected_point =
                                        Vector2::from_vector4(&projected_point);
                                    // perform windowing transform (scale then translate)
                                    // the division by 2 is b/c we are mapping the canonical view volume (which has bounds x,y: [-1, 1]) to screen
                                    // space (which has bounds x: [0, window_width], y: [0, window_height])
                                    {
                                        projected_point.x *=
                                            window_width as f32 / 2.0;
                                        projected_point.y *=
                                            window_height as f32 / 2.0;

                                        // since y grows down in screen space, but up in world space / canonical image space
                                        projected_point.y *= -1.0;

                                        projected_point.x +=
                                            window_width as f32 / 2.0;
                                        projected_point.y +=
                                            window_height as f32 / 2.0;
                                    }

                                    triangle.points[index] = projected_point;
                                    avg_depth += vertex.z;
                                }
                                None => {}
                            }
                        }
                        avg_depth /= transformed_vertices.len() as f32;
                        triangle.avg_depth = avg_depth;

                        triangles_to_render.push(triangle);
                    }
                }
            }
        }

        // sort triangles by average depth (painter's algorithm)
        triangles_to_render
            .sort_by(|a, b| b.avg_depth.partial_cmp(&a.avg_depth).unwrap());

        // render
        {
            color_buffer.clear(0xFF000000);

            if render_mode == RenderMode::FilledTriangles
                || render_mode == RenderMode::WireframeFilledTriangles
            {
                for triangle in &triangles_to_render {
                    draw_filled_triangle(
                        &mut color_buffer,
                        triangle,
                        triangle.color,
                    );
                }
            } else if render_mode == RenderMode::TexturedTriangles
                || render_mode == RenderMode::WireframeTexturedTriangles
            {
                for triangle in &triangles_to_render {
                    draw_textured_triangle(&mut color_buffer, &triangle, texture);
                }
            }

            if !(render_mode == RenderMode::FilledTriangles
                || render_mode == RenderMode::TexturedTriangles)
            {
                for triangle in &triangles_to_render {
                    draw_triangle(&mut color_buffer, triangle, 0xFFFFFFFF);
                }
            }

            if render_mode == RenderMode::WireframeVertices {
                for triangle in &triangles_to_render {
                    draw_triangle_vertices(
                        &mut color_buffer,
                        triangle,
                        0xFFFF0000,
                    );
                }
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
        }

        let frame_duration = Instant::now().duration_since(frame_start_time);
        if frame_duration.as_millis() < (FRAME_TARGET_TIME_MS as u128) {
            let sleep_time =
                frame_duration - Duration::new(0, FRAME_TARGET_TIME_NS);
            std::thread::sleep(sleep_time);
        }
    }

    ExitCode::from(0)
}
