use macroquad::prelude::*;

pub const GRIDSIZE: f32 = 20.0;

pub fn snap(v: Vec2) -> Vec2 {
    vec2(
        (v.x / GRIDSIZE).round() * GRIDSIZE,
        (v.y / GRIDSIZE).round() * GRIDSIZE,
    )
}
