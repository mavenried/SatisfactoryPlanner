use macroquad::prelude::*;

mod camera;
mod helpers;
mod render;
mod types;
mod input;

#[macroquad::main("Satisfactory Planner")]

async fn main() {
    let mut state = types::State::default();
    loop {
        clear_background(BLACK);
        state.camera.apply();
        input::update(&mut state);
        render::grid::draw();
        render::placeables::draw(&state);

        next_frame().await
    }
}
