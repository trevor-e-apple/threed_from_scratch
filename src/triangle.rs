use crate::{texture::TextureUv, vector::Vector4};

#[derive(Clone, Default)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub a_uv: usize,
    pub b_uv: usize,
    pub c_uv: usize,
    pub color: u32,
}

#[derive(Clone, Default)]
pub struct Triangle {
    pub points: [Vector4; 3],
    pub texel_coordinates: [TextureUv; 3],
    pub color: u32,
    pub light_intensity: f32,
}

/// sorts the triangle vertex data by their y coordinates. Ascending order.
pub fn get_sorted_triangle_vertices(
    triangle: &Triangle,
) -> (
    (Vector4, TextureUv),
    (Vector4, TextureUv),
    (Vector4, TextureUv),
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
