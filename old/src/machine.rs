use macroquad::prelude::*;

pub struct Machine {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Machine {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            size: vec2(80.0, 80.0),
        }
    }
}
