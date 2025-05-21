use crate::{
    matrix::Matrix4,
    texture::Texture2,
    triangle::{get_sorted_triangle_vertices, Triangle},
    vector::{calc_cross_product, Vector2, Vector2i, Vector3, Vector4},
};

pub struct ColorBuffer {
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
        if y >= self.height || x >= self.width {
            return;
        }

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
    let point_0 = Vector2i::from_vector2(&triangle.points[0]);
    let point_1 = Vector2i::from_vector2(&triangle.points[1]);
    let point_2 = Vector2i::from_vector2(&triangle.points[2]);

    draw_rect(color_buffer, point_0.x, point_0.y, 5, 5, color);
    draw_rect(color_buffer, point_1.x, point_1.y, 5, 5, color);
    draw_rect(color_buffer, point_2.x, point_2.y, 5, 5, color);
}

pub fn draw_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    color: u32,
) {
    let point_0 = Vector2i::from_vector2(&triangle.points[0]);
    let point_1 = Vector2i::from_vector2(&triangle.points[1]);
    let point_2 = Vector2i::from_vector2(&triangle.points[2]);

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
    vertex0: &(Vector2, Texture2),
    vertex1: &(Vector2, Texture2),
    vertex2: &(Vector2, Texture2),
    texture: &[u32],
) {
    // Calculate Barycentric coordinates
    let (alpha, beta, gamma) = {
        let v0_point = Vector3::from_vector2(&vertex0.0);
        let v1_point = Vector3::from_vector2(&vertex1.0);
        let v2_point = Vector3::from_vector2(&vertex2.0);

        let v0_v1 = &v1_point - &v0_point;
        let v0_v2 = &v2_point - &v0_point;

        let total_area = calc_cross_product(&v0_v1, &v0_v2);

        let alpha: f64 = {
            calc_cross_product(a, b);
        };
        let beta: f64 = {};

        let gamma = 1.0 - alpha - beta;

        (alpha, beta, gamma)
    };
}

pub fn draw_textured_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    texture: &[u32],
) {
    // Find triangle vertex order
    let (vertex0, vertex1, vertex2) = get_sorted_triangle_vertices(triangle);

    let x_0 = vertex0.0.x as i32;
    let y_0 = vertex0.0.y as i32;
    let x_1 = vertex1.0.x as i32;
    let y_1 = vertex1.0.y as i32;
    let x_2 = vertex2.0.x as i32;
    let y_2 = vertex2.0.y as i32;

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

        let y_start = y_0;
        let y_end = y_1;

        for current_y in y_start..=y_end {
            let x_start = ((current_y - y_start) as f32 * inv_slope_1
                + x_0 as f32) as i32;
            let x_end = ((current_y - y_start) as f32 * inv_slope_2
                + x_0 as f32) as i32;

            let (x_start, x_end) = if x_end < x_start {
                (x_end, x_start)
            } else {
                (x_start, x_end)
            };

            for x in x_start..x_end {
                // color_buffer.set_pixel(
                //     x as usize,
                //     current_y as usize,
                //     0xFFFF00FF,
                // );
                draw_texel(
                    color_buffer,
                    x,
                    current_y,
                    vertex0,
                    vertex1,
                    vertex2,
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

        let y_start = vertex1.0.y as i32;
        let y_end = vertex2.0.y as i32;

        for current_y in y_start..=y_end {
            let x_start =
                ((current_y - y_0) as f32 * inv_slope_0 + x_0 as f32) as i32;
            let x_end =
                ((current_y - y_1) as f32 * inv_slope_1 + x_1 as f32) as i32;

            let (x_start, x_end) = if x_end < x_start {
                (x_end, x_start)
            } else {
                (x_start, x_end)
            };

            for x in x_start..x_end {
                // color_buffer.set_pixel(
                //     x as usize,
                //     current_y as usize,
                //     0xFF00FF00,
                // );
                draw_texel(
                    color_buffer,
                    x,
                    current_y,
                    vertex0,
                    vertex1,
                    vertex2,
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
