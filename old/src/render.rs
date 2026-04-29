use crate::object::ObjectKind;
use crate::state::{Placeable, State};
use macroquad::prelude::*;

const GRID: f32 = 20.0;
pub fn draw_ui(state: &mut State) {
    if state.show_menu {
        draw_menu(state);
    }
}
pub fn draw_world(state: &State) {
    draw_grid();

    for obj in &state.objects {
        match &obj.kind {
            ObjectKind::Building(b) => {
                draw_rectangle(
                    b.pos.x - b.size.x * GRID / 2.0,
                    b.pos.y - b.size.y * GRID / 2.0,
                    b.size.x * GRID,
                    b.size.y * GRID,
                    LIGHTGRAY,
                );
                draw_text(&b.name.to_uppercase(), b.pos.x, b.pos.y, 40.0, BLACK);
            }
            ObjectKind::Belt(b) => {
                for i in 0..b.points.len().saturating_sub(1) {
                    let a = b.points[i];
                    let c = b.points[i + 1];
                    draw_line(a.x, a.y, c.x, c.y, 40.0, LIGHTGRAY);
                }
            }
        }
    }
}

fn draw_grid() {
    for x in -2000..2000 {
        let xf = x as f32 * GRID;
        let color = DARKGRAY;
        draw_line(xf, -40000.0, xf, 40000.0, 2.0, color);
    }
    for y in -2000..2000 {
        let yf = y as f32 * GRID;
        let color = DARKGRAY;
        draw_line(-40000.0, yf, 40000.0, yf, 2.0, color);
    }
    for x in -250..251 {
        let xf = x as f32 * 8f32 * GRID;
        let color = GRAY;
        draw_line(xf, -40000.0, xf, 40000.0, 4.0, color);
    }
    for y in -250..251 {
        let yf = y as f32 * 8f32 * GRID;
        let color = GRAY;
        draw_line(-40000.0, yf, 40000.0, yf, 4.0, color);
    }
}

enum MenuItem {
    Belt,
    Pipe,
    Building(usize),
}

fn draw_menu(state: &mut State) {
    let screen_w = screen_width();
    let margin = 20.0;
    let cell_size = 80.0;
    let padding = 10.0;

    let cols = ((screen_w - margin * 2.0) / (cell_size + padding))
        .floor()
        .max(1.0) as usize;

    let start_x = margin;
    let start_y = margin;

    let (_, scroll) = mouse_wheel();
    state.menu_scroll -= scroll * 30.0;
    if state.menu_scroll < 0.0 {
        state.menu_scroll = 0.0;
    }

    let mut index = 0;

    draw_item(
        state,
        "Belt",
        index,
        cols,
        cell_size,
        padding,
        start_x,
        start_y,
        MenuItem::Belt,
    );
    index += 1;

    draw_item(
        state,
        "Pipe",
        index,
        cols,
        cell_size,
        padding,
        start_x,
        start_y,
        MenuItem::Pipe,
    );
    index += 1;

    for (i, b) in state.building_defs.clone().iter().enumerate() {
        draw_item(
            state,
            &b.name,
            index,
            cols,
            cell_size,
            padding,
            start_x,
            start_y,
            MenuItem::Building(i),
        );
        index += 1;
    }
}

fn draw_item(
    state: &mut State,
    label: &str,
    index: usize,
    cols: usize,
    size: f32,
    pad: f32,
    sx: f32,
    sy: f32,
    item: MenuItem,
) {
    let col = index % cols;
    let row = index / cols;

    let x = sx + col as f32 * (size + pad);
    let y = sy + row as f32 * (size + pad) - state.menu_scroll;

    if y < -size || y > screen_height() {
        return;
    }

    let mouse: Vec2 = mouse_position().into();
    let hovered = mouse.x > x && mouse.x < x + size && mouse.y > y && mouse.y < y + size;

    draw_rectangle(x, y, size, size, if hovered { LIGHTGRAY } else { DARKGRAY });
    draw_text(label, x + 5.0, y + 20.0, 16.0, WHITE);

    if hovered && is_mouse_button_pressed(MouseButton::Left) {
        state.placing = Some(match item {
            MenuItem::Belt => Placeable::Belt,
            MenuItem::Pipe => Placeable::Pipe,
            MenuItem::Building(i) => Placeable::Building(i),
        });
    }
}
