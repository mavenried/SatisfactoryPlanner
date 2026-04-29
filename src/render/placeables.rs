use crate::{
    helpers::GRIDSIZE,
    types::{ObjRotation, Object, ObjectKind, Placeable, State},
};
use macroquad::prelude::*;

pub fn draw(state: &State) {
    for obj in &state.objects {
        match &obj.kind {
            ObjectKind::Building {
                def,
                position,
                rotation,
            } => {
                let (w, l) = match rotation {
                    ObjRotation::Up | ObjRotation::Down => (def.width, def.length),
                    ObjRotation::Left | ObjRotation::Right => (def.length, def.width),
                };

                let x = position.x - w / 2.0;
                let y = position.y - l / 2.0;

                draw_rectangle(
                    x * GRIDSIZE,
                    y * GRIDSIZE,
                    w * GRIDSIZE,
                    l * GRIDSIZE,
                    LIGHTGRAY,
                );
            }
            _ => (),
        }
    }
}
