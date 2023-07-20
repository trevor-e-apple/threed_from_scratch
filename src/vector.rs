pub struct Vec2 {
	x: f32,
	y: f32
}

pub struct Vec3 {
	x: f32,
	y: f32,
	z: f32
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Vec3 {x, y, z}
	}
}