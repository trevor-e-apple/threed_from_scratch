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

        color_buffer.set_pixel(x.round() as usize, y.round() as usize, color);
    }
}

pub fn draw_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    offset_vector: &Vector2,
) {
    let point_0 =
        Vector2i::from_vector2(&(&triangle.points[0] + offset_vector));
    let point_1 =
        Vector2i::from_vector2(&(&triangle.points[1] + offset_vector));
    let point_2 =
        Vector2i::from_vector2(&(&triangle.points[2] + offset_vector));

    // draw lines of triangle
    const LINE_COLOR: u32 = 0xFFFFFFFF;
    draw_line(
        color_buffer,
        point_0.x,
        point_0.y,
        point_1.x,
        point_1.y,
        LINE_COLOR,
    );
    draw_line(
        color_buffer,
        point_1.x,
        point_1.y,
        point_2.x,
        point_2.y,
        LINE_COLOR,
    );
    draw_line(
        color_buffer,
        point_2.x,
        point_2.y,
        point_0.x,
        point_0.y,
        LINE_COLOR,
    );

    // draw vertices
    draw_rect(
        color_buffer,
        point_0.x as i32,
        point_0.y as i32,
        4,
        4,
        0xFFFFFF00,
    );
    draw_rect(color_buffer, point_1.x, point_1.y, 4, 4, 0xFFFFFF00);
    draw_rect(color_buffer, point_2.x, point_2.y, 4, 4, 0xFFFFFF00);
}

pub fn draw_filled_triangle(
    color_buffer: &mut ColorBuffer,
    triangle: &Triangle,
    offset_vector: &Vector2,
    color: u32,
) {
    // need to sort the vertices by ascending (y0 < y1 < y2)
    let (point0, point1, point2) = {
        let point0 = &triangle.points[0];
        let point1 = &triangle.points[1];
        let point2 = &triangle.points[2];

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

    // calculate the mid points of the triangle
    let midpoint_ = {
        let x = (((point2.x - point0.x) * (point1.y - point0.y))
            / (point2.y - point0.y))
            + point0.x;
        Vector2 { x, y: point1.y }
    };
    let midpoint = &midpoint_;

    // Fill flat bottom triangle
    {
        let (y_start, y_end) =
            (point0.y.round() as i32, midpoint.y.round() as i32);

        // TODO: we might need to check for very narrow triangles that are just a line from the perspective of the camera
        let point1_slope = (point1.x - point0.x) / (point1.y - point0.y);
        let midpoint_slope = (midpoint.x - point0.x) / (midpoint.y - point0.y);

        let (inv_start_slope, inv_end_slope) = if point1.x <= point0.x {
            (point1_slope, midpoint_slope)
        } else {
            (midpoint_slope, point1_slope)
        };

        let mut x_start_f = point0.x;
        let mut x_end_f = point0.x;
        for y in y_start..=y_end {
            let x_start = x_start_f.round() as i32;
            let x_end = x_end_f.round() as i32;
            for x in x_start..=x_end {
                color_buffer.set_pixel(x as usize, y as usize, color);
            }

            x_start_f += inv_start_slope;
            x_end_f += inv_end_slope;
        }
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
