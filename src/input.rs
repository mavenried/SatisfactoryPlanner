use crate::helpers::{GRIDSIZE, snap};
use crate::types::{ObjRotation, Object, Placeable, State};
use macroquad::prelude::*;

fn mouse_scroll(state: &mut State) {
    let (_, scroll) = mouse_wheel();
    if scroll != 0.0 {
        if state.show_menu {
            state.menu_scroll += scroll;
        } else if state.placing.is_some() {
            state.placing_rotation = match state.placing_rotation {
                ObjRotation::Up => ObjRotation::Right,
                ObjRotation::Right => ObjRotation::Down,
                ObjRotation::Down => ObjRotation::Left,
                ObjRotation::Left => ObjRotation::Up,
            }
        } else {
            state.camera.zoom = (state.camera.zoom * 1.0 + scroll * 0.1).clamp(1.0, 2.0);
        }
    }
}
fn mouse_rb(state: &mut State) {
    let mouse_screen: Vec2 = mouse_position().into();
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
}

fn mouse_lb(state: &mut State) {
    let mouse_screen: Vec2 = mouse_position().into();
    let mouse_world = state.camera.screen_to_world(mouse_screen);
    if is_mouse_button_pressed(MouseButton::Left) {
        let pos = snap(mouse_world);

        if state.show_menu {
            return;
        }

        if let Some(placeable) = &state.placing {
            match placeable {
                Placeable::Building(def) => {
                    state.objects.push(Object {
                        id: state.next_id,
                        kind: crate::types::ObjectKind::Building {
                            def: def.clone(),
                            rotation: state.placing_rotation.clone(),
                            position: Vec2 {
                                x: pos.x / GRIDSIZE,
                                y: pos.y / GRIDSIZE,
                            },
                        },
                    });
                    state.next_id += 1;
                }
                Placeable::Belt => {}
                Placeable::Pipe => {}
            }
        }
    }
}

fn keys(state: &mut State) {
    if is_key_pressed(KeyCode::Q) {
        state.show_menu = !state.show_menu;
    }
    if is_key_pressed(KeyCode::C) {
        state.camera.target = Vec2::ZERO;
    }
}

pub fn update(state: &mut State) {
    mouse_scroll(state);
    mouse_rb(state);
    mouse_lb(state);
    keys(state);
}
