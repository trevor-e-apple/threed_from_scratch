use crate::{
    matrix::Matrix4,
    texture::{Texture, TextureUv},
    triangle::{get_sorted_triangle_vertices, Triangle},
    vector::{Vector2, Vector2i, Vector4},
};

pub struct ColorBuffer {
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl ColorBuffer {
    /// Initializes a new instance of Self
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            zbuffer: vec![0.0; width * height],
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
        if y >= self.height || x >= self.width {
            return;
        }

        let pixel = self.buffer.get_mut(y * self.width + x).unwrap();
        *pixel = color;
    }

    pub fn set_pixel_zcell(&mut self, x: usize, y: usize, reciprocal_z: f32, color: u32) {
        if y >= self.height || x >= self.width {
            return;
        }

        let index = y * self.width + x;

        if self.zbuffer[index] < reciprocal_z {
            let pixel = self.buffer.get_mut(index).unwrap();
            *pixel = color;
            self.zbuffer[index] = reciprocal_z;
        }
    }

    /// Clears the color buffer to a specified color and clears zbuffer to 1.0.
    /// The zbuffer is cleared to 1.0 because the z coordinates are normalized
    /// and stored as a reciprocal.
    pub fn clear(&mut self, color: u32) {
        for index in 0..(self.width * self.height) {
            let pixel = self.buffer.get_mut(index).unwrap();
            *pixel = color;
            let zcell = self.zbuffer.get_mut(index).unwrap();
            *zcell = 0.0;
        }
    }
}

/// Draws a grid on the color buffer at every 'step' pixels
pub fn draw_dot_grid(color_buffer: &mut ColorBuffer, step: usize, color: u32) {
    // draw horizontal lines
    for y in (0..color_buffer.height).step_by(step) {
        for x in (0..color_buffer.width).step_by(step) {
            color_buffer.set_pixel(x, y, color);
        }
    }
}

pub fn draw_rect(
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

pub fn draw_line(
    color_buffer: &mut ColorBuffer,
    x_1: i32,
    y_1: i32,
    x_2: i32,
    y_2: i32,
    color: u32,
) {
    let delta_x = x_2 - x_1;
    let delta_y = y_2 - y_1;

    let side_length = {
        if delta_x.abs() > delta_y.abs() {
            delta_x.abs()
        } else {
            delta_y.abs()
        }
    };

    let x_increment = delta_x as f64 / side_length as f64;
    let y_increment = delta_y as f64 / side_length as f64;

    let mut x = x_1 as f64;
    let mut y = y_1 as f64;
    for _ in 0..side_length {
        x += x_increment;
        y += y_increment;

        color_buffer.set_pixel(x as usize, y as usize, color);
    }
}

pub fn draw_triangle_vertices(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    color: u32,
) {
    let point_0 = Vector2i::from_vector4(&triangle.points[0]);
    let point_1 = Vector2i::from_vector4(&triangle.points[1]);
    let point_2 = Vector2i::from_vector4(&triangle.points[2]);

    draw_rect(color_buffer, point_0.x, point_0.y, 5, 5, color);
    draw_rect(color_buffer, point_1.x, point_1.y, 5, 5, color);
    draw_rect(color_buffer, point_2.x, point_2.y, 5, 5, color);
}

pub fn draw_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    color: u32,
) {
    let point_0 = Vector2i::from_vector4(&triangle.points[0]);
    let point_1 = Vector2i::from_vector4(&triangle.points[1]);
    let point_2 = Vector2i::from_vector4(&triangle.points[2]);

    // draw lines of triangle
    draw_line(
        color_buffer,
        point_0.x,
        point_0.y,
        point_1.x,
        point_1.y,
        color,
    );
    draw_line(
        color_buffer,
        point_1.x,
        point_1.y,
        point_2.x,
        point_2.y,
        color,
    );
    draw_line(
        color_buffer,
        point_2.x,
        point_2.y,
        point_0.x,
        point_0.y,
        color,
    );
}

///////////////////////////////////////////////////////////////////////////////
// Draw a filled a triangle with a flat bottom
///////////////////////////////////////////////////////////////////////////////
//
//        (x0,y0)
//          / \
//         /   \
//        /     \
//       /       \
//      /         \
//  (x1,y1)------(x2,y2)
//
///////////////////////////////////////////////////////////////////////////////
fn draw_flat_bottom_triangle(
    color_buffer: &mut ColorBuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: u32,
) {
    let (y_start, y_end) = (y0, y1);

    // TODO: we might need to check for very narrow triangles that are just a line from the perspective of the camera
    let inv_slope_1 = ((x1 - x0) as f32) / ((y1 - y0) as f32);
    let inv_slope_2 = (x2 - x0) as f32 / ((y2 - y0) as f32);

    let mut x_start_f = x0 as f32;
    let mut x_end_f = x0 as f32;
    for y in y_start..=y_end {
        let x_start = x_start_f as i32;
        let x_end = x_end_f as i32;

        draw_line(color_buffer, x_start, y, x_end, y, color);

        x_start_f += inv_slope_1;
        x_end_f += inv_slope_2;
    }
}

