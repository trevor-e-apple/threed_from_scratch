use crate::{
    texture::TextureUv,
    triangle::Triangle,
    vector::{Vector3, Vector4},
};

pub struct Plane {
    pub position: Vector3,
    pub normal: Vector3,
}

pub struct FrustumPlanes {
    near_plane: Plane,
    far_plane: Plane,
    top_plane: Plane,
    bottom_plane: Plane,
    left_plane: Plane,
    right_plane: Plane,
}

#[derive(Clone, Default)]
struct Polygon {
    vertices: Vec<(Vector3, TextureUv)>,
}

impl Polygon {
    fn len(&self) -> usize {
        self.vertices.len()
    }
}

impl FrustumPlanes {
    /// The normals for the planes should point to the inside of the frustum
    pub fn new(znear: f32, zfar: f32, fov_x: f32, fov_y: f32) -> Self {
        // let half_fov = fov / 2.0;
        let cos_x = (fov_x / 2.0).cos();
        let sin_x = (fov_x / 2.0).sin();

        let cos_y = (fov_y / 2.0).cos();
        let sin_y = (fov_y / 2.0).sin();

        let near_plane = Plane {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: znear,
            },
            normal: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let far_plane = Plane {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: zfar,
            },
            normal: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        };
        let left_plane = Plane {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector3 {
                x: cos_x,
                y: 0.0,
                z: sin_x,
            },
        };
        let right_plane = Plane {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector3 {
                x: -1.0 * cos_x,
                y: 0.0,
                z: sin_x,
            },
        };
        let top_plane = Plane {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: -1.0 * cos_y,
                z: sin_y,
            },
        };
        let bottom_plane = Plane {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: cos_y,
                z: sin_y,
            },
        };
        Self {
            near_plane,
            far_plane,
            top_plane,
            bottom_plane,
            left_plane,
            right_plane,
        }
    }
}

fn clip_polygon_against_plane(polygon: &Polygon, plane: &Plane) -> Polygon {
    if polygon.len() < 2 {
        return polygon.clone();
    }

    // Vec for tracking vertices of the final polygon that are inside the frustum
    let mut inside_vertices = Vec::<(Vector3, TextureUv)>::with_capacity(10);

    // Track the current and previous vertex so that we know if we've crossed a frustum plane
    let mut current_vertex_index = 0;
    let mut previous_vertex_index = polygon.len() - 1;

    while current_vertex_index < polygon.len() {
        let (current_vertex, current_uv) =
            &polygon.vertices[current_vertex_index];
        let (previous_vertex, previous_uv) =
            &polygon.vertices[previous_vertex_index];

        let current_dot = Vector3::dot_product(
            &(current_vertex - &plane.position),
            &plane.normal,
        );
        let previous_dot = Vector3::dot_product(
            &(previous_vertex - &plane.position),
            &plane.normal,
        );

        // We have moved inside the frustum to the outside or vice versa
        if (current_dot * previous_dot) < 0.0 {
            // Find the interpolation factor
            let t = previous_dot / (previous_dot - current_dot);

            let intersection_point =
                previous_vertex + &(t * (current_vertex - previous_vertex));
            let intersection_uv = TextureUv {
                u: previous_uv.u + t * (current_uv.u - previous_uv.u),
                v: previous_uv.v + t * (current_uv.v - previous_uv.v),
            };

            // Insert the intersection point to the list of inside vertices
            inside_vertices.push((intersection_point, intersection_uv));
        }

        // This vertex is on the inside of the frustum
        if current_dot > 0.0 {
            inside_vertices.push((current_vertex.clone(), current_uv.clone()));
        }

        current_vertex_index += 1;
        previous_vertex_index = current_vertex_index - 1;
    }

    Polygon {
        vertices: inside_vertices,
    }
}

pub fn clip_triangle(
    frustum_planes: &FrustumPlanes,
    triangle: Triangle,
) -> Vec<Triangle> {
    // Find the resulting polygon from all of the clipping
    let polygon: Polygon = Polygon {
        vertices: vec![
            (
                Vector3::from_vector4(&triangle.points[0]),
                triangle.texel_coordinates[0].clone(),
            ),
            (
                Vector3::from_vector4(&triangle.points[1]),
                triangle.texel_coordinates[1].clone(),
            ),
            (
                Vector3::from_vector4(&triangle.points[2]),
                triangle.texel_coordinates[2].clone(),
            ),
        ],
    };

    let polygon =
        clip_polygon_against_plane(&polygon, &frustum_planes.left_plane);
    let polygon =
        clip_polygon_against_plane(&polygon, &frustum_planes.right_plane);
    let polygon =
        clip_polygon_against_plane(&polygon, &frustum_planes.top_plane);
    let polygon =
        clip_polygon_against_plane(&polygon, &frustum_planes.bottom_plane);
    let polygon =
        clip_polygon_against_plane(&polygon, &frustum_planes.near_plane);
    let polygon =
        clip_polygon_against_plane(&polygon, &frustum_planes.far_plane);

    // Transform polygon into triangles
    let triangles_after_clipping = {
        let mut triangles_after_clipping: Vec<Triangle> = vec![];

        if polygon.len() < 3 {
            triangles_after_clipping
        } else {
            for index in 0..polygon.len() - 2 {
                let index0 = 0;
                let index1 = index + 1;
                let index2 = index + 2;

                let (vertex0, uv0) = &polygon.vertices[index0];
                let (vertex1, uv1) = &polygon.vertices[index1];
                let (vertex2, uv2) = &polygon.vertices[index2];
                let new_triangle = Triangle {
                    points: [
                        Vector4::from_vector3(vertex0),
                        Vector4::from_vector3(vertex1),
                        Vector4::from_vector3(vertex2),
                    ],
                    texel_coordinates: [uv0.clone(), uv1.clone(), uv2.clone()],
                    color: triangle.color,
                    light_intensity: triangle.light_intensity,
                    texture_handle: triangle.texture_handle,
                };

                triangles_after_clipping.push(new_triangle);
            }

            triangles_after_clipping
        }
    };

    triangles_after_clipping
}
