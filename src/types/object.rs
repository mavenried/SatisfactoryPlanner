use super::BuildingDef;
use macroquad::prelude::*;

#[derive(Clone)]
pub enum ObjRotation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
pub enum ObjectKind {
    Building {
        def: BuildingDef,
        position: Vec2,
        rotation: ObjRotation,
    },
    Belt {
        pos: Vec<Vec2>,
    },
    Pipe {
        pos: Vec<Vec2>,
    },
}

#[derive(Clone)]
pub struct Object {
    pub id: usize,
    pub kind: ObjectKind,
}
