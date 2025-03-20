use crate::vector::Vector3;

/// Function for generating a point cloud
fn gen_point_cloud(point_count: usize) -> Vec<Vector3> {
    // TODO: it may be neat to convert a mesh to a point cloud!
    let mut points = Vec::<Vector3>::with_capacity(point_count);
    let mut x: f32 = -1.0;
    while x <= 1.0 {
        let mut y: f32 = -1.0;
        while y <= 1.0 {
            let mut z: f32 = -1.0;
            while z <= 1.0 {
                points.push(Vector3 { x, y, z });
                z += 0.25;
            }
            y += 0.25;
        }
        x += 0.25;
    }
    points
}
