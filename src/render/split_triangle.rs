use crate::{
    triangle::{get_split_triangle_point, Triangle},
    vector2::Vec2i,
};

pub struct SplitTriangle {
    top: Vec2i,
    middle: Vec2i,
    bottom: Vec2i,
    ray_intersection: Vec2i,
    top_triangle_top_y: i32,
    top_triangle_bottom_y: i32,
    top_triangle_x_per_y_1: f32,
    top_triangle_x_per_y_2: f32,
    bottom_triangle_top_y: i32,
    bottom_triangle_bottom_y: i32,
    bottom_triangle_x_per_y_1: f32,
    bottom_triangle_x_per_y_2: f32,
}

impl SplitTriangle {
    pub fn from_triangle(triangle: &Triangle) -> Self {
        let (sorted_points, ray_intersection) =
            get_split_triangle_point(triangle);

        let top = Vec2i::from_vec2_floor(&sorted_points[0]);
        let middle = Vec2i::from_vec2_floor(&sorted_points[1]);
        let bottom = Vec2i::from_vec2_floor(&sorted_points[2]);
        let ray_intersection = Vec2i::from_vec2_floor(&ray_intersection);

        // the top filled triangle (flat bottom)
        let top_triangle_data = {
            let top_y = top.y;
            let bottom_y = ray_intersection.y;
            // find the change in x for each y pixel (top to bottom)
            let x_per_y_1 =
                (middle.x - top.x) as f32 / (middle.y - top.y) as f32;
            let x_per_y_2 = (ray_intersection.x - top.x) as f32
                / (ray_intersection.y - top.y) as f32;

            (top_y, bottom_y, x_per_y_1, x_per_y_2)
        };

        // the bottom filled triangle (flat top)
        let bottom_triangle_data = {
            let top_y = ray_intersection.y as i32;
            let bottom_y = bottom.y as i32;
            let x_per_y_1 =
                (bottom.x - middle.x) as f32 / (bottom.y - middle.y) as f32;
            let x_per_y_2 = (bottom.x - ray_intersection.x) as f32
                / (bottom.y - ray_intersection.y) as f32;

            (top_y, bottom_y, x_per_y_1, x_per_y_2)
        };

        Self {
            top,
            middle,
            bottom,
            ray_intersection,
            top_triangle_top_y: top_triangle_data.0,
            top_triangle_bottom_y: top_triangle_data.1,
            top_triangle_x_per_y_1: top_triangle_data.2,
            top_triangle_x_per_y_2: top_triangle_data.3,
            bottom_triangle_top_y: bottom_triangle_data.0,
            bottom_triangle_bottom_y: bottom_triangle_data.1,
            bottom_triangle_x_per_y_1: bottom_triangle_data.2,
            bottom_triangle_x_per_y_2: bottom_triangle_data.3,
        }
    }

    pub fn should_fill_top(&self) -> bool {
        self.top_triangle_top_y != self.top_triangle_bottom_y
    }

    pub fn should_fill_bottom(&self) -> bool {
        self.bottom_triangle_top_y != self.bottom_triangle_bottom_y
    }
}

pub struct FillTriangleIter {
    x_start: f32,
    x_end: f32,
    x_per_y_1: f32,
    x_per_y_2: f32,
    upper_bound: i32,
    y: i32,
}

impl FillTriangleIter {
    /// construct the iterator for the top triangle
    pub fn top_iter(split_triangle: &SplitTriangle) -> Self {
        Self {
            x_start: split_triangle.top.x as f32,
            x_end: split_triangle.top.x as f32,
            x_per_y_1: split_triangle.top_triangle_x_per_y_1,
            x_per_y_2: split_triangle.top_triangle_x_per_y_2,
            upper_bound: split_triangle.top_triangle_bottom_y,
            y: split_triangle.top_triangle_top_y,
        }
    }

    /// construct the iterator for the bottom triangle
    pub fn bottom_iter(split_triangle: &SplitTriangle) -> Self {
        Self {
            x_start: split_triangle.middle.x as f32,
            x_end: split_triangle.ray_intersection.x as f32,
            x_per_y_1: split_triangle.bottom_triangle_x_per_y_1,
            x_per_y_2: split_triangle.bottom_triangle_x_per_y_2,
            upper_bound: split_triangle.bottom_triangle_bottom_y,
            y: split_triangle.bottom_triangle_top_y,
        }
    }
}

impl Iterator for FillTriangleIter {
    // x_start, x_end, y
    type Item = (i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.upper_bound {
            let result = (self.x_start as i32, self.x_end as i32, self.y);

            self.x_start += self.x_per_y_1;
            self.x_end += self.x_per_y_2;
            self.y += 1;

            Some(result)
        } else {
            None
        }
    }
}
