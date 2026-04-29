use super::vec2_serde;
use macroquad::prelude::*;
use serde::Deserialize;
#[derive(Clone, Deserialize)]
pub enum PortKind {
    BeltInput,
    BeltOutput,
    PipeInput,
    PipeOutput,
}

#[derive(Clone, Deserialize)]
pub struct Port {
    #[serde(with = "vec2_serde")]
    pub pos: Vec2,
    pub kind: PortKind,
}
#[derive(Clone, serde::Deserialize)]
pub struct BuildingDef {
    pub name: String,
    pub display: String,
    pub width: f32,
    pub length: f32,
    pub i_ports: Vec<Port>,
    pub o_ports: Vec<Port>,
}
