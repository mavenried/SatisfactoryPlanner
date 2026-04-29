use crate::helpers::GRIDSIZE;
use macroquad::prelude::*;
pub fn draw() {
    for x in -2000..2000 {
        let xf = x as f32 * GRIDSIZE;
        let color = DARKGRAY;
        draw_line(xf, -40000.0, xf, 40000.0, 2.0, color);
    }
    for y in -2000..2000 {
        let yf = y as f32 * GRIDSIZE;
        let color = DARKGRAY;
        draw_line(-40000.0, yf, 40000.0, yf, 2.0, color);
    }
    for x in -250..251 {
        let xf = x as f32 * 8f32 * GRIDSIZE;
        let color = GRAY;
        draw_line(xf, -40000.0, xf, 40000.0, 4.0, color);
    }
    for y in -250..251 {
        let yf = y as f32 * 8f32 * GRIDSIZE;
        let color = GRAY;
        draw_line(-40000.0, yf, 40000.0, yf, 4.0, color);
    }
}
