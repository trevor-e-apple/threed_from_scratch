use crate::{
    texture::TextureUv, triangle::Triangle, vector::{Vector3, Vector4}
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

impl FrustumPlanes {
    /// The normals for the planes should point to the inside of the frustum
    pub fn new(znear: f32, zfar: f32, fov: f32) -> Self {
        let half_fov = fov / 2.0;
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
                x: half_fov.cos(),
                y: 0.0,
                z: half_fov.sin(),
            },
        };
        let right_plane = Plane {
            position: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vector3 {
                x: -1.0 * half_fov.cos(),
                y: 0.0,
                z: half_fov.sin(),
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
                y: -1.0 * half_fov.cos(),
                z: half_fov.sin(),
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
                y: half_fov.cos(),
                z: half_fov.sin(),
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

fn clip_polygon_against_plane(
    polygon: &Vec<Vector3>,
    plane: &Plane,
) -> Vec<Vector3> {
    if polygon.len() == 0 {
        return vec![];
    } else if polygon.len() == 1 {
        return polygon.clone();
    }

    // Vec for tracking vertices of the final polygon that are inside the frustum
    let mut inside_vertices = Vec::<Vector3>::with_capacity(10);

    // Track the current and previous vertex so that we know if we've crossed a frustum plane
    let mut current_vertex_index = 0;
    let mut previous_vertex_index = polygon.len() - 1;

    while current_vertex_index < polygon.len() {
        let current_vertex = &polygon[current_vertex_index];
        let previous_vertex = &polygon[previous_vertex_index];

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

            // Insert the intersection point to the list of inside vertices
            inside_vertices.push(intersection_point);
        }

        // This vertex is on the inside of the frustum
        if current_dot > 0.0 {
            inside_vertices.push(current_vertex.clone());
        }

        current_vertex_index += 1;
        previous_vertex_index = current_vertex_index - 1;
    }

    inside_vertices
}

pub fn clip_triangle(
    frustum_planes: &FrustumPlanes,
    triangle: Triangle,
) -> Vec<Triangle> {
    // Find the resulting polygon from all of the clipping
    let polygon = vec![
        Vector3::from_vector4(&triangle.points[0]),
        Vector3::from_vector4(&triangle.points[1]),
        Vector3::from_vector4(&triangle.points[2]),
    ];

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

                let new_triangle = Triangle {
                    points: [
                        Vector4::from_vector3(&polygon[index0]),
                        Vector4::from_vector3(&polygon[index1]),
                        Vector4::from_vector3(&polygon[index2]),
                    ],
                    color: triangle.color,
                    ..Default::default() // TODO: handle UV coordinates
                };

                triangles_after_clipping.push(new_triangle);
            }

            triangles_after_clipping
        }
    };

    triangles_after_clipping
}
