use std::f32::consts::PI;

use crate::{
    color::{get_alpha, get_blue, get_green, get_red, make_color, Color},
    vector3::{self, Vec3},
};

pub struct Light {
    pub direction: Vec3,
}

pub fn compute_shaded_color(
    light: &Light,
    face_normal: Vec3,
    face_color: Color,
) -> Color {
    // negate b/c 
    let intensity = -1.0
        * vector3::dot(
            &vector3::normalize(&light.direction),
            &vector3::normalize(&face_normal),
        );

    let percentage = if intensity < 0.0 {
        0.0
    } else if intensity > 1.0 {
        1.0
    } else {
        intensity
    };

    let red = (get_red(face_color) as f32 * percentage).round() as u8;
    let green = (get_green(face_color) as f32 * percentage).round() as u8;
    let blue = (get_blue(face_color) as f32 * percentage).round() as u8;

    make_color(get_alpha(face_color), red, green, blue)
}
