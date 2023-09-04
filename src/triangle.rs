use crate::{color::Color, texture::Tex2, vector2::Vec2};

#[derive(Default, Clone)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub a_uv: Tex2,
    pub b_uv: Tex2,
    pub c_uv: Tex2,
    pub color: Color,
}

#[derive(Default, Clone)]
pub struct Triangle {
    pub points: [Vec2; 3],
    pub tex_coordinates: [Tex2; 3],
    pub color: Color,
    pub avg_depth: f32,
}

/// considers the triangle to have a "top", "middle", and "bottom" vertex
/// returns the sorted vertices and the intersection point for a horizontal ray
/// cast from the "middle" vertex
pub fn get_split_triangle_point(triangle: &Triangle) -> ([Vec2; 3], Vec2) {
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

    let x =
        ((bottom.x - top.x) * (middle.y - top.y)) / (bottom.y - top.y) + top.x;
    let split_point = Vec2 { x, y: middle.y };

    (points, split_point)
}
