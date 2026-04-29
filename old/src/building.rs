use macroquad::prelude::*;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct BuildingDef {
    pub name: String,
    pub width: f32,
    pub length: f32,
}

pub struct BuildingInstance {
    pub pos: Vec2,
    pub size: Vec2,
    pub name: String,
}

impl BuildingInstance {
    pub fn new(def: &BuildingDef, pos: Vec2) -> Self {
        Self {
            pos,
            size: vec2(def.width, def.length),
            name: def.name.clone(),
        }
    }
}
