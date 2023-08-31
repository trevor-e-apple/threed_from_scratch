pub type Color = u32;

pub fn get_alpha(color: Color) -> u8 {
    ((0xFF000000 & color) >> 24) as u8
}

pub fn get_red(color: Color) -> u8 {
    ((0x00FF0000 & color) >> 16) as u8
}

pub fn get_green(color: Color) -> u8 {
    ((0x0000FF00 & color) >> 8) as u8
}

pub fn get_blue(color: Color) -> u8 {
    (0x000000FF & color) as u8
}

pub fn make_color(alpha: u8, red: u8, green: u8, blue: u8) -> Color {
    ((alpha as u32) << 24)
        | ((red as u32) << 16)
        | ((green as u32) << 8)
        | (blue as u32)
}
