use crate::{
    triangle::Triangle,
    vector::{Vector2, Vector2i, Vector3},
};

const FOV_FACTOR: f32 = 640.0;

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
    offset_vector: &Vector2,
    color: u32,
) {
    let point_0 =
        Vector2i::from_vector2(&(&triangle.points[0] + offset_vector));
    let point_1 =
        Vector2i::from_vector2(&(&triangle.points[1] + offset_vector));
    let point_2 =
        Vector2i::from_vector2(&(&triangle.points[2] + offset_vector));

    draw_rect(color_buffer, point_0.x, point_0.y, 5, 5, color);
    draw_rect(color_buffer, point_1.x, point_1.y, 5, 5, color);
    draw_rect(color_buffer, point_2.x, point_2.y, 5, 5, color);
}

pub fn draw_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    offset_vector: &Vector2,
    color: u32,
) {
    let point_0 =
        Vector2i::from_vector2(&(&triangle.points[0] + offset_vector));
    let point_1 =
        Vector2i::from_vector2(&(&triangle.points[1] + offset_vector));
    let point_2 =
        Vector2i::from_vector2(&(&triangle.points[2] + offset_vector));

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
    offset_vector: &Vector2,
    color: u32,
) {
    // need to sort the vertices by ascending (y0 < y1 < y2)
    let (point0, point1, point2) = {
        let point0 = &triangle.points[0] + offset_vector;
        let point1 = &triangle.points[1] + offset_vector;
        let point2 = &triangle.points[2] + offset_vector;

        let (point0, point1) = if point0.y > point1.y {
            (point1, point0)
        } else {
            (point0, point1)
        };
        let (point1, point2) = if point1.y > point2.y {
            (point2, point1)
        } else {
            (point1, point2)
        };
        let (point0, point1) = if point0.y > point1.y {
            (point1, point0)
        } else {
            (point0, point1)
        };

        (point0, point1, point2)
    };

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

pub fn orthographic_projection(vector: &Vector3) -> Vector2 {
    Vector2 {
        x: FOV_FACTOR * vector.x,
        y: FOV_FACTOR * vector.y,
    }
}

pub fn perspective_projection(vector: &Vector3) -> Option<Vector2> {
    if vector.z != 0.0 {
        Some(Vector2 {
            x: (FOV_FACTOR * vector.x) / vector.z,
            y: (FOV_FACTOR * vector.y) / vector.z,
        })
    } else {
        None
    }
}
