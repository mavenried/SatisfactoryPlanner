use crate::building::BuildingDef;
use crate::camera::Camera;
use crate::object::Object;

#[derive(Clone)]
pub enum Placeable {
    Building(usize),
    Belt,
    Pipe,
}

pub struct State {
    pub objects: Vec<Object>,
    pub selected: Option<usize>,
    pub placing: Option<Placeable>,
    pub show_menu: bool,
    pub camera: Camera,
    pub building_defs: Vec<BuildingDef>,
    pub menu_scroll: f32,
}

impl State {
    pub fn new() -> Self {
        let data = std::fs::read_to_string("assets/db.json").unwrap();
        let defs: Vec<BuildingDef> = serde_json::from_str(&data).unwrap();

        Self {
            objects: vec![],
            selected: None,
            placing: None,
            show_menu: false,
            camera: Camera::new(),
            building_defs: defs,
            menu_scroll: 0.0,
        }
    }
}
