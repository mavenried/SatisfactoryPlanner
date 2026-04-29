use macroquad::prelude::*;

#[derive(Clone)]
pub struct Belt {
    pub points: Vec<Vec2>,
}

impl Belt {
    pub fn new(start: Vec2) -> Self {
        Self { points: vec![start] }
    }

    pub fn add_point(&mut self, p: Vec2) {
        self.points.push(p);
    }
}
