use crate::{texture::Texture2, vector::Vector2};

#[derive(Clone, Default)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub a_uv: Texture2,
    pub b_uv: Texture2,
    pub c_uv: Texture2,
    pub color: u32,
}

#[derive(Clone, Default)]
pub struct Triangle {
    pub points: [Vector2; 3],
    pub texel_coordinates: [Texture2; 3],
    pub color: u32,
    pub avg_depth: f32,
}

/// sorts the triangle vertex data by their y coordinates. Ascending order.
pub fn get_sorted_triangle_vertices(
    triangle: &Triangle,
) -> (
    (Vector2, Texture2),
    (Vector2, Texture2),
    (Vector2, Texture2),
) {
    let vertex0 = (&triangle.points[0], &triangle.texel_coordinates[0]);
    let vertex1 = (&triangle.points[1], &triangle.texel_coordinates[1]);
    let vertex2 = (&triangle.points[2], &triangle.texel_coordinates[2]);

    let (vertex0, vertex1) = if vertex0.0.y > vertex1.0.y {
        (vertex1, vertex0)
    } else {
        (vertex0, vertex1)
    };
    let (vertex1, vertex2) = if vertex1.0.y > vertex2.0.y {
        (vertex2, vertex1)
    } else {
        (vertex1, vertex2)
    };
    let (vertex0, vertex1) = if vertex0.0.y > vertex1.0.y {
        (vertex1, vertex0)
    } else {
        (vertex0, vertex1)
    };

    (
        (vertex0.0.clone(), vertex0.1.clone()),
        (vertex1.0.clone(), vertex1.1.clone()),
        (vertex2.0.clone(), vertex2.1.clone()),
    )
}
