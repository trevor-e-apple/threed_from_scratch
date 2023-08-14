use std::todo;

use crate::vector2::{dot, normalize, Vec2};

#[derive(Clone)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[derive(Default, Clone)]
pub struct Triangle {
    pub points: [Vec2; 3],
}

/// considers the triangle to have a "top", "middle", and "bottom" vertex
/// returns the middle vertex and the intersection point for a horizontal ray
/// cast from the "middle" vertex
pub fn get_split_triangle_point(triangle: &Triangle) -> (Vec2, Vec2) {
    // sort the points according to their y values
    // -- (ascending sort, but top-to-bottom)
    let mut points: [Vec2; 3] = triangle.points.clone();
    loop {
        let mut sorted = true;
        for index in 0..(points.len() - 1) {
            let a = points[index];
            let b = points[index + 1];
            if a.y > b.y {
                points.swap(index, index + 1);
                sorted = false;
            }
        }

        if sorted {
            break;
        }
    }

    let top = points[0];
    let middle = points[1];
    let bottom = points[2];

    let split_point = Vec2 {
        x: ((bottom.x - top.x) * (middle.y - top.y)) / (bottom.y - top.y) + top.x,
        y: middle.y,
    };

    (middle, split_point)
}
