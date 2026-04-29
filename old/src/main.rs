use macroquad::prelude::*;

mod belt;
mod building;
mod camera;
mod input;
mod object;
mod render;
mod state;

use state::State;

#[macroquad::main("Planner")]
async fn main() {
    let mut state = State::new();

    loop {
        clear_background(Color::from_rgba(30, 30, 30, 255));

        // ===== INPUT FIRST =====
        input::update(&mut state);

        // ===== WORLD PASS =====
        state.camera.apply();
        render::draw_world(&state);

        // ===== UI PASS =====
        set_default_camera();
        render::draw_ui(&mut state);

        next_frame().await;
    }
}