///////////////////////////////////////////////////////////////////////////////
// Draw a filled a triangle with a flat top
///////////////////////////////////////////////////////////////////////////////
//
//  (x0,y0)------(x1,y1)
//      \         /
//       \       /
//        \     /
//         \   /
//          \ /
//        (x2,y2)
//
///////////////////////////////////////////////////////////////////////////////
fn draw_flat_top_triangle(
    color_buffer: &mut ColorBuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: u32,
) {
    let inv_slope_1 = ((x0 - x2) as f32) / ((y0 - y2) as f32);
    let inv_slope_2 = ((x1 - x2) as f32) / ((y1 - y2) as f32);

    let mut x_start_f = x2 as f32;
    let mut x_end_f = x2 as f32;
    for y in (y1..=y2).rev() {
        let x_start = x_start_f as i32;
        let x_end = x_end_f as i32;

        draw_line(color_buffer, x_start, y, x_end, y, color);
        // y is decreasing, so we subtract
        x_start_f -= inv_slope_1;
        x_end_f -= inv_slope_2;
    }
}

pub fn draw_filled_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    color: u32,
) {
    let (vertex0, vertex1, vertex2) = get_sorted_triangle_vertices(triangle);
    let point0 = vertex0.0;
    let point1 = vertex1.0;
    let point2 = vertex2.0;

    // this flow control avoids division by 0 in a somewhat elegant manner
    if point1.y == point2.y {
        draw_flat_bottom_triangle(
            color_buffer,
            point0.x as i32,
            point0.y as i32,
            point1.x as i32,
            point1.y as i32,
            point2.x as i32,
            point2.y as i32,
            color,
        );
    } else if point0.y == point1.y {
        draw_flat_top_triangle(
            color_buffer,
            point0.x as i32,
            point0.y as i32,
            point1.x as i32,
            point1.y as i32,
            point2.x as i32,
            point2.y as i32,
            color,
        );
    } else {
        // calculate the mid points of the triangle
        let midpoint_ = {
            let x = (((point2.x - point0.x) * (point1.y - point0.y))
                / (point2.y - point0.y))
                + point0.x;
            Vector2 { x, y: point1.y }
        };
        let midpoint = &midpoint_;

        draw_flat_bottom_triangle(
            color_buffer,
            point0.x as i32,
            point0.y as i32,
            point1.x as i32,
            point1.y as i32,
            midpoint.x as i32,
            midpoint.y as i32,
            color,
        );
        draw_flat_top_triangle(
            color_buffer,
            point1.x as i32,
            point1.y as i32,
            midpoint.x as i32,
            midpoint.y as i32,
            point2.x as i32,
            point2.y as i32,
            color,
        );
    }
}

fn draw_texel(
    color_buffer: &mut ColorBuffer,
    x: i32,
    y: i32,
    vertex0: &(Vector4, TextureUv),
    vertex1: &(Vector4, TextureUv),
    vertex2: &(Vector4, TextureUv),
    texture: &Texture,
) {
    // Calculate Barycentric coordinates
    //
    //         (B)
    //         /|\
    //        / | \
    //       /  |  \
    //      /  (P)  \
    //     /  /   \  \
    //    / /       \ \
    //   //           \\
    //  (A)------------(C)
    //
    let (alpha, beta, gamma) = {
        let a = Vector2::from_vector4(&vertex0.0);
        let b = Vector2::from_vector4(&vertex1.0);
        let c = Vector2::from_vector4(&vertex2.0);
        let p = Vector2 {
            x: x as f32,
            y: y as f32,
        };

        let ab = &b - &a;
        let ac = &c - &a;
        let ap = &p - &a;
        let pc = &c - &p;
        let pb = &b - &p;

        let abc_parallelagram_area = ac.x * ab.y - ac.y * ab.x; // || AC x AB ||

        // Area of PBC divided by the entire area
        let alpha = (pc.x * pb.y - pc.y * pb.x) / abc_parallelagram_area;

        // Area of APC divided by the entire area
        let beta = (ac.x * ap.y - ac.y * ap.x) / abc_parallelagram_area;

        // Gamma is the remaining part of the ratio
        let gamma = 1.0 - alpha - beta;

        (alpha, beta, gamma)
    };

    if alpha < 0.0 || beta < 0.0 || gamma < 0.0 {
        return;
    }

    // Interpolate UV values
    let (interpolated_u, interpolated_v, interpolated_reciprocal_w) = {
        let uv0 = vertex0.1.clone();
        let uv1 = vertex1.1.clone();
        let uv2 = vertex2.1.clone();

        let reciprocal_w_0 = 1.0 / vertex0.0.w;
        let reciprocal_w_1 = 1.0 / vertex1.0.w;
        let reciprocal_w_2 = 1.0 / vertex2.0.w;

        // interpolate over the reciprocal of w (our Z-value prior to projection)
        let interpolated_u = uv0.u * reciprocal_w_0 * alpha
            + uv1.u * reciprocal_w_1 * beta
            + uv2.u * reciprocal_w_2 * gamma;
        let interpolated_v = uv0.v * reciprocal_w_0 * alpha
            + uv1.v * reciprocal_w_1 * beta
            + uv2.v * reciprocal_w_2 * gamma;

        let interpolated_reciprocal_w = reciprocal_w_0 * alpha
            + reciprocal_w_1 * beta
            + reciprocal_w_2 * gamma;

        // Undo reciprocal
        let interpolated_u = interpolated_u / interpolated_reciprocal_w;
        let interpolated_v = interpolated_v / interpolated_reciprocal_w;

        (interpolated_u, interpolated_v, interpolated_reciprocal_w)
    };

    // Map UV value to texture coordinates
    let texture_x =
        ((texture.width - 1) as f32 * interpolated_u).abs() as usize;
    let texture_y =
        ((texture.height - 1) as f32 * interpolated_v).abs() as usize;

    // Draw pixel
    let pixel_color = texture.get_pixel(texture_x, texture_y);
    color_buffer.set_pixel_zcell(x as usize, y as usize, interpolated_reciprocal_w, pixel_color);
}

