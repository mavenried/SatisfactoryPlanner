use crate::belt::Belt;
use crate::building::BuildingInstance;

pub enum ObjectKind {
    Building(BuildingInstance),
    Belt(Belt),
}

pub struct Object {
    pub kind: ObjectKind,
}
