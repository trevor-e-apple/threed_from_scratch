use std::{mem::size_of, todo};

use sdl2::{render::Canvas, video::Window};

use crate::{
    color::Color,
    color_buffer::ColorBuffer,
    texture::{self, Tex2},
    triangle::{barycentric_weights, get_split_triangle_point, Triangle},
    vector2::{Vec2, Vec2i},
    vector4::Vec4, z_buffer::ZBuffer,
};

pub fn draw_grid(
    color_buffer: &mut ColorBuffer,
    width_interval: usize,
    height_interval: usize,
    color: Color,
) {
    for y in 0..color_buffer.height {
        for x in 0..color_buffer.width {
            if (x % width_interval == 0) && (y % height_interval == 0) {
                let pixel = match color_buffer.get_mut(x, y) {
                    Some(value) => value,
                    None => todo!(),
                };
                *pixel = color;
            }
        }
    }
}

pub fn draw_rect(
    color_buffer: &mut ColorBuffer,
    top_left_x: i32,
    top_left_y: i32,
    width: i32,
    height: i32,
    color: Color,
) {
    // calculate start / stop x coordinates
    let (start_x, end_x) = {
        let mut start_x = top_left_x;
        let buffer_width = color_buffer.width as i32;

        if start_x < 0 {
            start_x = 0;
        } else if start_x > buffer_width {
            start_x = buffer_width;
        }

        let mut end_x = top_left_x + width;
        if end_x < 0 {
            end_x = 0;
        } else if end_x > buffer_width {
            end_x = buffer_width;
        }

        (start_x, end_x)
    };

    // calculate start / stop y coordinates
    let (start_y, end_y) = {
        let mut start_y = top_left_y;
        let buffer_height = color_buffer.height as i32;

        if start_y < 0 {
            start_y = 0;
        } else if start_y > buffer_height {
            start_y = buffer_height;
        }

        let mut end_y = top_left_y + height;
        if end_y < 0 {
            end_y = 0;
        } else if end_y > buffer_height {
            end_y = buffer_height;
        }

        (start_y, end_y)
    };

    for y in start_y..end_y {
        for x in start_x..end_x {
            let pixel = match color_buffer.get_mut(x as usize, y as usize) {
                Some(value) => value,
                None => todo!(),
            };
            *pixel = color;
        }
    }
}

pub fn draw_pixel(
    color_buffer: &mut ColorBuffer,
    x: i32,
    y: i32,
    color: Color,
) {
    if let Some(pixel) = color_buffer.get_mut(x as usize, y as usize) {
        *pixel = color;
    }
}

pub fn draw_line(
    color_buffer: &mut ColorBuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    let delta_x = x1 - x0;
    let delta_y = y1 - y0;
    let longest_side_length = if delta_x.abs() > delta_y.abs() {
        delta_x.abs()
    } else {
        delta_y.abs()
    };
    let x_inc = delta_x as f32 / longest_side_length as f32;
    let y_inc = delta_y as f32 / longest_side_length as f32;

    let mut current_x = x0 as f32;
    let mut current_y = y0 as f32;
    for _ in 0..=longest_side_length {
        draw_pixel(
            color_buffer,
            current_x.round() as i32,
            current_y.round() as i32,
            color,
        );
        current_x += x_inc;
        current_y += y_inc;
    }
}

pub fn draw_triangle_with_coordinates(
    color_buffer: &mut ColorBuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: Color,
) {
    draw_line(color_buffer, x0, y0, x1, y1, color);
    draw_line(color_buffer, x0, y0, x2, y2, color);
    draw_line(color_buffer, x1, y1, x2, y2, color);
}

pub fn draw_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    color: Color,
) {
    draw_triangle_with_coordinates(
        color_buffer,
        triangle.points[0].x as i32,
        triangle.points[0].y as i32,
        triangle.points[1].x as i32,
        triangle.points[1].y as i32,
        triangle.points[2].x as i32,
        triangle.points[2].y as i32,
        color,
    );
}