pub fn draw_textured_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    texture: &Texture,
) {
    // Find triangle vertex order
    let (vertex0, vertex1, vertex2) = get_sorted_triangle_vertices(triangle);

    let x_0 = vertex0.0.x as i32;
    let y_0 = vertex0.0.y as i32;
    let x_1 = vertex1.0.x as i32;
    let y_1 = vertex1.0.y as i32;
    let x_2 = vertex2.0.x as i32;
    let y_2 = vertex2.0.y as i32;

    let vertex0 = (
        Vector4 {
            x: x_0 as f32,
            y: y_0 as f32,
            z: vertex0.0.z,
            w: vertex0.0.w,
        },
        vertex0.1,
    );
    let vertex1 = (
        Vector4 {
            x: x_1 as f32,
            y: y_1 as f32,
            z: vertex1.0.z,
            w: vertex1.0.w,
        },
        vertex1.1,
    );
    let vertex2 = (
        Vector4 {
            x: x_2 as f32,
            y: y_2 as f32,
            z: vertex2.0.z,
            w: vertex2.0.w,
        },
        vertex2.1,
    );

    // Fill flat bottom triangle (y0 to y1)
    if (y_1 - y_0) != 0 {
        // Find inverse slopes (delta-x over delta-y)
        let (inv_slope_1, inv_slope_2) = {
            let inv_slope_1 = {
                let denom = (y_1 - y_0).abs();
                if denom == 0 {
                    0.0
                } else {
                    (x_1 - x_0) as f32 / (denom as f32)
                }
            };
            let inv_slope_2 = {
                let denom = (y_2 - y_0).abs();
                if denom == 0 {
                    0.0
                } else {
                    (x_2 - x_0) as f32 / (denom as f32)
                }
            };

            (inv_slope_1, inv_slope_2)
        };

        for current_y in y_0..=y_1 {
            let x_start =
                (x_1 as f32 + ((current_y - y_1) as f32 * inv_slope_1)) as i32;
            let x_end =
                (x_0 as f32 + ((current_y - y_0) as f32 * inv_slope_2)) as i32;

            let (x_start, x_end) = if x_end < x_start {
                (x_end, x_start)
            } else {
                (x_start, x_end)
            };

            for x in x_start..=x_end {
                draw_texel(
                    color_buffer,
                    x,
                    current_y,
                    &vertex0,
                    &vertex1,
                    &vertex2,
                    texture,
                );
            }
        }
    }

    // Fill flat top triangle (y1 to y2)
    if (y_2 - y_1) != 0 {
        // Find inverse slopes (delta-x over delta-y)
        let (inv_slope_0, inv_slope_1) = {
            let inv_slope_0 = {
                let denom = (y_2 - y_0).abs();
                if denom == 0 {
                    0.0
                } else {
                    (x_2 - x_0) as f32 / denom as f32
                }
            };
            let inv_slope_1 = {
                let denom = (y_2 - y_1).abs();
                if denom == 0 {
                    0.0
                } else {
                    (x_2 - x_1) as f32 / denom as f32
                }
            };

            (inv_slope_0, inv_slope_1)
        };

        for current_y in y_1..=y_2 {
            let x_start =
                ((current_y - y_0) as f32 * inv_slope_0 + x_0 as f32) as i32;
            let x_end =
                ((current_y - y_1) as f32 * inv_slope_1 + x_1 as f32) as i32;

            let (x_start, x_end) = if x_end < x_start {
                (x_end, x_start)
            } else {
                (x_start, x_end)
            };

            for x in x_start..=x_end {
                draw_texel(
                    color_buffer,
                    x,
                    current_y,
                    &vertex0,
                    &vertex1,
                    &vertex2,
                    texture,
                );
            }
        }
    }
}

pub fn perspective_projection(
    projection_matrix: &Matrix4,
    vector: &Vector4,
) -> Option<Vector4> {
    if vector.z != 0.0 {
        let mut result = Matrix4::mult_vector(projection_matrix, vector);
        result.x /= result.w;
        result.y /= result.w;
        result.z /= result.w;
        Some(result)
    } else {
        None
    }
}
