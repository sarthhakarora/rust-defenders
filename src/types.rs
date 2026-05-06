use raylib::prelude::*;

pub struct Circle {
    pub radius: f32,
    pub center: Vector2,
}

impl Circle {
    pub fn new(radius: f32, center: Vector2) -> Self {
        Self { radius, center }
    }
}

pub fn point_in_circle(x: f32, y: f32, cx: f32, cy: f32, r: f32) -> bool {
    let dx = x - cx;
    let dy = y - cy;
    dx * dx + dy * dy <= r * r
}