fn draw_texel(
    color_buffer: &mut ColorBuffer,
    z_buffer: &mut ZBuffer,
    texture: &texture::Texture,
    x: i32,
    y: i32,
    a: Vec4,
    a_uv: Tex2,
    b: Vec4,
    b_uv: Tex2,
    c: Vec4,
    c_uv: Tex2,
) {
    let weights = barycentric_weights(
        Vec2::from_vec4(&a),
        Vec2::from_vec4(&b),
        Vec2::from_vec4(&c),
        Vec2 {
            x: x as f32,
            y: y as f32,
        },
    );

    let alpha = weights.x;
    let beta = weights.y;
    let gamma = weights.z;

    let interpolated_reciprocal_w =
        (1.0 / a.w) * alpha + (1.0 / b.w) * beta + (1.0 / c.w) * gamma;
    let interpolated_u = ((a_uv.u / a.w) * alpha
        + (b_uv.u / b.w) * beta
        + (c_uv.u / c.w) * gamma)
        / interpolated_reciprocal_w;
    let interpolated_v = ((a_uv.v / a.w) * alpha
        + (b_uv.v / b.w) * beta
        + (c_uv.v / c.w) * gamma)
        / interpolated_reciprocal_w;

    let tex_x = (interpolated_u * texture.width as f32).abs() as usize;
    let tex_y = (interpolated_v * texture.height as f32).abs() as usize;

    let color = match texture.get_pixel(tex_x, tex_y) {
        Some(color) => color,
        None => return,
    };


    // adjust 1/w values so that pixels closer to the camera have smaller values
    let z_buffer_test = 1.0 - interpolated_reciprocal_w;
    // only draw if the depth is less than the depth stored in the z_buffer
    if z_buffer_test < z_buffer.get_pixel_value(x as usize, y as usize) {
        draw_pixel(color_buffer, x, y, color);
        // update the z-buffer with the 1/w of this pixel
        z_buffer.set_pixel_value(x as usize, y as usize, z_buffer_test);
    }
}

pub fn draw_textured_triangle(
    color_buffer: &mut ColorBuffer,
    z_buffer: &mut ZBuffer,
    triangle: &Triangle,
    texture: &texture::Texture,
) {
    let (sorted_points, uv_points, ray_intersection) =
        get_split_triangle_point(triangle);

    // Vec2 points needed for texel draw
    let a = sorted_points[0];
    let a_uv = uv_points[0];
    let b = sorted_points[1];
    let b_uv = uv_points[1];
    let c = sorted_points[2];
    let c_uv = uv_points[2];

    // Vec2i needed for scanline fill
    let top = Vec2i::from_vec4_floor(&a);
    let middle = Vec2i::from_vec4_floor(&b);
    let bottom = Vec2i::from_vec4_floor(&c);
    let ray_intersection = Vec2i::from_vec2_floor(&ray_intersection);

    // draw the top filled triangle (flat bottom)
    {
        let top_y = top.y;
        let bottom_y = ray_intersection.y;
        if top_y != bottom_y {
            // find the change in x for each y pixel (top to bottom)
            let x_per_y_1 =
                (middle.x - top.x) as f32 / (middle.y - top.y) as f32;
            let x_per_y_2 = (ray_intersection.x - top.x) as f32
                / (ray_intersection.y - top.y) as f32;

            let mut x_bound_one = top.x as f32;
            let mut x_bound_two = top.x as f32;

            if x_per_y_1 < x_per_y_2 {
                for y in top_y..=bottom_y {
                    for x in (x_bound_one as i32)..=(x_bound_two as i32) {
                        draw_texel(
                            color_buffer,
                            z_buffer,
                            texture,
                            x,
                            y,
                            a,
                            a_uv,
                            b,
                            b_uv,
                            c,
                            c_uv,
                        );
                    }
                    x_bound_one += x_per_y_1;
                    x_bound_two += x_per_y_2;
                }
            } else {
                for y in top_y..=bottom_y {
                    for x in (x_bound_two as i32)..=(x_bound_one as i32) {
                        draw_texel(
                            color_buffer,
                            z_buffer,
                            texture,
                            x,
                            y,
                            a,
                            a_uv,
                            b,
                            b_uv,
                            c,
                            c_uv,
                        );
                    }
                    x_bound_one += x_per_y_1;
                    x_bound_two += x_per_y_2;
                }
            }
        }
    }

    // draw the bottom filled triangle (flat top)
    {
        let top_y = ray_intersection.y as i32;
        let bottom_y = bottom.y as i32;
        if top_y != bottom_y {
            let x_per_y_1 =
                (bottom.x - middle.x) as f32 / (bottom.y - middle.y) as f32;
            let x_per_y_2 = (bottom.x - ray_intersection.x) as f32
                / (bottom.y - ray_intersection.y) as f32;

            if middle.x < ray_intersection.x {
                let mut x_start = middle.x as f32;
                let mut x_end = ray_intersection.x as f32;

                for y in top_y..=bottom_y {
                    for x in (x_start as i32)..=(x_end as i32) {
                        draw_texel(
                            color_buffer,
                            z_buffer,
                            texture,
                            x,
                            y,
                            a,
                            a_uv,
                            b,
                            b_uv,
                            c,
                            c_uv,
                        );
                    }
                    x_start += x_per_y_1;
                    x_end += x_per_y_2;
                }
            } else {
                let mut x_start = ray_intersection.x as f32;
                let mut x_end = middle.x as f32;

                for y in top_y..=bottom_y {
                    for x in (x_start as i32)..=(x_end as i32) {
                        draw_texel(
                            color_buffer,
                            z_buffer,
                            texture,
                            x,
                            y,
                            a,
                            a_uv,
                            b,
                            b_uv,
                            c,
                            c_uv,
                        );
                    }
                    x_start += x_per_y_2;
                    x_end += x_per_y_1;
                }
            }
        }
    }
}

