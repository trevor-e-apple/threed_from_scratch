use crate::{color::Color, texture::Tex2, vector2::Vec2, vector3::Vec3};

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
pub fn get_split_triangle_point(
    triangle: &Triangle,
) -> ([Vec2; 3], [Tex2; 3], Vec2) {
    // sort the points according to their y values
    // -- (ascending sort, but top-to-bottom)
    let mut points: [Vec2; 3] = triangle.points.clone();
    let mut uv_points: [Tex2; 3] = triangle.tex_coordinates.clone();

    loop {
        let mut sorted = true;
        for index in 0..(points.len() - 1) {
            let a = points[index];
            let b = points[index + 1];
            if a.y > b.y {
                points.swap(index, index + 1);
                uv_points.swap(index, index + 1);
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

    (points, uv_points, split_point)
}

pub fn barycentric_weights(a: Vec2, b: Vec2, c: Vec2, p: Vec2) -> Vec3 {
    let ac = c - a;
    let ab = b - a;
    let pc = c - p;
    let pb = b - p;
    let ap = p - a;

    let abc_parallelogram_area = ac.x * ab.y - ac.y * ab.x; // || AC x AB ||

    let pbc_parallelogram_area = pc.x * pb.y - pc.y * pb.x; // || PC x PB ||
    let alpha = pbc_parallelogram_area / abc_parallelogram_area;

    let apc_parallelogram_area = ac.x * ap.y - ac.y * ap.x;
    let beta = apc_parallelogram_area / abc_parallelogram_area;

    let gamma = 1.0 - alpha - beta;

    Vec3 {
        x: alpha,
        y: beta,
        z: gamma,
    }
}
