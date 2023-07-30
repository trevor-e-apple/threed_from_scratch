#[derive(Default, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn rotate_vec2(vector: &Vec2, degrees: f32) -> Vec2 {
    let cos_degrees = degrees.cos();
    let sin_degrees = degrees.sin();
    Vec2 {
        x: vector.x * cos_degrees - vector.y * sin_degrees,
        y: vector.x * sin_degrees + vector.y * cos_degrees,
    }
}

/* For now, our vec3 rotations will just be around one  of the axes */
fn x_axis_rotate(vector: &Vec3, degrees: f32) -> Vec3 {
    let cos_degrees = degrees.cos();
    let sin_degrees = degrees.sin();
    Vec3 {
        x: vector.x,
        y: vector.y * cos_degrees - vector.z * sin_degrees,
        z: vector.y * sin_degrees + vector.z * cos_degrees,
    }
}

fn y_axis_rotate(vector: &Vec3, degrees: f32) -> Vec3 {
    let cos_degrees = degrees.cos();
    let sin_degrees = degrees.sin();
    Vec3 {
        x: vector.x * cos_degrees - vector.z * sin_degrees,
        y: vector.y,
        z: vector.x * sin_degrees + vector.z * cos_degrees,
    }
}

fn z_axis_rotate(vector: &Vec3, degrees: f32) -> Vec3 {
    let cos_degrees = degrees.cos();
    let sin_degrees = degrees.sin();
    Vec3 {
        x: vector.x * cos_degrees - vector.y * sin_degrees,
        y: vector.x * sin_degrees + vector.y * cos_degrees,
        z: vector.z,
    }
}

pub fn rotate_vec3(vector: &Vec3, x_degrees: f32, y_degrees: f32, z_degrees: f32) -> Vec3 {
    let result = vector.clone();
    let result = x_axis_rotate(&result, x_degrees);
    let result = y_axis_rotate(&result, y_degrees);
    let result = z_axis_rotate(&result, z_degrees);

    result
}