pub fn draw_filled_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    color: Color,
) {
    let (sorted_points, _, ray_intersection) =
        get_split_triangle_point(triangle);

    let top = Vec2i::from_vec4_floor(&sorted_points[0]);
    let middle = Vec2i::from_vec4_floor(&sorted_points[1]);
    let bottom = Vec2i::from_vec4_floor(&sorted_points[2]);
    let ray_intersection = Vec2i::from_vec2_floor(&ray_intersection);

    // draw the top filled triangle (flat bottom)
    {
        let top_y = top.y;
        let bottom_y = ray_intersection.y;
        if top_y != bottom_y {
            // find the change in x for each y pixel (top to bottom)
            let x_per_y_1 =
                (middle.x - top.x) as f32 / (middle.y - top.y) as f32;
            let x_per_y_2 = (ray_intersection.x - top.x) as f32
                / (ray_intersection.y - top.y) as f32;

            let mut x_start = top.x as f32;
            let mut x_end = top.x as f32;
            for y in top_y..=bottom_y {
                draw_line(
                    color_buffer,
                    x_start as i32,
                    y,
                    x_end as i32,
                    y,
                    color,
                );
                x_start += x_per_y_1;
                x_end += x_per_y_2;
            }
        }
    }

    // draw the bottom filled triangle (flat top)
    {
        let top_y = ray_intersection.y as i32;
        let bottom_y = bottom.y as i32;
        if top_y != bottom_y {
            let x_per_y_1 =
                (bottom.x - middle.x) as f32 / (bottom.y - middle.y) as f32;
            let x_per_y_2 = (bottom.x - ray_intersection.x) as f32
                / (bottom.y - ray_intersection.y) as f32;

            let mut x_start = middle.x as f32;
            let mut x_end = ray_intersection.x as f32;
            for y in top_y..=bottom_y {
                draw_line(
                    color_buffer,
                    x_start as i32,
                    y,
                    x_end as i32,
                    y,
                    color,
                );
                x_start += x_per_y_1;
                x_end += x_per_y_2;
            }
        }
    }
}

/// Renders a color buffer to the screen
/// color_buffer: the color_buffer that contains the render data
/// canvas: the sdl canvas to draw to
/// texture: the texture to copy the color buffer to
pub fn render(
    color_buffer: &mut ColorBuffer,
    canvas: &mut Canvas<Window>,
    texture: &mut sdl2::render::Texture,
) -> bool {
    let window_width = color_buffer.width;

    // TODO: Error handling
    match texture.update(
        None,
        unsafe { color_buffer.get_raw_data() },
        (window_width as usize) * size_of::<u32>(),
    ) {
        Err(err) => {
            println!("Texture update failed: {:?}", err);
            return false;
        }
        _ => {}
    };

    // TODO: error handling
    match canvas.copy(&texture, None, None) {
        Err(err) => {
            println!("Canvas copy failed: {:?}", err);
            return false;
        }
        _ => {}
    };

    canvas.present();

    true
}
