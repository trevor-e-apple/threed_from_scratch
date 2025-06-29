use crate::{triangle::Triangle, vector::Vector3};

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

pub fn clip_triangle(
    frustum_planes: &FrustumPlanes,
    triangle: Triangle,
) -> Vec<Triangle> {
    // Find the resulting polygon from the clipping
    {}

    // Convert the polygon into triangles
    {
        // Pick a vertex in the triangle
    }

    todo!()
}
