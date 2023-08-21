struct Matrix4 {
    data: [[f32; 4]; 4],
}

impl Matrix4 {
    fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn scale(scalar: f32) -> Self {
        Self {
            data: [
                [scalar, 0.0, 0.0, 0.0],
                [0.0, scalar, 0.0, 0.0],
                [0.0, 0.0, scalar, 0.0],
                [0.0, 0.0, 0.0, scalar],
            ],
        }
    }
}
