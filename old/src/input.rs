use crate::belt::Belt;
use crate::building::BuildingInstance;
use crate::object::{Object, ObjectKind};
use crate::state::{Placeable, State};
use macroquad::prelude::*;

const GRID: f32 = 20.0;

fn snap(v: Vec2) -> Vec2 {
    vec2((v.x / GRID).round() * GRID, (v.y / GRID).round() * GRID)
}

pub fn update(state: &mut State) {
    let mouse_screen: Vec2 = mouse_position().into();
    let mouse_world = state.camera.screen_to_world(mouse_screen);

    let (_, scroll) = mouse_wheel();
    if scroll != 0.0 {
        state.camera.zoom = (state.camera.zoom * 1.0 + scroll * 0.1).clamp(1.0, 2.0);
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        state.camera.dragging = true;
        state.camera.last_mouse = mouse_screen;
    }

    if is_mouse_button_down(MouseButton::Right) && state.camera.dragging {
        let delta = mouse_screen - state.camera.last_mouse;
        state.camera.target -= delta * 2f32 / state.camera.zoom;

        state.camera.last_mouse = mouse_screen;
    }

    if is_mouse_button_released(MouseButton::Right) {
        state.camera.dragging = false;
    }

    if is_key_pressed(KeyCode::Q) {
        state.show_menu = !state.show_menu;
    }
    if is_key_pressed(KeyCode::C) {
        state.camera.target = Vec2::ZERO;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        let pos = snap(mouse_world);

        if state.show_menu {
            return;
        }

        if let Some(placeable) = &state.placing {
            match placeable {
                Placeable::Building(i) => {
                    let def = &state.building_defs[*i];
                    state.objects.push(Object {
                        kind: ObjectKind::Building(BuildingInstance::new(def, pos)),
                    });
                }
                Placeable::Belt => {
                    if let Some(last) = state.objects.last_mut() {
                        if let ObjectKind::Belt(b) = &mut last.kind {
                            b.add_point(pos);
                            return;
                        }
                    }

                    state.objects.push(Object {
                        kind: ObjectKind::Belt(Belt::new(pos)),
                    });
                }
                Placeable::Pipe => {}
            }
        }
    }
}
