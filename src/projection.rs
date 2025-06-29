use crate::{matrix::Matrix4, triangle::Triangle, vector::Vector4};

fn perspective_projection(
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

pub fn project_triangles(
    projection_matrix: &Matrix4,
    window_width: u32,
    window_height: u32,
    triangles: &mut Vec<Triangle>,
    triangles_to_render: &mut Vec<Triangle>,
) {
    for triangle in triangles {
        for vertex in &mut triangle.points {
            match perspective_projection(&projection_matrix, vertex) {
                Some(projected_point) => {
                    let mut projected_point = projected_point.clone();
                    // perform windowing transform (scale then translate)
                    // the division by 2 is b/c we are mapping the canonical view volume (which has bounds x,y: [-1, 1]) to screen
                    // space (which has bounds x: [0, window_width], y: [0, window_height])
                    {
                        projected_point.x *= window_width as f32 / 2.0;
                        projected_point.y *= window_height as f32 / 2.0;

                        // since y grows down in screen space, but up in world space / canonical image space
                        projected_point.y *= -1.0;

                        projected_point.x += window_width as f32 / 2.0;
                        projected_point.y += window_height as f32 / 2.0;
                    }

                    *vertex = projected_point;
                }
                None => {}
            }
        }
        triangles_to_render.push(triangle.clone());
    }
}
