extern crate sdl3;

mod camera;
mod clipping;
mod light_source;
mod matrix;
mod mesh;
mod point;
mod projection;
mod render;
mod texture;
mod triangle;
mod vector;

use std::{
    env,
    process::ExitCode,
    time::{Duration, Instant},
};

use light_source::{apply_intensity, LightSource};
use matrix::Matrix4;
use mesh::{load_obj_mesh, load_test_mesh};
use render::{
    draw_filled_triangle, draw_textured_triangle, draw_triangle,
    draw_triangle_vertices, ColorBuffer,
};
use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    sys::{
        pixels::SDL_PIXELFORMAT_ARGB8888, render::SDL_TEXTUREACCESS_STREAMING,
    },
};
use texture::{load_png_texture, load_test_texture};
use triangle::Triangle;
use vector::{calc_cross_product, Vector3, Vector4};

use crate::{
    camera::Camera,
    clipping::{clip_triangle, FrustumPlanes},
    projection::project_triangles,
};

const FRAMES_PER_SEC: f32 = 30.0;
const FRAME_TARGET_TIME_MS: f32 = 1000.0 / FRAMES_PER_SEC;
const FRAME_TARGET_TIME_NS: u32 = (1000.0 * FRAME_TARGET_TIME_MS) as u32;
const CAMERA_UNITS_PER_FRAME: f32 = 2.0 * (1.0 / FRAMES_PER_SEC); // speed in units / frame

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

    let (mesh, texture) = if args.len() == 1 {
        println!("No model path passed in. Using in-memory cube data");
        (load_test_mesh(), load_test_texture())
    } else if args.len() == 3 {
        let model_path = args[1].clone();
        let texture_path = args[2].clone();
        (load_obj_mesh(&model_path), load_png_texture(&texture_path))
    } else {
        println!("Bad arguments");
        return ExitCode::from(1);
    };

    // Init SDL
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // let (fullscreen_width, fullscreen_height) = {
    //     let displays = video_subsystem.displays().unwrap();
    //     let display = displays.get(0).unwrap();
    //     let display_mode = display.get_mode().unwrap();

    //     (display_mode.w, display_mode.h)
    // };

    // let window_width = (0.5 * fullscreen_width as f32) as u32;
    // let window_height = {
    //     let window_height = (0.75 * window_width as f32) as u32;

    //     if window_height > fullscreen_height as u32 {
    //         fullscreen_height as u32
    //     } else {
    //         window_height
    //     }
    // };
    let window_width = 800;
    let window_height = 600;
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
    let mut camera = Camera::new();

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
        z: 5.0,
        w: 1.0,
    };

    // Initialize model scale
    let mut scale: f32 = 1.0;

    // Initialize light source
    // NOTE: the light direction is in camera space, not world space, since it is not
    // transformed via the view matrix
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

        let (camera_direction, camera_right) = {
            // direction that the camera is pointing relative to the camera's origin
            let mut camera_direction = &camera.target - &camera.position;
            camera_direction.normalize();

            let mut camera_right =
                calc_cross_product(&camera_direction, &camera.up);
            camera_right.normalize();

            (camera_direction, camera_right)
        };

        // process input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    let delta = CAMERA_UNITS_PER_FRAME * &camera_direction;
                    camera.position = &camera.position + &delta;
                    camera.target = &camera.target + &delta;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let delta =
                        -1.0 * CAMERA_UNITS_PER_FRAME * &camera_direction;
                    camera.position = &camera.position + &delta;
                    camera.target = &camera.target + &delta;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    let delta = CAMERA_UNITS_PER_FRAME * &camera_right;
                    camera.position = &camera.position + &delta;
                    camera.target = &camera.target + &delta;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    let delta = -1.0 * CAMERA_UNITS_PER_FRAME * &camera_right;
                    camera.position = &camera.position + &delta;
                    camera.target = &camera.target + &delta;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    // Move camera up
                    let delta = CAMERA_UNITS_PER_FRAME * &camera.up;
                    camera.position = &camera.position + &delta;
                    camera.target = &camera.target + &delta;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    // move camera down
                    let delta = -1.0 * CAMERA_UNITS_PER_FRAME * &camera.up;
                    camera.position = &camera.position + &delta;
                    camera.target = &camera.target + &delta;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    // Rotate camera left
                    let rotation_matrix = Matrix4::rotate_around_y(-0.02);
                    let new_direction = Matrix4::mult_vector(
                        &rotation_matrix,
                        &Vector4::from_vector3(&camera_direction),
                    );

                    // compute new target
                    camera.target = &camera.position
                        + &Vector3::from_vector4(&new_direction);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let rotation_matrix = Matrix4::rotate_around_y(0.02);
                    let new_direction = Matrix4::mult_vector(
                        &rotation_matrix,
                        &Vector4::from_vector3(&camera_direction),
                    );

                    // compute new target
                    camera.target = &camera.position
                        + &Vector3::from_vector4(&new_direction);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    culling_mode = BackfaceCullingMode::Enabled;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::V),
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
            // orientation.x += 0.0025;
            // orientation.y += 0.0025;
            // orientation.z += 0.0025;

            // translation.x += 0.005;
            // translation.z += 0.005;

            // scale += 0.0001;

            // camera.position.x += 0.0025;
            // camera.position.y += 0.0025;
        }

        // Transform and project
        {
            // World matrix is invariant for each face
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

            // The view matrix is invariant for each face
            let view_matrix = camera.view_matrix();

            let znear = 0.1;
            let zfar = 20.0;

            // The frustum planes are invariant for each face
            let frustum_planes = FrustumPlanes::new(znear, zfar, fov);

            // The projection matrix is invariant for each face
            let projection_matrix =
                Matrix4::projection_matrix(fov, aspect_ratio, znear, zfar);

            // loop over faces
            triangles_to_render.clear();
            for face in &mesh.faces {
                let vertices: [Vector3; 3] = mesh.get_vertices(face);

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

                // Transform
                for (index, vertex) in vertices.into_iter().enumerate() {
                    // world transform
                    let transformed_vertex = Matrix4::mult_vector(
                        &world_matrix,
                        &Vector4::from_vector3(&vertex),
                    );
                    // view transform
                    let transformed_vertex =
                        Matrix4::mult_vector(&view_matrix, &transformed_vertex);
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
                let culled: bool =
                    if culling_mode == BackfaceCullingMode::Enabled {
                        // Calculate the to-camera vector.
                        // Since this is performed after the view matrix transform,
                        // the camera is at the origin
                        let face_to_camera = -1.0 * vector_a;

                        let dot_product =
                            Vector3::dot_product(&face_normal, &face_to_camera);

                        dot_product < 0.0
                    } else {
                        false
                    };

                // Project
                if !culled {
                    // Lighting
                    // Note that lighting is currently applied *after* the view matrix transform, which means the
                    // "direction" of the light is always from the camera position.
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
                        let triangle = Triangle {
                            points: transformed_vertices.clone(),
                            texel_coordinates: mesh.get_texel_coordinates(face),
                            color,
                            ..Default::default()
                        };

                        let mut triangles =
                            clip_triangle(&frustum_planes, triangle);

                        project_triangles(
                            &projection_matrix,
                            window_width,
                            window_height,
                            &mut triangles,
                            &mut triangles_to_render,
                        );
                    }
                }
            }
        }

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
                    draw_textured_triangle(
                        &mut color_buffer,
                        &triangle,
                        &texture,
                    );
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
