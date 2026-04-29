use crate::camera::Camera;
use macroquad::prelude::*;
mod building;
 mod vec2_serde;
pub use building::*;
mod object;
pub use object::*;

#[derive(Clone)]
pub enum Placeable {
    Building(BuildingDef),
    Belt,
    Pipe,
}


pub struct State {
    pub objects: Vec<Object>,
    pub selected: Option<Vec<Object>>,
    pub placing: Option<Placeable>,
    pub placing_rotation: ObjRotation,
    pub show_menu: bool,
    pub camera: Camera,
    pub building_defs: Vec<BuildingDef>,
    pub menu_scroll: f32,
    pub next_id: usize,
}
impl State {
    pub fn get_new_id(&mut self) -> usize {
        let out = self.next_id;
        self.next_id += 1;
        out
    }
}
impl Default for State {
    fn default() -> Self {
        let data = include_str!("../../assets/db.json");
        let defs: Vec<BuildingDef> = serde_json::from_str(&data).unwrap();
        Self {
            objects: vec![],
            selected: None,
            placing: Some(Placeable::Building(defs[0].clone())),
            show_menu: false,
            camera: Camera::new(),
            building_defs: defs,
            menu_scroll: 0.0,
            next_id: 0,
            placing_rotation: ObjRotation::Up,
        }
    }
}
